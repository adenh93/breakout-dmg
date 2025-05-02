use bevy::prelude::*;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}
