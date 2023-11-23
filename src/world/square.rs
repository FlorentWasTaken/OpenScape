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

#[derive(Clone)]
pub struct Square {
    pub rect: Rect
}

pub struct ClonableOptionSquare(Option<Square>);

impl Clone for ClonableOptionSquare{
    fn clone(&self) -> Self {
        ClonableOptionSquare(self.0.clone())
    }
}

impl Square {
    pub fn new(x: i32, y: i32) -> Square {
        let rectangle = Rect::new(x, y, SQUARE_SIZE, SQUARE_SIZE);

        Square {
            rect: rectangle
        }
    }

    pub fn display(&mut self, x: i32, y: i32, canvas: &mut Canvas<Window>, texture: &Texture<'_>) {
        let dest_rect = Rect::new(self.rect.x + x, self.rect.y + y, self.rect.width(), self.rect.height());

        canvas.copy(&texture, None, dest_rect).expect("Failed to apply texture");
    }
}
