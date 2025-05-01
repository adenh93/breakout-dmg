use bevy::prelude::*;
use std::f32::consts::PI;

/// The pre-scaled width of the game window.
pub const DMG_WIDTH: f32 = 160.;

/// The pre-scaled height of the game window.
pub const DMG_HEIGHT: f32 = 144.;

/// The resolution scale used to scale both the game window, and the projection
/// of the orthographic camera.
pub const RESOLUTION_SCALE: f32 = 5.;

// The DMG_COLOR_X constants define the four color tones used by a DMG. On some
// displays these tones were grayscale, but in this case, we are emulating shades of
// green.

/// The 0 (lightest) tone on a DMG display.
pub const DMG_COLOR_0: Color = Color::srgb_u8(155, 188, 15);
/// The 1 (light) tone on a DMG display.
pub const DMG_COLOR_1: Color = Color::srgb_u8(139, 172, 15);
/// The 2 (dark) tone on a DMG display.
pub const DMG_COLOR_2: Color = Color::srgb_u8(48, 98, 48);
/// The 2 (darkest) tone on a DMG display.
pub const DMG_COLOR_3: Color = Color::srgb_u8(15, 56, 15);

/// The farthest top point of the viewable screen.
pub const SCREEN_TOP: f32 = DMG_HEIGHT / 2.;

/// The farthest right point of the viewable screen.
pub const SCREEN_RIGHT: f32 = DMG_WIDTH / 2.;

/// The farthest left point of the viewable screen.
pub const SCREEN_LEFT: f32 = -SCREEN_RIGHT;

/// The farthest bottom point of the viewable screen.
pub const SCREEN_BOTTOM: f32 = -SCREEN_TOP;

/// The length and width of a wall tile.
pub const WALL_TILE_SIZE: f32 = 8.;

/// Half of the length and width of a wall tile, used in positioning logic.
pub const HALF_WALL_TILE: f32 = WALL_TILE_SIZE / 2.;

/// The number of left/right wall tiles to spawn vertically.
pub const WALL_TILE_COUNT_VERTICAL: usize = 17;

/// The number of top wall tiles to spawn horizontally.
pub const WALL_TILE_COUNT_HORIZONTAL: usize = 13;

/// The total length in logical units of the left and right walls.
pub const WALL_LENGTH_VERTICAL: f32 = WALL_TILE_COUNT_VERTICAL as f32 * WALL_TILE_SIZE;

/// The total length in logical units of the top wall.
pub const WALL_LENGTH_HORIZONTAL: f32 = WALL_TILE_COUNT_HORIZONTAL as f32 * WALL_TILE_SIZE;

/// The total length of the top wall including the corner walls, used for
/// positioning logic.
pub const WALL_LENGTH_HORIZONTAL_WITH_CORNERS: f32 = WALL_LENGTH_HORIZONTAL + WALL_TILE_SIZE * 2.;

/// The calculated center point to spawn the left/right walls.
pub const WALL_LOCATION_CENTER: f32 = -WALL_TILE_SIZE + HALF_WALL_TILE;

/// The center of the playable area.
pub const PLAY_AREA_CENTER: f32 = SCREEN_LEFT + WALL_LENGTH_HORIZONTAL_WITH_CORNERS / 2.;

/// The calculated point to spawn the top wall.
pub const WALL_LOCATION_TOP: Vec2 = Vec2::new(PLAY_AREA_CENTER, SCREEN_TOP - HALF_WALL_TILE);

/// The calculated point to spawn the left wall.
pub const WALL_LOCATION_LEFT: Vec2 = Vec2::new(SCREEN_LEFT + HALF_WALL_TILE, WALL_LOCATION_CENTER);

/// The calculated point to spawn the right wall.
pub const WALL_LOCATION_RIGHT: Vec2 = Vec2::new(
    SCREEN_LEFT + WALL_LENGTH_HORIZONTAL_WITH_CORNERS - HALF_WALL_TILE,
    WALL_LOCATION_CENTER,
);

/// The calculated point to spawn the top-left corner wall.
pub const WALL_LOCATION_TOP_LEFT_CORNER: Vec2 =
    Vec2::new(SCREEN_LEFT + HALF_WALL_TILE, SCREEN_TOP - HALF_WALL_TILE);

/// The calculated point to spawn the top-right corner wall.
pub const WALL_LOCATION_TOP_RIGHT_CORNER: Vec2 =
    Vec2::new(WALL_LOCATION_RIGHT.x, SCREEN_TOP - HALF_WALL_TILE);

/// Represents a 90 degrees rotation in radians.
pub const RADIAN_90_DEGREES: f32 = -PI / 2.;

/// Assets directory path for the wall sprite.
pub const WALL_SPRITE_PATH: &str = "sprites/wall.png";

/// Assets directory path for the wall-corner sprite.
pub const WALL_CORNER_SPRITE_PATH: &str = "sprites/wall-corner.png";

/// Assets directory path for the normal brick sprite.
pub const BRICK_NORMAL_SPRITE_PATH: &str = "sprites/brick-normal.png";

/// Assets directory path for the multihit brick sprite.
pub const BRICK_MULTIHIT_SPRITE_PATH: &str = "sprites/brick-multihit.png";

/// Assets directory path for the paddle sprite.
pub const PADDLE_SPRITE_PATH: &str = "sprites/paddle.png";

/// The length and width of a breakable brick.
pub const BRICK_SIZE: Vec2 = Vec2::new(8., 4.);

/// The maximum amount of brick rows that can be spawned.
pub const BRICK_ROW_COUNT: usize = 18;

/// The maximum amount of brick columns that can be spawned.
pub const BRICK_COLUMN_COUNT: usize = 13;

/// The total maximum amount of bricks that can be spawned.
pub const BRICK_COUNT_TOTAL: usize = BRICK_ROW_COUNT * BRICK_COLUMN_COUNT;

/// The calculated point to spawn bricks.
pub const BRICK_SPAWN_LOCATION: Vec2 = Vec2::new(
    SCREEN_LEFT + WALL_TILE_SIZE + BRICK_SIZE.x / 2.,
    SCREEN_TOP - WALL_TILE_SIZE - BRICK_SIZE.y / 2.,
);

/// The length and width of the player paddle.
pub const PADDLE_SIZE: Vec2 = Vec2::new(26., 5.);

/// The amount of space between the paddle and the bottom of the screen
pub const PADDLE_BOTTOM_PADDING: f32 = 8.;

/// The starting position of the player paddle.
pub const PADDLE_STARTING_POSITION: Vec2 = Vec2::new(
    PLAY_AREA_CENTER,
    SCREEN_BOTTOM + PADDLE_BOTTOM_PADDING + PADDLE_SIZE.y / 2.,
);

/// Hardcoded tilesheet representing a basic debug level used
/// for testing game mechanics. This level is not loaded during
/// typical gameplay.
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
