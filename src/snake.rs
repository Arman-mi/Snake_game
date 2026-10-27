//! The snake: where it is, which way it is heading, and how it moves.
//!
//! See GUIDE.md -> Phases 2 to 5.

use std::collections::LinkedList;

use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::draw_block;

/// Red, green, blue, alpha -- each from 0.0 to 1.0.
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

/// The four ways the snake can travel.
///
/// Remember: on screen, `y` grows *downwards*. Moving `Up` makes `y` smaller.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns the direction pointing the other way (`Up` <-> `Down`,
    /// `Left` <-> `Right`).
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

/// One square of the snake's body, in grid coordinates.
#[derive(Debug, Clone, PartialEq)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    /// The direction the snake is currently travelling.
    direction: Direction,
    /// The body, ordered from head (front) to tail end (back).
    body: LinkedList<Block>,
    /// The block that was removed from the back during the most recent move,
    /// or `None` if the snake has not moved yet.
    tail: Option<Block>,
}

impl Snake {
    /// Creates a snake that is three blocks long, lying horizontally on row
    /// `y`, facing `Right`, and that has not moved yet.
    ///
    /// The *tail end* is at `(x, y)` and the *head* is two cells to the right
    /// of it. Remember that `body` is ordered head first.
    pub fn new(x: i32, y: i32) -> Snake {
        // todo!("Phase 3")
        let mut body = LinkedList::new();
        body.push_front(Block {
            x: x,
            y: y,
        });
        body.push_front(
            Block { x:x+1, y }
        ); 
        body.push_front(Block { x:x+2, y });

        Snake { direction:Direction::Right , body, tail:None }
        
        
    
    }

    /// Draws every block of the snake in `SNAKE_COLOR`.
    ///
    /// Not unit tested -- you will check this one by running the game.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        todo!("Phase 9")
    }

    /// Returns the `(x, y)` grid position of the head.
    pub fn head_position(&self) -> (i32, i32) {
        
        let curr_block = self.body.front().unwrap();
        return (curr_block.x, curr_block.y)
    }

    /// Returns the direction the snake is currently travelling.
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    /// Moves the snake one cell forward.
    ///
    /// * If `dir` is `Some(d)`, the snake turns to face `d` first.
    /// * If `dir` is `None`, it keeps its current direction.
    ///
    /// Moving adds a new block in front of the head and removes the last
    /// block. The removed block must be remembered in `self.tail` so that
    /// [`Snake::restore_tail`] can give it back later.
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        todo!("Phase 4")
    }

    /// Returns where the head *would* be after one move in `dir` (or in the
    /// current direction if `dir` is `None`).
    ///
    /// This only looks ahead: it must not change the snake.
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        todo!("Phase 4")
    }

    /// Grows the snake by putting the most recently removed tail block back
    /// on the end of the body. Called right after the snake eats.
    pub fn restore_tail(&mut self) {
        todo!("Phase 5")
    }

    /// Returns `true` if `(x, y)` is covered by any block of the snake
    /// *except the very last one*.
    ///
    /// GUIDE.md explains why the last block is skipped.
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        todo!("Phase 5")
    }

    /// Test helper (already done for you): the number of blocks in the body.
    /// It only exists when running `cargo test`.
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.body.len()
    }
}

// The tests live in src/tests/snake_tests.rs. Do not edit them.
#[cfg(test)]
#[path = "tests/snake_tests.rs"]
mod tests;
