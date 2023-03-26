use gdnative::api::{AnimatedSprite, KinematicBody2D, CollisionShape2D, PhysicsBody2D};
use gdnative::prelude::*;
use gdnative::prelude::{Ref};

/// The player "class"
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
//#[user_data(user_data::MutexData<PlayerPad>)]
#[register_with(Self::register_player)]
pub struct PlayerPad {
    #[property(default = 400.0)]
    speed: f32,

    screen_size: Vector2,
}

#[methods]
impl PlayerPad {
    fn register_player(builder: &ClassBuilder<Self>) {
        builder.signal("hit").done()
    }

    fn new(_owner: &KinematicBody2D) -> Self {
        PlayerPad {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
        }
    }

    #[method]
    fn _ready(&mut self, #[base] owner: &KinematicBody2D) {
        let viewport = owner.get_viewport_rect();
        self.screen_size = viewport.size;
        owner.hide();
    }

    #[method]
    fn _process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        //godot_print!("pad process");
        let input = Input::godot_singleton();
        let mut velocity = Vector2::new(0.0, 0.0);

        // Note: exact=false by default, in Rust we have to provide it explicitly
        if Input::is_action_pressed(input, "ui_right", false) {
            //godot_print!("right");
            velocity.x += 1.0
        }
        if Input::is_action_pressed(input, "ui_left", false) {
            //godot_print!("left");
            velocity.x -= 1.0
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;
        }

        let change = velocity * delta;
        let position = owner.global_position() + change;
        let position = Vector2::new(
            position.x.max(0.0).min(self.screen_size.x),
            position.y.max(0.0).min(self.screen_size.y),
        );
        owner.set_global_position(position);
    }


    #[method]
    pub fn move_pad(&self, #[base] owner: &KinematicBody2D, xpos:f32) {
        let old_pos = owner.global_position();
        owner.set_position(Vector2::new(xpos, old_pos.y));
    }

    #[method]
    fn on_pad_body_entered(&self, #[base] owner: &KinematicBody2D, _body: Ref<PhysicsBody2D>) {
        owner.hide();
        owner.emit_signal("hit", &[]);

        let collision_shape = unsafe {
            owner
                .get_node_as::<CollisionShape2D>("collision_shape_2d")
                .unwrap()
        };

        collision_shape.set_deferred("disabled", true);
    }

    #[method]
    fn on_body_entered(&self, #[base] owner: &KinematicBody2D, _body: Ref<PhysicsBody2D>) {
        godot_print!("pad on body entered");
    }

    #[method]
    pub fn start(&self, #[base] owner: &KinematicBody2D, pos: Vector2) {
        owner.set_global_position(pos);
        owner.show();
        godot_print!("start pad");
        let collision_shape = unsafe {
            owner
                .get_node_as::<CollisionShape2D>("collision_shape_2d")
                .unwrap()
        };

        collision_shape.set_disabled(false);
    }


}
