use crate::hud;
use crate::mob;
use crate::ball;
use crate::brick;
use crate::player_pad;
use gdnative::api::ResourcePreloader;
use gdnative::api::{PathFollow2D, Position2D, StaticBody2D, RigidBody2D, AudioStreamPlayer, InputEventKey};
use gdnative::prelude::*;
use rand::*;
use std::f64::consts::PI;

#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::LocalCellData<Main>)]
pub struct Main {
    #[property]
    ball: Ref<PackedScene>,
    bricks : Vec<Ref<StaticBody2D>>,
    score: i64,
}

#[methods]
impl Main {
    fn new(_owner: &Node) -> Self {
        godot_print!("Main::new");
        Main {
            ball: PackedScene::new().into_shared(),
            bricks: Vec::new(),
            score: 0,
        }
    }

    #[method]
    fn game_over(&self, #[base] owner: &Node) {
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
    fn new_game(&mut self, #[base] owner: &Node) {
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
            godot_print!("-3");
            let brick_scene: Ref<StaticBody2D, _> = instance_scene(&brick_scene_res);
            //let brick_scene2 =  brick_scene.duplicate();
            godot_print!("-2");
            let pos = Vector2::new(100.0, 50.0);
            godot_print!("-1");
            brick_scene.set_position(pos);
            godot_print!("0");
            let brick_scene_s = brick_scene.into_shared();
            godot_print!("1");
            self.bricks.push(brick_scene_s);
            godot_print!("2");
            let brick_scene = unsafe { brick_scene_s.assume_safe() };
            godot_print!("3");
            owner.add_child(brick_scene, true);
        
            
            godot_print!("brick loaded {:?}",pos);
            
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
            
            owner.add_child(ball_scene, true);
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
    fn on_start_timer_timeout(&self, #[base] owner: &Node) {
        //let mob_timer = unsafe { owner.get_node_as::<Timer>("mob_timer").unwrap() };
        let score_timer = unsafe { owner.get_node_as::<Timer>("score_timer").unwrap() };
        //mob_timer.start(0.0);
        score_timer.start(0.0);
    }

    #[method]
    fn on_score_timer_timeout(&mut self, #[base] owner: &Node) {
        self.score += 1;

        let hud = unsafe { owner.get_node_as_instance::<hud::Hud>("hud").unwrap() };
        hud.map(|x, o| x.update_score(&o, self.score))
            .ok()
            .unwrap_or_else(|| godot_print!("Unable to get hud"));
    }

    /*
    #[method]
    fn on_mob_timer_timeout(&self, #[base] owner: &Node) {
        let mob_spawn_location = unsafe {
            owner
                .get_node_as::<PathFollow2D>("mob_path/mob_spawn_locations")
                .unwrap()
        };

        let mob_scene: Ref<RigidBody2D, _> = instance_scene(&self.mob);

        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(std::u32::MIN..std::u32::MAX);

        mob_spawn_location.set_offset(offset.into());

        let mut direction = mob_spawn_location.rotation() + PI / 2.0;

        mob_scene.set_position(mob_spawn_location.position());

        direction += rng.gen_range(-PI / 4.0..PI / 4.0);
        mob_scene.set_rotation(direction);
        let d = direction as f32;

        let mob_scene = unsafe { mob_scene.into_shared().assume_safe() };
        owner.add_child(mob_scene, false);

        let mob = mob_scene.cast_instance::<mob::Mob>().unwrap();

        //godot_print!("on_mob_timer_timeout");

        mob.map(|x, mob_owner| {
            mob_owner
                .set_linear_velocity(Vector2::new(rng.gen_range(x.min_speed..x.max_speed), 0.0));

            mob_owner.set_linear_velocity(mob_owner.linear_velocity().rotated(d));

            let hud = unsafe { owner.get_node_as_instance::<hud::Hud>("hud").unwrap() };
            hud.map(|_, o| {
                o.connect(
                    "start_game",
                    mob_owner,
                    "on_start_game",
                    VariantArray::new_shared(),
                    0,
                )
                .unwrap();
            })
            .unwrap();
        })
        .unwrap();
    }
 */

//    #[method]
//    fn _input(&self, #[base] owner: &Node, event: Ref<InputEvent>) {
//        //if let Some(brick_scene_res) = brick_scene_res.and_then(|s| s.cast::<PackedScene>()) {
//        //owner.get_tree().map( paused = true
//        //godot_print!("Input");
//        
//        //if let Some(keyEvent) = unsafe { event.assume_safe() }.cast_instance::<InputEventKey>() {
//        //
//        //}
//        //()
//        if(unsafe { event.assume_safe() }.is_action_pressed("game_pause",false,false)) {
//            godot_print!("Pause");
//            owner.get_tree().map(|x| {
//                let tree = unsafe { x.assume_safe() };
//                let is_paused = tree.is_paused();
//                tree.set_pause(!is_paused);
//                ()
//            });
//
//        }
//            
//    }
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
