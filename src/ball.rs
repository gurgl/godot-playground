use std::fmt::Debug;

use gdnative::api::{AnimatedSprite, KinematicBody2D, PhysicsBody2D, RigidBody2D, StaticBody2D, CollisionShape2D};
use gdnative::prelude::*;
use rand::seq::SliceRandom;
use crate::brick;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[user_data(user_data::MutexData<Ball>)]
#[register_with(Self::register_ball)]
pub struct Ball {
    #[property(default = 150.0)]
    pub speed: f32,
    pub velocity: Vector2,

}


#[methods]
impl Ball {
    fn new(_owner: &KinematicBody2D) -> Self {
        Ball {
            speed: 1.0,
            velocity: Vector2::new(0.0, -200.0)
        }
    }


    fn register_ball(builder: &ClassBuilder<Self>) {
        godot_print!("register ball");
        builder.signal("game_over").done();
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta:f32) {
        //godot_print!("_physics_process");     
           
        let collision_info_opt = owner.move_and_collide(self.velocity * delta, true ,true, false);
        if let Some(collision_info_ref) = collision_info_opt {
            //godot_print!("collision_info collision");     
            let collision_info = unsafe { collision_info_ref.assume_safe() };
            if let Some(collider) = collision_info.collider() {
                //godot_print!("collider collision {:?}",res.get_class());     
                if let Some(static_body) = unsafe { collider.assume_safe() }.cast::<StaticBody2D>() {
                    //godot_print!("static body collision");     
                    if static_body.is_in_group("BrickGroup") {
                        godot_print!("brick collision");     
                        if let Some(brick) = unsafe { static_body.assume_shared().assume_unique() }.cast_instance::<brick::Brick>() {
                            godot_print!("brick brick collision");     
                            brick.map(|x,_| x.hit(static_body.as_ref()));
                            ()
                        }
                    } else if static_body.name().eq(&GodotString::from_str("BottomEdge")) {
                        godot_print!("Game over");
                        let root = unsafe { owner.get_parent().unwrap().assume_safe() };
                        owner.emit_signal(GodotString::from_str("game_over"), &[]);
                        //SignalBuilder::new("game_over")
                        return ()
                    }
                }
            }
            self.velocity = self.velocity.bounce(collision_info.normal());
        }
    } 

    #[method]
    fn _ready(&mut self, #[base] owner: &KinematicBody2D) {
        godot_print!("ball ready");     
    }



    #[method]
    fn on_body_entered(&self, #[base] owner: &KinematicBody2D, _body: Ref<PhysicsBody2D>) {
        godot_print!("brick collide");
        let body = unsafe {_body.assume_safe() };
        if body.is_in_group("BrickGroup") {
            godot_print!("brick col");     
        }

    }   


    /*
    
    func _on_Ball_area_entered(area):
	bounce(area)
	if area.is_in_group("Brick"):
		var break_p = break_prefab.instance().duplicate()
		break_p.color = area.get_child(1).modulate
		break_p.position = area.position
		get_parent().add_child(break_p)
		break_p.emitting = true
		area.queue_free()
	if area.is_in_group("Bottom"):
		get_tree().quit()

    
    */

    #[method]
    fn on_visibility_screen_exited(&self, #[base] owner: &KinematicBody2D) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }

    #[method]
    fn on_start_game(&self, #[base] owner: &KinematicBody2D) {
        godot_print!("ball start");
        unsafe {
            owner.assume_unique().queue_free();
        }
    }

    #[method]
    pub fn tear_down(&self, #[base] owner: &KinematicBody2D) {
        godot_print!("ball game over");
        unsafe {
            owner.assume_unique().queue_free();
        }
    }


}
