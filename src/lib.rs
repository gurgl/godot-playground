use gdnative::prelude::*;

mod hud;
mod main_scene;
mod player_pad;
mod ball;
mod brick;
mod tests;
mod util;

fn init(handle: InitHandle) {
    godot_print!("init");
    handle.add_class::<player_pad::PlayerPad>();
    handle.add_class::<brick::Brick>();
    handle.add_class::<ball::Ball>();
    handle.add_class::<main_scene::Main>();
    handle.add_class::<hud::Hud>();
    godot_print!("end");
}

godot_init!(init);



