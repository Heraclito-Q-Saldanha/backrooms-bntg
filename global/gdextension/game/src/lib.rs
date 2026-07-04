mod main_menu;
mod steam;

use godot::prelude::*;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
