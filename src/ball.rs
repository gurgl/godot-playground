use std::fmt::Debug;


use gdnative::api::{AnimatedSprite, KinematicBody2D, PhysicsBody2D, RigidBody2D, StaticBody2D, CollisionShape2D, RectangleShape2D};
use gdnative::prelude::*;
use rand::seq::SliceRandom;
use crate::{brick, player_pad};
use crate::util::*;
use crate::util::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
//#[user_data(user_data::MutexData<Ball>)]
//#[user_data(user_data::LocalCellData<Ball>)]
#[register_with(Self::register_ball)]
pub struct Ball {
    #[property(default = 150.0)]
    pub speed: f32,
    pub velocity: Vector2,

}

const D45:f32 = (std::f32::consts::PI)/4.0;

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


    fn try_brick_collision(static_body:TRef<StaticBody2D>, ball: &KinematicBody2D) {
        if static_body.is_in_group("BrickGroup") {
            godot_print!("brick collision");     
            if let Some(brick) = unsafe { static_body.assume_shared().assume_unique() }.cast_instance::<brick::Brick>() {
                godot_print!("brick brick collision");     
                brick.map(|x,_| x.hit(static_body.as_ref()));
                ()
            }
        } else if static_body.name().eq(&GodotString::from_str("BottomEdge")) {
            godot_print!("ball coll game over");
            ball.set_process(false);
            ball.set_sync_to_physics(false);
            ball.emit_signal(GodotString::from_str("game_over"), &[]);
            //unsafe { owner.assume_unique().queue_free() };
            //self.tear_down(&owner);
            return ()
        }
    }

    fn try_pad_collision(&mut self, kinematic_body:TRef<KinematicBody2D>, collision_info: TRef<gdnative::api::KinematicCollision2D>) -> Option<Vector2> {
        if let Some(player_pad) = unsafe { kinematic_body.assume_shared().assume_unique() }.cast_instance::<player_pad::PlayerPad>() {
                        
            let coll_pos = collision_info.position();
            let pad_pos = kinematic_body.position();
            let diff = coll_pos.x - pad_pos.x;
            let is_pad = kinematic_body.name() == GodotString::from_str("pad");    //kinematic_body.local_shape;
            let newVel = if is_pad {
                godot_print!("cast 0");
                let collision_shape = unsafe {
                    kinematic_body
                        .get_node_as::<CollisionShape2D>("collision_shape_2d")
                        .unwrap()
                };                    
                if let Some(obj) = collision_shape.shape() {
                    //let res : CollisionShape2D = unsafe { obj.assume_safe().assume_unique()  };
                    let rect = unsafe { obj.assume_safe().assume_unique() }.cast::<RectangleShape2D>().unwrap();
                    let padExtents = rect.extents();
                    let width = padExtents.x;
                    
                    //let boo = (diff / width).tan()
                    let foo = Vector2::new(-diff,width).normalized();
                    godot_print!("cast 1 {} {}", foo.x, foo.y);
                    let bounced = self.velocity.bounce(foo);
                                            
                    Some(Ball::clampAngle(bounced))
                } else {
                    None
                }
            } else {
                None
            };

            //let msg = format!("brick pad collision {}", diff);
            godot_print!("brick pad collision {}", diff);     
            //unsafe { collision_info.assume_safe() };            
            //brick.map(|x,_| x.hit(static_body.as_ref()));
            newVel            
        } else {
            None
        }                

    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta:f32) {
           
        let collision_info_opt = owner.move_and_collide(self.velocity * delta, true ,true, false);
        if let Some(collision_info_ref) = collision_info_opt {
            let collision_info: TRef<gdnative::api::KinematicCollision2D> = unsafe { collision_info_ref.assume_safe() };
            if let Some(collider) = collision_info.collider() {
                if let Some(static_body) = unsafe { collider.assume_safe() }.cast::<StaticBody2D>() {
                    Ball::try_brick_collision(static_body, owner);
                    self.velocity = self.velocity.bounce(collision_info.normal());
                } else if let Some(kinematic_body) = unsafe { collider.assume_safe() }.cast::<KinematicBody2D>() {
                    let velOpt = self.try_pad_collision(kinematic_body, collision_info);
                    if let Some(newVel) = velOpt {
                        self.velocity = newVel;
                    } else {
                        self.velocity = self.velocity.bounce(collision_info.normal());
                    }
                }  
            }            
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


    //#[method]
    //pub fn tear_down(&self, #[base] owner: &KinematicBody2D) {
    pub fn tear_down(&self, owner: TRef<KinematicBody2D>) {
        godot_print!("ball game over");
        unsafe {
            //owner.assume_unique().hide();
            owner.assume_unique().queue_free();
        }
        //owner.queue_free();
    }

    pub fn angleCorrection(bounced:Vector2) -> f32 {
        if let Some(angle) = bounced.as_angle() {
            if angle < (D45) { 
                D45 - angle  
            } else if angle > (D45 * 3.0) { 
                -(3.0 * D45 - angle)
            } else { 
                0.0 
            }
        } else {
            panic!("crash and burn")
        }
    }

    pub fn clampAngle(v:Vector2) -> Vector2 {
        let correction = Ball::angleCorrection(v);
        v.rotated(correction)
    }

}
