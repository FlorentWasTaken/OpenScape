/*
** OpenScape 2023
** info.rs
** File description:
** info.rs
*/

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::TextureCreator;
use std::time::{Duration, Instant};
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::render::Texture;

pub fn draw_info<'a>(frames: &mut f32, prev_frame_time: &mut Instant, texture_creator: &'a TextureCreator<WindowContext>, fps_text: &mut Texture<'a>, font: &Font, canvas: &mut Canvas<Window>) {
    *frames += 1.0;

    let now = Instant::now();
    let frame_time = now - *prev_frame_time;

    if frame_time >= Duration::from_secs(1) {
        let fps = *frames / frame_time.as_secs_f32();
        *fps_text = texture_creator
            .create_texture_from_surface(
                &font.render(&format!("FPS: {:.2}", fps)).blended(Color::WHITE).unwrap(),
            )
            .unwrap();
        *prev_frame_time = now;
        *frames = 0.0;
    }
    canvas.copy(&fps_text, None, Some(Rect::new(10, 10, 100, 50))).unwrap();
}
