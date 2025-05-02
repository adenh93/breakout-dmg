use crate::components::{Ball, Brick, Collider, Velocity};
use crate::constants::*;
use crate::events::CollisionEvent;
use crate::{components::Paddle, resources::Keybindings};
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

const MOVE_LEFT: f32 = -1.;
const MOVE_RIGHT: f32 = 1.;
const NO_MOVEMENT: f32 = 0.;

pub fn handle_input(
    keybindings: Res<Keybindings>,
    input: Res<ButtonInput<KeyCode>>,
    paddle: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut transform = paddle.into_inner();

    let new_direction = if input.any_pressed(keybindings.move_left.clone()) {
        MOVE_LEFT
    } else if input.any_pressed(keybindings.move_right.clone()) {
        MOVE_RIGHT
    } else {
        NO_MOVEMENT
    };

    let new_paddle_position =
        transform.translation.x + new_direction * PADDLE_SPEED * time.delta_secs();

    transform.translation.x = new_paddle_position.clamp(PADDLE_LEFT_BOUND, PADDLE_RIGHT_BOUND);
}

pub fn ball_movement(ball: Single<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    let (mut transform, velocity) = ball.into_inner();
    transform.translation.x += velocity.0.x * time.delta_secs();
    transform.translation.y += velocity.0.y * time.delta_secs();
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Thanks and credits to the official Bevy examples for this first-draft collision system:
// https://github.com/bevyengine/bevy/blob/main/examples/games/breakout.rs
pub fn check_for_collisions(
    mut commands: Commands,
    ball: Single<(&mut Velocity, &Transform), With<Ball>>,
    mut collider_query: Query<(Entity, &Transform, Option<&mut Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut velocity, ball_transform) = ball.into_inner();

    for (collider_entity, collider_transform, maybe_brick) in &mut collider_query {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_SIZE / 2.),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            collision_events.write_default();

            if maybe_brick.is_some() {
                commands.entity(collider_entity).despawn();
            }

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = velocity.x > 0.,
                Collision::Right => reflect_x = velocity.x < 0.,
                Collision::Top => reflect_y = velocity.y < 0.,
                Collision::Bottom => reflect_y = velocity.y > 0.,
            }

            if reflect_x {
                velocity.x = -velocity.x;
            }

            if reflect_y {
                velocity.y = -velocity.y;
            }
        }
    }
}

fn ball_collision(ball_bounds: BoundingCircle, box_bounds: Aabb2d) -> Option<Collision> {
    // Check if no intersection/collision has occured.
    if !ball_bounds.intersects(&box_bounds) {
        return None;
    }

    let closest = box_bounds.closest_point(ball_bounds.center());
    let offset = ball_bounds.center() - closest;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
