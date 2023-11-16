/*
** OpenScape 2023
** square.rs
** File description:
** square.rs
*/

use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::render::Canvas;

const SQUARE_SIZE: u32 = 50;

pub struct Square<'a> {
    pub rect: Rect,
    pub texture: &'a Texture<'a>
}

impl<'a> Square<'a> {
    pub fn new(x: i32, y: i32, texture: &'a Texture<'a>) -> Square<'a> {
        let rectangle = Rect::new(x, y, SQUARE_SIZE, SQUARE_SIZE);

        Square {
            rect: rectangle,
            texture: texture
        }
    }

    pub fn display(&mut self, x: i32, y: i32, canvas: &mut Canvas<Window>) {
        let dest_rect = Rect::new(self.rect.x + x, self.rect.y + y, self.rect.width(), self.rect.height());

        canvas.copy(&self.texture, None, dest_rect).expect("Failed to apply texture");
    }
}
