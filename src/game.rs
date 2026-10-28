//! The game rules: the board, the food, collisions, timing and restarting.
//!
//! See GUIDE.md -> Phases 6 to 9.

use std::arch::x86_64::_mm_getmant_round_sd;

use piston_window::types::Color;
use piston_window::{Context, G2d, Key};
use rand::{Rng, thread_rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

/// Seconds between automatic snake moves. Smaller = faster snake.
const MOVING_PERIOD: f64 = 0.1;
/// Seconds to wait on the game-over screen before starting again.
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    /// Board size in grid cells, *including* the one-cell border on each side.
    width: i32,
    height: i32,

    game_over: bool,
    /// Seconds since the snake last moved (or since game over started).
    waiting_time: f64,
}

impl Game {
    /// Creates a new game on a `width` x `height` board.
    ///
    /// Starting state:
    /// * the snake is `Snake::new(2, 2)`
    /// * food exists at `(6, 4)`
    /// * the game is not over and `waiting_time` is `0.0`
    pub fn new(width: i32, height: i32) -> Game {
        let snake:Snake = Snake::new(2,2);
        let food_exists = true;
        let (food_x,food_y) = (6,4);
        let game_over = false; 
        let waiting_time = 0.0;

        return Game{
            snake,
            food_exists,
            food_x,
            food_y,
            width,
            height,
            game_over,
            waiting_time,

        }
        



    }

    /// Reacts to a key press.
    ///
    /// * Does nothing if the game is over.
    /// * The arrow keys choose a direction. Any other key keeps the current
    ///   direction.
    /// * Pressing the opposite of the current direction is ignored (the
    ///   snake cannot reverse into itself).
    /// * Otherwise the snake is updated immediately in the chosen direction.
    pub fn key_pressed(&mut self, key: Key) {
        // phase 8

        if self.game_over ==true{
            return
        }
        let dir = match key{
            Key::Up | Key::W => Some(Direction::Up),
            Key::Down | Key::S => Some(Direction::Down),
            Key::Left | Key::A => Some(Direction::Left),
            Key::Right | Key::D => Some(Direction::Right),
            _=> Some(self.snake.head_direction())

        };
        
        if self.snake.head_direction().opposite() == dir.unwrap() {
            return
        }
        self.update_snake(dir);



    }

    /// Draws the snake, the food (if any), the four border walls and, when
    /// the game is over, a see-through red overlay on the whole board.
    ///
    /// Not unit tested -- you will check this one by running the game.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // todo!("Phase 9")
        self.snake.draw(con, g);
        if self.food_exists{
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        //top wall
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);

        //left side wall
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
       
        //bottom wall
        draw_rectangle(BORDER_COLOR, 0, self.height, self.width-1, 1, con, g);
        
        //right side wall
        draw_rectangle(BORDER_COLOR, self.width, 0, 1, self.height-1, con, g);

        if self.game_over{
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }

    }

    /// Advances the game clock by `delta_time` seconds.
    ///
    /// * Always adds `delta_time` to `waiting_time` first.
    /// * If the game is over: restart once `waiting_time` is greater than
    ///   `RESTART_TIME`, and do nothing else.
    /// * If there is no food, add some.
    /// * If `waiting_time` is greater than `MOVING_PERIOD`, move the snake
    ///   one step in its current direction.
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;


        if self.game_over{
            if self.waiting_time > RESTART_TIME{
            self.restart();
            
            }
            return 
        }
        if !self.food_exists{
            self.add_food();
        }
        if self.waiting_time > MOVING_PERIOD{
            self.update_snake(Option::Some(self.snake.head_direction()));
        }

    }

    /// If the head is on the food: the food is eaten (it no longer exists)
    /// and the snake grows by one block.
    fn check_eating(&mut self) {
        //phase 7
        // let  snake = &mut self.snake;
        if self.snake.head_position() == (self.food_x,self.food_y){
            self.food_exists = false;
            self.snake.restore_tail();
        }

    }






    /// Returns `true` if the snake can safely make its next move in `dir`.
    ///
    /// The move is deadly if the next head position hits the snake's own
    /// body or lands on (or past) any of the four border walls.
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        //phase 7
        let snake = &self.snake;
       
        let (head_x,head_y) = snake.next_head(dir);

        let (width,height) = (self.width,self.height);
        if !(0< head_x && head_x < width-1 && 0<head_y && head_y < height-1){
            return false
        }
        let body_collision = snake.overlap_tail(head_x, head_y);
        return !body_collision


    }

    /// Puts food on a random cell that is inside the walls and not on the
    /// snake, and marks the food as existing.
    fn add_food(&mut self) {

        let mut  random_x = thread_rng().gen_range(1..self.width-1);
        let mut random_y = thread_rng().gen_range(1..self.height-1);
        while self.snake.overlap_tail(random_x,random_y){
        random_x = thread_rng().gen_range(1..self.width-1);
         random_y = thread_rng().gen_range(1..self.height-1);
        }
        (self.food_x,self.food_y) = (random_x,random_y);
        self.food_exists = true; 

        

        



    }

    /// Takes one step of the game in `dir`.
    ///
    /// * If the move is safe: move the snake, then check whether it ate.
    /// * Otherwise: the game is over (and the snake does not move).
    /// * Either way, `waiting_time` goes back to `0.0`.
    fn update_snake(&mut self, dir: Option<Direction>) {
        
        if self.check_if_snake_alive(dir){
            self.snake.move_forward(dir);
            self.check_eating();
        }else {
            self.game_over = true;

        }
        self.waiting_time = 0.0;
    }

    /// Puts everything back to the starting state described in
    /// [`Game::new`]. The board size stays the same.
    fn restart(&mut self) {

        let (current_width, current_height ) = (self.width, self.height);
        *self = Game::new(current_width, current_height);

    }
}

// The tests live in src/tests/game_tests.rs. Do not edit them.
#[cfg(test)]
#[path = "tests/game_tests.rs"]
mod tests;
