use crate::constants::*;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::new(BALL_SPEED, BALL_SPEED))
    }
}
