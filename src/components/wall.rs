use super::Collider;
use crate::constants::*;
use bevy::prelude::*;

pub enum CornerLocation {
    TopLeft,
    TopRight,
}

impl CornerLocation {
    fn position(&self) -> Vec2 {
        match self {
            Self::TopLeft => WALL_LOCATION_TOP_LEFT_CORNER,
            Self::TopRight => WALL_LOCATION_TOP_RIGHT_CORNER,
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
    Left,
    Right,
    Top,
    Corner(CornerLocation),
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            Self::Left => WALL_LOCATION_LEFT,
            Self::Right => WALL_LOCATION_RIGHT,
            Self::Top => WALL_LOCATION_TOP,
            Self::Corner(location) => location.position(),
        }
    }

    fn asset_path(&self) -> &str {
        match self {
            Self::Corner(_) => WALL_CORNER_SPRITE_PATH,
            Self::Top => WALL_TOP_SPRITE_PATH,
            _ => WALL_SPRITE_PATH,
        }
    }

    fn size(&self) -> Vec2 {
        match self {
            Self::Left | Self::Right => Vec2::new(WALL_TILE_SIZE, WALL_LENGTH_VERTICAL),
            Self::Top => Vec2::new(WALL_LENGTH_HORIZONTAL, WALL_TILE_SIZE),
            _ => Vec2::new(WALL_TILE_SIZE, WALL_TILE_SIZE),
        }
    }

    fn flip_x(&self) -> bool {
        match self {
            Self::Left | Self::Top => false,
            Self::Right => true,
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
        let image_mode = SpriteImageMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.,
        };

        let sprite = Sprite {
            image: asset_server.load(location.asset_path()),
            flip_x: location.flip_x(),
            custom_size: Some(Vec2::ONE),
            image_mode,
            ..default()
        };

        let transform = Transform::from_translation(location.position().extend(0.))
            .with_scale(location.size().extend(1.));

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
