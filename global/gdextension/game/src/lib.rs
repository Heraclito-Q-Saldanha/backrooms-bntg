mod create_lobby;
mod main_menu;
mod search_lobby;
mod steam;

use godot::prelude::*;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
