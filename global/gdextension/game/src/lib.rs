mod main_menu;

use godot::prelude::*;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}
