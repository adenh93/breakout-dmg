use bevy::{prelude::*, window::WindowResolution};
use std::f32::consts::PI;

const DMG_WIDTH: f32 = 160.;
const DMG_HEIGHT: f32 = 144.;
const RESOLUTION_SCALE: f32 = 5.;

const DMG_COLOR_0: Color = Color::srgb_u8(155, 188, 15);
const DMG_COLOR_1: Color = Color::srgb_u8(139, 172, 15);
const DMG_COLOR_2: Color = Color::srgb_u8(48, 98, 48);
const DMG_COLOR_3: Color = Color::srgb_u8(15, 56, 15);

const WALL_TILE_SIZE: f32 = 8.;
const WALL_TILE_COUNT_VERTICAL: usize = 17;
const WALL_TILE_COUNT_HORIZONTAL: usize = 13;

const SCREEN_TOP: f32 = DMG_HEIGHT / 2.;
const SCREEN_BOTTOM: f32 = -SCREEN_TOP;
const SCREEN_RIGHT: f32 = DMG_WIDTH / 2.;
const SCREEN_LEFT: f32 = -SCREEN_RIGHT;

const WALLS_TOP: f32 = SCREEN_TOP - WALL_TILE_SIZE / 2.;
const WALLS_BOTTOM: f32 = SCREEN_BOTTOM - WALL_TILE_SIZE / 2.;
const WALLS_LEFT: f32 = SCREEN_LEFT + WALL_TILE_SIZE / 2.;
const WALLS_RIGHT: f32 = WALLS_LEFT + (WALL_TILE_COUNT_HORIZONTAL as f32 + 1.) * WALL_TILE_SIZE;

const RADIAN_90_DEGREES: f32 = -PI / 2.;

const WALL_SPRITE_PATH: &str = "sprites/wall.png";
const WALL_CORNER_SPRITE_PATH: &str = "sprites/wall-corner.png";

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Sprite, Transform)]
struct Wall;

enum WallLocation {
    Left(f32 /*offset*/),
    Right(f32 /*offset*/),
    Top(f32 /*offset*/),
    Corner(CornerLocation),
}

enum CornerLocation {
    TopLeft,
    TopRight,
}

impl CornerLocation {
    fn position(&self) -> Vec2 {
        match self {
            Self::TopLeft => Vec2::new(WALLS_LEFT, WALLS_TOP),
            Self::TopRight => Vec2::new(WALLS_RIGHT, WALLS_TOP),
        }
    }

    fn flip_x(&self) -> bool {
        match self {
            Self::TopLeft => false,
            Self::TopRight => true,
        }
    }
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            Self::Left(offset) => Vec2::new(WALLS_LEFT, WALLS_BOTTOM + offset * WALL_TILE_SIZE),
            Self::Right(offset) => Vec2::new(WALLS_RIGHT, WALLS_BOTTOM + offset * WALL_TILE_SIZE),
            Self::Top(offset) => Vec2::new(WALLS_LEFT + offset * WALL_TILE_SIZE, WALLS_TOP),
            Self::Corner(location) => location.position(),
        }
    }

    fn asset_path(&self) -> &str {
        match self {
            Self::Corner(_) => WALL_CORNER_SPRITE_PATH,
            _ => WALL_SPRITE_PATH,
        }
    }

    fn rotation(&self) -> Quat {
        match self {
            Self::Top(_) => Quat::from_rotation_z(RADIAN_90_DEGREES),
            _ => Quat::default(),
        }
    }

    fn flip_x(&self) -> bool {
        match self {
            Self::Left(_) | Self::Top(_) => false,
            Self::Right(_) => true,
            Self::Corner(location) => location.flip_x(),
        }
    }
}

impl Wall {
    fn new(location: WallLocation, asset_server: &Res<AssetServer>) -> (Wall, Sprite, Transform) {
        let sprite = Sprite {
            image: asset_server.load(location.asset_path()),
            flip_x: location.flip_x(),
            ..default()
        };

        let transform = Transform::from_translation(location.position().extend(0.))
            .with_rotation(location.rotation());

        (Wall, sprite, transform)
    }

    fn with_collision(
        location: WallLocation,
        asset_server: &Res<AssetServer>,
    ) -> (Wall, Sprite, Transform, Collider) {
        let (wall, sprite, transform) = Self::new(location, asset_server);
        (wall, sprite, transform, Collider)
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(get_window_plugin())
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(DMG_COLOR_0))
        .add_systems(Startup, (setup, spawn_walls))
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
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: DMG_WIDTH,
                height: DMG_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn left/right walls
    for offset in 1..=WALL_TILE_COUNT_VERTICAL {
        commands.spawn(Wall::with_collision(
            WallLocation::Left(offset as f32),
            &asset_server,
        ));

        commands.spawn(Wall::with_collision(
            WallLocation::Right(offset as f32),
            &asset_server,
        ));
    }

    // spawn top-left corner wall
    commands.spawn(Wall::new(
        WallLocation::Corner(CornerLocation::TopLeft),
        &asset_server,
    ));

    // Spawn top-right corner wall
    commands.spawn(Wall::new(
        WallLocation::Corner(CornerLocation::TopRight),
        &asset_server,
    ));

    // spawn top walls
    for offset in 1..=WALL_TILE_COUNT_HORIZONTAL {
        commands.spawn(Wall::with_collision(
            WallLocation::Top(offset as f32),
            &asset_server,
        ));
    }
}
