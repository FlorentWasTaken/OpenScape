/*
** OpenScape 2023
** main.rs
** File description:
** main.rs
*/

extern crate sdl2;
mod window;
mod square;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

fn main() {
    let game: (Canvas<Window>, sdl2::EventPump) = window::init_game(800, 600);
    let mut canvas: Canvas<Window> = game.0;
    let mut event_pump: sdl2::EventPump = game.1;
    let square = square::Square::new(0, 0, Color::RGB(255, 0, 0));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        // logic here
        square.draw(&mut canvas);

        canvas.present();
    }
}
