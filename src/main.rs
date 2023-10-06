/*
** OpenScape 2023
** main.rs
** File description:
** main.rs
*/

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {
    let scr_w:i32 = 800;
    let scr_h:i32 = 600;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("OpenScape", scr_w as u32, scr_h as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let square_width:i32 = 100;
    let square_height:i32 = 100;
    let square_x:i32 = (scr_w - square_width) / 2;
    let square_y:i32 = (scr_h - square_height) / 2;

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
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(Rect::new(square_x + (square_x as f32 * 0.05) as i32, square_y + (square_y as f32 * 0.05) as i32, ((scr_w as f32) * 0.05) as u32, ((scr_w as f32) * 0.05) as u32));

        canvas.present();
    }
}
