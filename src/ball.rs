use gdnative::api::{AnimatedSprite, RigidBody2D, KinematicBody2D};
use gdnative::prelude::*;
use rand::seq::SliceRandom;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
#[user_data(user_data::MutexData<Ball>)]
pub struct Ball {
    #[property(default = 150.0)]
    pub speed: f32,

}


#[methods]
impl Ball {
    fn new(_owner: &RigidBody2D) -> Self {
        Ball {
            speed: 1.0,
        }
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &RigidBody2D, delta:f32) {
        //let collision_info = owner.move_and_collide(delta * )
    } 

    #[method]
    fn _ready(&mut self, #[base] owner: &RigidBody2D) {
        let mut rng = rand::thread_rng();
        let animated_sprite = unsafe {
            owner
                .get_node_as::<Sprite>("ball")
                .unwrap()
        };
    }

    #[method]
    fn on_visibility_screen_exited(&self, #[base] owner: &RigidBody2D) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }

    #[method]
    fn on_start_game(&self, #[base] owner: &RigidBody2D) {
        godot_print!("ball start");
        unsafe {
            owner.assume_unique().queue_free();
        }
    }
}
