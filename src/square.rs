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

pub fn create_squares(x: i32, y: i32, rectangle_color: Color, nb_x: i32, nb_y: i32) -> Vec<Vec<Square>> {
    let size: i32 = SQUARE_SIZE as i32;
    let mut vec: Vec<Vec<Square>> = Vec::new();

    for j in 1..nb_y {
        let mut row: Vec<Square> = Vec::new();

        for i in 1..nb_x {
            let square = Square::new(x + size * (i - 1), y + size * (j - 1), rectangle_color);

            row.push(square);
        }
        vec.push(row);
    }
    vec
}
