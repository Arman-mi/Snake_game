//! Drawing helpers.
//!
//! The game thinks in *grid cells* (whole numbers like `x = 3, y = 7`).
//! The window thinks in *pixels* (floating point numbers like `75.0`).
//! This module converts between the two and draws coloured squares.
//!
//! See GUIDE.md -> Phase 1 and Phase 9.

use piston_window::types::Color;
use piston_window::{Context, G2d, rectangle};

/// How many pixels wide (and tall) one grid cell is.
const BLOCK_SIZE: f64 = 25.0;

/// Converts a grid coordinate into a pixel coordinate.
///
/// Example: with a `BLOCK_SIZE` of 25.0, cell `4` starts at pixel `100.0`.
pub fn to_coord(game_coord: i32) -> f64 {
    todo!("Phase 1: convert a grid coordinate to pixels")
}

/// Same as [`to_coord`], but returns a `u32`.
///
/// The window size has to be given in whole pixels, which is why this exists.
pub fn to_coord_u32(game_coord: i32) -> u32 {
    todo!("Phase 1: reuse to_coord and convert the result")
}

/// Draws a single square cell of `color` at grid position `(x, y)`.
///
/// Not unit tested -- you will check this one by running the game.
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    todo!("Phase 9: draw one block with piston_window::rectangle")
}

/// Draws a rectangle of `color` whose top-left corner is grid cell `(x, y)`
/// and which is `width` cells wide and `height` cells tall.
///
/// Not unit tested -- you will check this one by running the game.
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    todo!("Phase 9: draw a rectangle that spans several blocks")
}

// The tests live in src/tests/draw_tests.rs. Do not edit them.
#[cfg(test)]
#[path = "tests/draw_tests.rs"]
mod tests;
