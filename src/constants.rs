use bevy::prelude::*;
use std::f32::consts::PI;

pub const DMG_WIDTH: f32 = 160.;
pub const DMG_HEIGHT: f32 = 144.;
pub const RESOLUTION_SCALE: f32 = 5.;

pub const DMG_COLOR_0: Color = Color::srgb_u8(155, 188, 15);
pub const DMG_COLOR_1: Color = Color::srgb_u8(139, 172, 15);
pub const DMG_COLOR_2: Color = Color::srgb_u8(48, 98, 48);
pub const DMG_COLOR_3: Color = Color::srgb_u8(15, 56, 15);

pub const WALL_TILE_SIZE: f32 = 8.;
pub const WALL_TILE_COUNT_VERTICAL: usize = 17;
pub const WALL_TILE_COUNT_HORIZONTAL: usize = 13;

pub const SCREEN_TOP: f32 = DMG_HEIGHT / 2.;
pub const SCREEN_BOTTOM: f32 = -SCREEN_TOP;
pub const SCREEN_RIGHT: f32 = DMG_WIDTH / 2.;
pub const SCREEN_LEFT: f32 = -SCREEN_RIGHT;

pub const WALLS_TOP: f32 = SCREEN_TOP - WALL_TILE_SIZE / 2.;
pub const WALLS_BOTTOM: f32 = SCREEN_BOTTOM - WALL_TILE_SIZE / 2.;
pub const WALLS_LEFT: f32 = SCREEN_LEFT + WALL_TILE_SIZE / 2.;
pub const WALLS_RIGHT: f32 = WALLS_LEFT + (WALL_TILE_COUNT_HORIZONTAL as f32 + 1.) * WALL_TILE_SIZE;

pub const RADIAN_90_DEGREES: f32 = -PI / 2.;

pub const WALL_SPRITE_PATH: &str = "sprites/wall.png";
pub const WALL_CORNER_SPRITE_PATH: &str = "sprites/wall-corner.png";
pub const BRICK_NORMAL_SPRITE_PATH: &str = "sprites/brick-normal.png";
pub const BRICK_MULTIHIT_SPRITE_PATH: &str = "sprites/brick-multihit.png";

pub const BRICK_SIZE: Vec2 = Vec2::new(8., 4.);
pub const BRICK_COUNT_VERTICAL: usize = 18;
pub const BRICK_COUNT_HORIZONTAL: usize = 13;
pub const BRICK_COUNT_TOTAL: usize = BRICK_COUNT_VERTICAL * BRICK_COUNT_HORIZONTAL;
pub const BRICK_AREA_TOP_LEFT: Vec2 = Vec2::new(
    SCREEN_LEFT + WALL_TILE_SIZE + BRICK_SIZE.x / 2.,
    SCREEN_TOP - WALL_TILE_SIZE - BRICK_SIZE.y / 2.,
);

#[rustfmt::skip]
pub const DEBUG_LEVEL: [u8; BRICK_COUNT_TOTAL] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 2, 2, 2, 2, 2, 2 ,2 ,2 ,2 ,2 ,2 ,0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 2, 2, 2, 2, 2, 2 ,2 ,2 ,2 ,2 ,2 ,0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 2, 2, 2, 2, 2, 2 ,2 ,2 ,2 ,2 ,2 ,0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 2, 2, 2, 2, 2, 2 ,2 ,2 ,2 ,2 ,2 ,0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
