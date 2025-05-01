mod components;
mod constants;
mod systems;

use bevy::{prelude::*, window::WindowResolution};
use constants::*;
use systems::startup::{setup_camera, spawn_bricks, spawn_paddle, spawn_walls};

fn get_scaled_window() -> Window {
    let scaled_width = DMG_WIDTH * RESOLUTION_SCALE;
    let scaled_height = DMG_HEIGHT * RESOLUTION_SCALE;

    Window {
        resolution: WindowResolution::new(scaled_width, scaled_height),
        resizable: false,
        ..default()
    }
}

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(get_scaled_window()),
        ..default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(window_plugin)
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(DMG_COLOR_0))
        .add_systems(
            Startup,
            (setup_camera, spawn_walls, spawn_bricks, spawn_paddle),
        )
        .run();
}
