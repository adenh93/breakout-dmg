use super::Velocity;
use crate::constants::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(Sprite, Transform, Velocity)]
pub struct Ball;

impl Ball {
    pub fn new(asset_server: &AssetServer) -> (Ball, Sprite, Transform, Velocity) {
        let mut sprite = Sprite::from_image(asset_server.load(BALL_SPRITE_PATH));
        sprite.custom_size = Some(Vec2::ONE);

        let transform = Transform::from_translation(BALL_START_POSITION.extend(0.))
            .with_scale(Vec2::new(BALL_SIZE, BALL_SIZE).extend(1.));

        (Ball, sprite, transform, Velocity::default())
    }
}
