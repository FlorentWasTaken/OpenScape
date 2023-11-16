/*
** OpenScape 2023
** main.rs
** File description:
** main.rs
*/

extern crate sdl2;
mod window;
mod camera;
mod world;
mod sky;
mod info;

use sdl2::pixels::Color;
use sdl2::ttf;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use camera::Camera;
use world::init_world;
use sky::manage_day_time;
use std::time::Instant;
use info::draw_info;

const CUBE_SIZE: i32 = 50;

fn main() {
    let game: (Canvas<Window>, sdl2::EventPump) = window::init_game(800, 600);
    let mut canvas: Canvas<Window> = game.0;
    let mut event_pump: sdl2::EventPump = game.1;
    let mut vect: Vec<Vec<Option<world::square::Square>>> = init_world();
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::JPG).unwrap();
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut camera = Camera::new(0, 0);

    let texture = texture_creator
        .load_texture("./assets/grass.jpg")
        .expect("Failed to load texture");

    let start_time = Instant::now();
    let mut frames: f32 = 0.0;
    let mut prev_frame_time = Instant::now();
    let ttf_context = ttf::init().unwrap();
    let font = ttf_context.load_font("assets/Basic-Regular.ttf", 36).unwrap();
    let mut fps_text = texture_creator.create_texture_from_surface(
        font.render("FPS: 0").blended(Color::WHITE).unwrap(),
    ).unwrap();


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
                            camera.x += 10;
                        }
                        Some(Keycode::D) => {
                            camera.x -= 10;
                        }
                        Some(Keycode::Z) => {
                            camera.y += 10;
                        }
                        Some(Keycode::S) => {
                            camera.y -= 10;
                        }
                        _ => {}
                    }
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            if let Some(val) = vect.get_mut((y / CUBE_SIZE) as usize) {
                                if let Some(bloc) = val.get_mut((x / CUBE_SIZE) as usize) {
                                    *bloc = None;
                                }
                            }
                        }
                        MouseButton::Right => {
                            if let Some(val) = vect.get_mut((y / CUBE_SIZE) as usize) {
                                if let Some(bloc) = val.get_mut((x / CUBE_SIZE) as usize) {
                                    *bloc = Some(world::square::Square::new((x / CUBE_SIZE) * CUBE_SIZE, (y / CUBE_SIZE) * CUBE_SIZE));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        manage_day_time(start_time, &mut canvas);
        draw_info(&mut frames, &mut prev_frame_time, &texture_creator, &mut fps_text, &font, &mut canvas);

        // logic here
        for row in &vect {
            for block in row {
                if let Some(inner_square) = block.as_ref() {
                    let dest_rect = Rect::new(inner_square.rect.x + camera.x, inner_square.rect.y + camera.y, inner_square.rect.width(), inner_square.rect.height());
                    canvas.copy(&texture, None, dest_rect).expect("Failed to apply texture");
                }
            }
        }

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
