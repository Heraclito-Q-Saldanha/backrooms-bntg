use crate::*;

use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control, init)]
pub struct SearchLobby {
	base: Base<Control>,
	item_list: Option<Gd<ItemList>>,
}

#[godot_api]
impl IControl for SearchLobby {
	fn ready(&mut self) {
		self.update_list();
	}
}

impl SearchLobby {
	pub fn update_list(&self) {
		let steam = steam::Steam::singleton();
		let steam = steam.bind();

		let Some(item_list) = &self.item_list else {
			return;
		};

		// let Ok(list) = steam.request_lobby_list() else {
		// 	godot_error!("Error requesting lobby list");
		// 	return;
		// };

		// for i in list {
		// 	godot_print!("{i:?}");
		// }
	}
}
