use super::Collider;
use crate::constants::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

impl Paddle {
    pub fn new(asset_server: &AssetServer) -> (Paddle, Sprite, Transform, Collider) {
        let sprite = Sprite {
            image: asset_server.load(PADDLE_SPRITE_PATH),
            custom_size: Some(Vec2::ONE),
            ..default()
        };

        let transform = Transform::from_translation(PADDLE_STARTING_POSITION.extend(1.))
            .with_scale(PADDLE_SIZE.extend(0.));

        (Paddle, sprite, transform, Collider)
    }
}
