use crate::{
    components::{Brick, BrickVariant, CornerLocation, Wall, WallLocation},
    constants::{
        BRICK_AREA_TOP_LEFT, BRICK_COUNT_HORIZONTAL, BRICK_SIZE, DEBUG_LEVEL, DMG_HEIGHT,
        DMG_WIDTH, WALL_TILE_COUNT_HORIZONTAL, WALL_TILE_COUNT_VERTICAL,
    },
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

pub fn spawn_bricks(mut commands: Commands, asset_server: Res<AssetServer>) {
    for (index, variant) in DEBUG_LEVEL.iter().enumerate() {
        if let Some(variant) = BrickVariant::from_u8(*variant) {
            let horizontal_tile = (index % BRICK_COUNT_HORIZONTAL) as f32;
            let vertical_tile = (index / BRICK_COUNT_HORIZONTAL) as f32;

            let mut position = BRICK_AREA_TOP_LEFT;
            position.x += horizontal_tile * BRICK_SIZE.x;
            position.y -= vertical_tile * BRICK_SIZE.y;

            commands.spawn(Brick::new(variant, position, &asset_server));
        }
    }
}
