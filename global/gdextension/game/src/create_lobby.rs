use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control, init)]
pub struct CreateLobby {
	base: Base<Control>,
}

#[godot_api]
impl IControl for CreateLobby {
	fn ready(&mut self) {}
}
