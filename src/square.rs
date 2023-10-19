/*
** OpenScape 2023
** square.rs
** File description:
** square.rs
*/

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;

const SQUARE_SIZE: u32 = 50;

pub struct Square {
    pub rect: Rect,
    pub color: Color,
}

impl Square {
    pub fn new(x: i32, y: i32, rectangle_color: Color) -> Square {
        let rectangle = Rect::new(x, y, SQUARE_SIZE, SQUARE_SIZE);

        Square {
            rect: rectangle,
            color: rectangle_color
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect);
    }
}
