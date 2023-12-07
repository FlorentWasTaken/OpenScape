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
mod command;

use sdl2::pixels::Color;
use sdl2::ttf;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use camera::Camera;
use sdl2::video::WindowContext;
use world::create_block;
use world::init_world;
use sky::manage_day_time;
use world::remove_block;
use std::time::Instant;
use info::draw_info;
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use world::square::Square;
use command::listen_commands;

const CUBE_SIZE: i32 = 50;
const WORLD_SIZE: i32 = 100;

lazy_static! {
    pub static ref GLOBAL_VECT: Arc<Mutex<Vec<Vec<Option<Square>>>>> = Arc::new(Mutex::new(init_world(WORLD_SIZE as u32)));
}

lazy_static! {
    pub static ref GLOBAL_CAMERA: Arc<Mutex<Camera>> = Arc::new(Mutex::new(Camera::new(0, 0)));
}

fn manage_events(event_pump: &mut sdl2::EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            Event::KeyDown { keycode, .. } => {
                match keycode {
                    Some(Keycode::Q) => {
                        let mut camera = GLOBAL_CAMERA.lock().unwrap();

                        camera.x += 10;
                    }
                    Some(Keycode::D) => {
                        let mut camera = GLOBAL_CAMERA.lock().unwrap();

                        camera.x -= 10;
                    }
                    Some(Keycode::Z) => {
                        let mut camera = GLOBAL_CAMERA.lock().unwrap();

                        camera.y += 10;
                    }
                    Some(Keycode::S) => {
                        let mut camera = GLOBAL_CAMERA.lock().unwrap();

                        camera.y -= 10;
                    }
                    _ => {}
                }
            }
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                match mouse_btn {
                    MouseButton::Left => {
                        remove_block(x, y);
                    }
                    MouseButton::Right => {
                        create_block((x / CUBE_SIZE) * CUBE_SIZE, (y / CUBE_SIZE) * CUBE_SIZE);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    false
}

fn main() {
    let game: (Canvas<Window>, sdl2::EventPump) = window::init_game(800, 600);
    let mut canvas: Canvas<Window> = game.0;
    let mut event_pump: sdl2::EventPump = game.1;
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::JPG).unwrap();
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    let texture = texture_creator
        .load_texture("./assets/grass.jpg".to_string())
        .expect("Failed to load texture");

    let start_time = Instant::now();
    let mut frames: f32 = 0.0;
    let mut prev_frame_time = Instant::now();
    let ttf_context = ttf::init().unwrap();
    let font = ttf_context.load_font("assets/Basic-Regular.ttf", 36).unwrap();
    let mut fps_text = texture_creator.create_texture_from_surface(
        font.render("FPS: 0").blended(Color::WHITE).unwrap(),
    ).unwrap();

    listen_commands();
    world::script::run_script().unwrap();
    loop {
        if manage_events(&mut event_pump) == true {
            break;
        }

        manage_day_time(start_time, &mut canvas);
        let mut vect = GLOBAL_VECT.lock().unwrap();
        let camera = GLOBAL_CAMERA.lock().unwrap();

        for row in vect.iter_mut() {
            for block in row.iter_mut() {
                if let Some(inner_square) = block.as_mut() {
                    inner_square.display(camera.x, camera.y, &mut canvas, &texture);
                }
            }
        }
        draw_info(&mut frames, &mut prev_frame_time, &texture_creator, &mut fps_text, &font, &mut canvas);

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
