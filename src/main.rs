// While the project is unfinished, most functions are `todo!()` and their
// parameters are unused, which would bury you in warnings. This line hides
// those warnings. Delete it in Phase 11 and fix whatever the compiler reports.
#![allow(dead_code, unused_variables, unused_imports)]

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord_u32;
use crate::game::Game;

/// Background colour of the window (grey).
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    // Phase 10: build the window and run the game loop.
    //
    // 1. Pick a board size in grid cells (the original uses 30 x 30).
    // 2. Create a `PistonWindow` with `WindowSettings`. Its size is in pixels,
    //    so convert with `to_coord_u32`. Make Esc close the window.
    // 3. Create a `Game`.
    // 4. Loop over the window's events. For each event:
    //      - if it is a keyboard press, pass the key to the game
    //      - draw: clear the screen with BACK_COLOR, then draw the game
    //      - if it is an update event, advance the game by its delta time
    // todo!("Phase 10")
    let (width, height) = (20,20);
    let mut window: PistonWindow = WindowSettings::new("Snake",[to_coord_u32(width),to_coord_u32(height)]).exit_on_esc(true).build().unwrap();
    let mut game = Game::new(width,height);


    while let Some(event) = window.next(){
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g,_|{
            clear(BACK_COLOR,g);
            game.draw(&c, g);
        },);
        event.update(|args| {
            game.update(args.dt);
        });
    }


}
