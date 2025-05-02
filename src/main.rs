mod components;
mod constants;
mod resources;
mod systems;

use bevy::{prelude::*, window::WindowResolution};
use constants::*;
use resources::Keybindings;
use systems::{
    fixed_update::handle_input,
    startup::{setup_camera, spawn_bricks, spawn_paddle, spawn_walls},
};

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(get_scaled_window()),
        ..default()
    };

    let keybindings = Keybindings::load(KEYBINDINGS_PATH).unwrap_or_else(|err| {
        warn!("Couldn't load keybindings. Falling back to defaults. {err:?}");
        Keybindings::default()
    });

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(window_plugin)
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(DMG_COLOR_0))
        .insert_resource(keybindings)
        .add_systems(
            Startup,
            (setup_camera, spawn_walls, spawn_bricks, spawn_paddle),
        )
        .add_systems(FixedUpdate, handle_input)
        .run();
}

fn get_scaled_window() -> Window {
    let scaled_width = DMG_WIDTH * RESOLUTION_SCALE;
    let scaled_height = DMG_HEIGHT * RESOLUTION_SCALE;

    Window {
        resolution: WindowResolution::new(scaled_width, scaled_height),
        resizable: false,
        ..default()
    }
}
