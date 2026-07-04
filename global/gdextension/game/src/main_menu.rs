use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct MainMenu {
	#[export]
	game_scene: Option<Gd<PackedScene>>,
	base: Base<Control>,
}

#[godot_api]
impl IControl for MainMenu {
	fn init(base: Base<Control>) -> Self {
		let game_scene = None;
		Self { game_scene, base }
	}
}

#[godot_api]
impl MainMenu {
	#[func]
	fn start_game(&self) {
		if let Some(scene) = &self.game_scene {
			self.base().get_tree().change_scene_to_packed(&*scene);
		}
	}
	#[func]
	fn exit_game(&self) {
		self.base().get_tree().quit();
	}
}
