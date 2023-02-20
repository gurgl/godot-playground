use gdnative::api::{AnimatedSprite, KinematicBody2D};
use gdnative::prelude::*;
use rand::seq::SliceRandom;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[user_data(user_data::MutexData<Ball>)]
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
            velocity: Vector2::new(0.0, -100.0)
        }
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta:f32) {
        //godot_print!("_physics_process");     
           
        let collision_info_opt = owner.move_and_collide(self.velocity * delta, true ,true, false);
        if let Some(collision_info) = collision_info_opt {
            self.velocity = self.velocity.bounce(unsafe { collision_info.assume_safe() }.normal())
        }
    } 

    #[method]
    fn _ready(&mut self, #[base] owner: &KinematicBody2D) {
        godot_print!("ball ready");     
        //let mut rng = rand::thread_rng();
    
        /*let animated_sprite = unsafe {
            owner
                .get_node_as::<Sprite>("ball")
                .unwrap()
        };*/
    }

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
}
