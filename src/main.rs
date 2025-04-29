use bevy::{prelude::*, window::WindowResolution};

const DMG_WIDTH: f32 = 166.;
const DMG_HEIGHT: f32 = 140.;
const RESOLUTION_SCALE: f32 = 3.;

const DMG_COLOR_0: Color = Color::srgb_u8(155, 188, 15);
const DMG_COLOR_1: Color = Color::srgb_u8(139, 172, 15);
const DMG_COLOR_2: Color = Color::srgb_u8(48, 98, 48);
const DMG_COLOR_3: Color = Color::srgb_u8(15, 56, 15);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(get_window_plugin()))
        .insert_resource(ClearColor(DMG_COLOR_0))
        .add_systems(Startup, setup)
        .run();
}

fn get_window_plugin() -> WindowPlugin {
    let scaled_width = DMG_WIDTH * RESOLUTION_SCALE;
    let scaled_height = DMG_HEIGHT * RESOLUTION_SCALE;

    let window = Window {
        resolution: WindowResolution::new(scaled_width, scaled_height),
        ..default()
    };

    WindowPlugin {
        primary_window: Some(window),
        ..default()
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
