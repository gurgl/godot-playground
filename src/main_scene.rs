use crate::hud;
use crate::mob;
use crate::ball;
use crate::brick;
use crate::player_pad;
use gdnative::api::InputEventMouseMotion;
use gdnative::api::ResourcePreloader;
use gdnative::api::{PathFollow2D, Position2D, StaticBody2D, RigidBody2D, AudioStreamPlayer, InputEventKey};
use gdnative::prelude::*;
use rand::*;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[user_data(user_data::LocalCellData<Main>)]
#[register_with(Self::register_main)]
pub struct Main {
    #[property]
    ball: Ref<PackedScene>,
    bricks : Vec<Ref<StaticBody2D>>,
    mousePos : Option<Vector2>,
    score: i64,
}

#[methods]
impl Main {
    fn new(_owner: &Node2D) -> Self {
        godot_print!("Main::new");
        Main {
            ball: PackedScene::new().into_shared(),
            bricks: Vec::new(),
            mousePos: None, 
            score: 0,
        }
    }

    fn register_main(builder: &ClassBuilder<Self>) {
        godot_print!("register main");
        //builder.signal("game_over").done();
    }

    #[method]
    fn on_game_over(&self, #[base] owner: &Node2D) {
        godot_print!("Game over received");
        self.game_over(owner)
    }
    #[method]
    fn _on_ball_game_over_ball(&self, #[base] owner: &Node2D) {
        godot_print!("Game over received");
        self.game_over(owner)
    }

    #[method]
    fn game_over(&self, #[base] owner: &Node2D) {

        let score_timer = unsafe { owner.get_node_as::<Timer>("score_timer").unwrap() };
        //let mob_timer = unsafe { owner.get_node_as::<Timer>("mob_timer").unwrap() };

        let music = unsafe { owner.get_node_as::<AudioStreamPlayer>("Music").unwrap() };
        music.stop();

        score_timer.stop();
        //mob_timer.stop();

        let hud = unsafe { owner.get_node_as_instance::<hud::Hud>("hud").unwrap() };
        hud.map(|x, o| x.show_game_over(&o))
            .ok()
            .unwrap_or_else(|| godot_print!("Unable to get hud"));
    }

    #[method]
    fn new_game(&mut self, #[base] owner: &Node2D) {
        godot_print!("new_game");
        let music = unsafe { owner.get_node_as::<AudioStreamPlayer>("Music").unwrap() };
        music.play(0.0);
        
        
        let start_position = unsafe { owner.get_node_as::<Position2D>("start_position").unwrap() };
        let player = unsafe {
            owner
                .get_node_as_instance::<player_pad::PlayerPad>("pad")
                .unwrap()
        };
        let start_timer = unsafe { owner.get_node_as::<Timer>("start_timer").unwrap() };

        self.score = 0;

        player
            .map(|x, o| x.start(&o, start_position.position()))
            .ok()
            .unwrap_or_else(|| godot_print!("Unable to get player"));

        start_timer.start(0.0);


        let brick_scene_res = ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://breakout/Brick.tscn"),
            GodotString::from_str("PackedScene"), false);

