extern crate piston_window;
extern crate rand;
extern crate fps_counter;

use piston_window::*;
use std::time::{Duration, Instant};

mod board;
use crate::board::Board;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Game of Life", [512, 512])
            .exit_on_esc(true).build().unwrap();

    let mut board = Board::new();

    let color_on = [1.0, 0.2, 0.2, 1.0];
    let color_off = [0.05, 0.05, 0.05, 1.0];

    let mut last_draw = Instant::now();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            let now = Instant::now();

            if now - last_draw > Duration::from_millis(75) {
                clear([0.0, 0.0, 0.0, 1.0], graphics);

                for y in 0..32 {
                    for x in 0..32 {
                        rectangle(
                            if board.state(x, y) {
                                color_on
                            } else {
                                color_off
                            },
                            [(x as f64) * 16.0 + 2.0, (y as f64) * 16.0 + 2.0, 12.0, 12.0],
                            context.transform,
                            graphics
                        );
                    }
                }

                board.next();
                last_draw = now;
            }
        });
    }
}