use super::Collider;
use crate::constants::{
    RADIAN_90_DEGREES, WALL_CORNER_SPRITE_PATH, WALL_SPRITE_PATH, WALL_TILE_SIZE, WALLS_BOTTOM,
    WALLS_LEFT, WALLS_RIGHT, WALLS_TOP,
};
use bevy::prelude::*;

pub enum CornerLocation {
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

pub enum WallLocation {
    Left(f32 /*offset*/),
    Right(f32 /*offset*/),
    Top(f32 /*offset*/),
    Corner(CornerLocation),
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

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Wall;

impl Wall {
    pub fn new(
        location: WallLocation,
        asset_server: &Res<AssetServer>,
    ) -> (Wall, Sprite, Transform) {
        let sprite = Sprite {
            image: asset_server.load(location.asset_path()),
            flip_x: location.flip_x(),
            ..default()
        };

        let transform = Transform::from_translation(location.position().extend(0.))
            .with_rotation(location.rotation());

        (Wall, sprite, transform)
    }

    pub fn with_collision(
        location: WallLocation,
        asset_server: &Res<AssetServer>,
    ) -> (Wall, Sprite, Transform, Collider) {
        let (wall, sprite, transform) = Self::new(location, asset_server);
        (wall, sprite, transform, Collider)
    }
}
