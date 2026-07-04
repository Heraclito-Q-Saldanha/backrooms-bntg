use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct MainMenu {
    base: Base<Control>,
}

#[godot_api]
impl IControl for MainMenu {
    fn init(base: Base<Control>) -> Self {
        godot_print!("Hello, world!");
        Self { base }
    }
}
