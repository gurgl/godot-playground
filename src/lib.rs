use gdnative::prelude::*;

mod hud;
mod main_scene;
mod mob;
mod player_pad;
mod ball;

fn init(handle: InitHandle) {
    godot_print!("init");
    handle.add_class::<player_pad::PlayerPad>();
    handle.add_class::<mob::Mob>();
    handle.add_class::<ball::Ball>();
    handle.add_class::<main_scene::Main>();
    handle.add_class::<hud::Hud>();
    godot_print!("end");
}

godot_init!(init);
