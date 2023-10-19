/*
** OpenScape 2023
** window.rs
** File description:
** window.rs
*/

use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Camera {
    pub x: i32,
    pub y: i32
}

pub fn init_game(size_x: u32, size_y: u32) -> (Canvas<Window>, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem.window("OpenScape", size_x, size_y)
        .position_centered()
        .build()
        .unwrap();
    let canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    (canvas, event_pump)
}
