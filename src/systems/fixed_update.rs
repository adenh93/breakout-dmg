use crate::constants::*;
use crate::{components::Paddle, resources::Keybindings};
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
