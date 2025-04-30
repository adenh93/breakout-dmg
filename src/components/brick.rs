use super::Collider;
use crate::constants::{BRICK_MULTIHIT_SPRITE_PATH, BRICK_NORMAL_SPRITE_PATH};
use bevy::prelude::*;

const NORMAL: u8 = 1;
const MULTIHIT: u8 = 2;

pub enum BrickVariant {
    Normal,
    MultiHit,
}

impl BrickVariant {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            NORMAL => Some(Self::Normal),
            MULTIHIT => Some(Self::MultiHit),
            _ => None,
        }
    }

    fn asset_path(&self) -> &str {
        match self {
            Self::Normal => BRICK_NORMAL_SPRITE_PATH,
            Self::MultiHit => BRICK_MULTIHIT_SPRITE_PATH,
        }
    }
}

#[derive(Component, Default)]
#[require(Sprite, Transform, Collider)]
pub struct Brick;

impl Brick {
    pub fn new(
        variant: BrickVariant,
        position: Vec2,
        asset_server: &Res<AssetServer>,
    ) -> (Brick, Sprite, Transform, Collider) {
        let sprite = Sprite {
            image: asset_server.load(variant.asset_path()),
            ..default()
        };

        let transform = Transform::from_translation(position.extend(0.));
        (Brick, sprite, transform, Collider)
    }
}
