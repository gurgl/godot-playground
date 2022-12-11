use gdnative::prelude::*;

mod hud;
mod main_scene;
mod mob;
mod player;

fn init(handle: InitHandle) {
    godot_print!("init");
    handle.add_class::<player::Player>();
    handle.add_class::<mob::Mob>();
    handle.add_class::<main_scene::Main>();
    handle.add_class::<hud::Hud>();
    godot_print!("end");
}

godot_init!(init);
