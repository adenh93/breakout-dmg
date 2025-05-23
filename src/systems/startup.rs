use crate::{
    components::{Ball, Brick, BrickVariant, CornerLocation, Paddle, Wall, WallLocation},
    constants::*,
};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
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

pub fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn left/right wall
    commands.spawn(Wall::with_collision(WallLocation::Left, &asset_server));
    commands.spawn(Wall::with_collision(WallLocation::Right, &asset_server));

    // spawn top-left corner wall
    commands.spawn(Wall::with_collision(
        WallLocation::Corner(CornerLocation::TopLeft),
        &asset_server,
    ));

    // Spawn top-right corner wall
    commands.spawn(Wall::with_collision(
        WallLocation::Corner(CornerLocation::TopRight),
        &asset_server,
    ));

    // spawn top wall
    commands.spawn(Wall::with_collision(WallLocation::Top, &asset_server));
}

pub fn spawn_bricks(mut commands: Commands, asset_server: Res<AssetServer>) {
    for (index, variant) in DEBUG_LEVEL.iter().enumerate() {
        if let Some(variant) = BrickVariant::from_u8(*variant) {
            let horizontal_tile = (index % BRICK_COLUMN_COUNT) as f32;
            let vertical_tile = (index / BRICK_COLUMN_COUNT) as f32;

            let mut position = BRICK_SPAWN_LOCATION;
            position.x += horizontal_tile * BRICK_SIZE.x;
            position.y -= vertical_tile * BRICK_SIZE.y;

            commands.spawn(Brick::new(variant, position, &asset_server));
        }
    }
}

pub fn spawn_paddle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Paddle::new(&asset_server));
}

pub fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Ball::new(&asset_server));
}
