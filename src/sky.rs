/*
** OpenScape 2023
** sky.rs
** File description:
** sky.rs
*/

use std::time::Instant;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;

const CYCLE_DURATION: f32 = 60.0;
const SKY_BLUE: Color = Color::RGB(135, 206, 235);
const NIGHT_COLOR: Color = Color::RGB(0, 0, 50);

pub fn manage_day_time(start_time: Instant, canvas: &mut Canvas<Window>) {
    let elapsed_time = start_time.elapsed().as_secs_f32();
    let cycle_progress = (elapsed_time % CYCLE_DURATION) / CYCLE_DURATION;
    let transition_progress = (cycle_progress * std::f32::consts::PI).sin();
    let r = ((SKY_BLUE.r as f32) * (1.0 - transition_progress) + (NIGHT_COLOR.r as f32) * transition_progress) as u8;
    let g = ((SKY_BLUE.g as f32) * (1.0 - transition_progress) + (NIGHT_COLOR.g as f32) * transition_progress) as u8;
    let b = ((SKY_BLUE.b as f32) * (1.0 - transition_progress) + (NIGHT_COLOR.b as f32) * transition_progress) as u8;
    let color = Color::RGB(r, g, b);

    canvas.set_draw_color(color);
    canvas.clear();
}