        if let Some(brick_scene_res) = brick_scene_res.and_then(|s| s.cast::<PackedScene>()) {
            godot_print!("brick Have scene 1");
            //let brick = brick_scene_res;
            let brick_scene: Ref<StaticBody2D, _> = instance_scene(&brick_scene_res);
            //let brick_scene2 =  brick_scene.duplicate();
            let pos_top_left = Vector2::new(50.0, 50.0);
            //brick_scene.set_position(pos);
            let brick_scene_s = brick_scene.into_shared();
            self.bricks.push(brick_scene_s);
            let brick_scene = unsafe { brick_scene_s.assume_safe() };
            let bricks = unsafe { owner.get_node_as::<Node2D>("bricks").unwrap() };
            for n in 1..4 {
                let dup = brick_scene.duplicate(15).unwrap();
                let r = unsafe { dup.assume_safe() }.cast::<StaticBody2D>().unwrap();
                let pos = Vector2::new(pos_top_left.x + 100.0 * (n as f32), 100.0);
                r.set_position(pos);
                bricks.add_child(dup, true);
            }
        
            godot_print!("brick loaded {:?}",pos_top_left);
            
        } else {
            godot_print!("StarWorldLink could not load ship_link scene");
        }

        
        let ball_scene_res = ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://breakout/Ball.tscn"),
            GodotString::from_str("PackedScene"), false);

        if let Some(ball_scene_res) = ball_scene_res.and_then(|s| s.cast::<PackedScene>()) {
            godot_print!("ball Have scene");
            self.ball = ball_scene_res;
            let ball_scene: Ref<KinematicBody2D, _> = instance_scene(&self.ball);
            let pos = Vector2::new(100.0, 100.0);
            //ball_scene.set_linear_velocity(Vector2::new(0.0,-200.0));
            ball_scene.set_position(pos);
                        
            
            //ball.connect("game_over", owner, GodotString::from_str("game_over"),VariantArray::new_shared(),0).unwrap();
            let res = unsafe { owner.assume_shared() };
            //self.map(|_,mo| {
           
            //ball_scene.map(|_,b| 
            //    b.connect("game_over", self, GodotString::from_str("game_over"),VariantArray::new_shared(),0).unwrap() 
            //);
            let ball_scene2 = unsafe { ball_scene.assume_unique() };
            owner.add_child(ball_scene, true);
            
            let ball = ball_scene2.cast_instance::<ball::Ball>().unwrap();
            
            ball.map(|_,o| {
                
                o.connect("game_over", res, GodotString::from_str("game_over"),VariantArray::new_shared(),0).unwrap()
        //    
        //    }).unwrap()
        }).unwrap(); 
            godot_print!("Ball loaded {:?}",pos);
        } else {
            godot_print!("StarWorldLink could not load ship_link scene");
        }

        let hud = unsafe { owner.get_node_as_instance::<hud::Hud>("hud").unwrap() };
        hud.map(|x, o| {
            x.update_score(&o, self.score);
            x.show_message(&o, "Get Ready".into());
        })
        .ok()
        .unwrap_or_else(|| godot_print!("Unable to get hud"));
    }

    #[method]
    fn on_start_timer_timeout(&self, #[base] owner: &Node2D) {
        //let mob_timer = unsafe { owner.get_node_as::<Timer>("mob_timer").unwrap() };
        let score_timer = unsafe { owner.get_node_as::<Timer>("score_timer").unwrap() };
        //mob_timer.start(0.0);
        score_timer.start(0.0);
    }

    #[method]
    fn on_score_timer_timeout(&mut self, #[base] owner: &Node2D) {
        self.score += 1;

        let hud = unsafe { owner.get_node_as_instance::<hud::Hud>("hud").unwrap() };
        hud.map(|x, o| x.update_score(&o, self.score))
            .ok()
            .unwrap_or_else(|| godot_print!("Unable to get hud"));
    }


    #[method]
    fn _input(&mut self, #[base] owner: &Node2D, _event: Ref<InputEvent>) {
        //godot_print!("input");
        if let Some(mouseEvent) = _event.cast::<InputEventMouseMotion>() {
            let pos = unsafe { mouseEvent.assume_safe() }.position();
            //godot_print!("mouse");
            if let Some(prevPos) = self.mousePos {
                //godot_print!("upd");
                let delta = prevPos - pos;
                //let mut foo = self.mousePos;
                //foo = Some(pos);
                
                //godot_print!("delta {} and {}", delta.x, pos.x);
                
            } else {
                
            }

            let player = unsafe {
                owner
                    .get_node_as_instance::<player_pad::PlayerPad>("pad")
                    .unwrap()
            };

            player.map(|x, o| x.move_pad(&o, pos.x))
            .ok()
            .unwrap_or_else(|| godot_print!("Unable to get player"));

            self.mousePos = Some(pos);
            
        }
    }
   
}

/// Root here is needs to be the same type (or a parent type) of the node that you put in the child
///   scene as the root. For instance Spatial is used for this example. 
fn instance_scene<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
where
    Root: gdnative::object::GodotObject<Memory = ManuallyManaged> + SubClass<Node>,
{
    let scene = unsafe { scene.assume_safe() };

    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("should be able to instance scene");

    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .expect("root node type should be correct")
}
