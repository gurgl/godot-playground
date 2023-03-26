use gdnative::api::{AnimatedSprite, StaticBody2D, PhysicsBody2D};
use gdnative::prelude::*;
use rand::seq::SliceRandom;

#[derive(NativeClass)]
#[inherit(StaticBody2D)]
//#[user_data(user_data::MutexData<Brick>)]
pub struct Brick {
    
}

#[derive(Copy, Clone)]
enum BrickType {
    Walk,
    Swim,
    Fly,
}

impl BrickType {
    fn to_str(self) -> String {
        match self {
            BrickType::Walk => "walk".to_string(),
            BrickType::Swim => "swim".to_string(),
            BrickType::Fly => "fly".to_string(),
        }
    }
}

const MOB_TYPES: [BrickType; 3] = [BrickType::Walk, BrickType::Swim, BrickType::Fly];

#[methods]
impl Brick {
    fn new(_owner: &StaticBody2D) -> Self {
        Brick {
            
        }
    }

    #[method]
    fn _ready(&mut self, #[base] owner: &StaticBody2D) {
        
    }



    #[method]
    fn on_start_game(&self, #[base] owner: &StaticBody2D) {
        godot_print!("start");
        unsafe {
            owner.assume_unique().queue_free();
        }
    }

    #[method]
    pub fn hit(&self, #[base] owner: &StaticBody2D) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }
}
