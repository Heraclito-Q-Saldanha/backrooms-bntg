use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control, init)]
struct MainMenu {
	#[export]
	create_lobby_scene: Option<Gd<PackedScene>>,
	#[export]
	search_lobby_scene: Option<Gd<PackedScene>>,
	base: Base<Control>,
}

#[godot_api]
impl MainMenu {
	#[func]
	fn create_lobby(&self) {
		if let Some(scene) = &self.create_lobby_scene {
			self.base().get_tree().change_scene_to_packed(&*scene);
		}
	}
	#[func]
	fn search_lobby(&self) {
		if let Some(scene) = &self.search_lobby_scene {
			self.base().get_tree().change_scene_to_packed(&*scene);
		}
	}
	#[func]
	fn exit_game(&self) {
		self.base().get_tree().quit();
	}
}
