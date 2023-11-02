/*
** OpenScape 2023
** main.rs
** File description:
** main.rs
*/

extern crate sdl2;
mod window;
mod square;
mod camera;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use square::create_squares;
use camera::Camera;

fn main() {
    let game: (Canvas<Window>, sdl2::EventPump) = window::init_game(800, 600);
    let mut canvas: Canvas<Window> = game.0;
    let mut event_pump: sdl2::EventPump = game.1;
    let vect: Vec<Vec<square::Square>> = create_squares(0, 0, Color::RGB(255, 0, 0), 5, 7);
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::JPG).unwrap();
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut camera = Camera::new(0, 0);

    let texture = texture_creator
        .load_texture("./assets/grass.jpg")
        .expect("Failed to load texture");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Q) => {
                            camera.x -= 10;
                        }
                        Some(Keycode::D) => {
                            camera.x += 10;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        // logic here
        for row in &vect {
            for block in row {
                let dest_rect = Rect::new(block.rect.x + camera.x, block.rect.y + camera.y, block.rect.width(), block.rect.height());
                canvas.copy(&texture, None, dest_rect).expect("Échec du rendu de la texture sur le carré");
            }
        }

        canvas.present();
    }
}
