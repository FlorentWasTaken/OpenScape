/*
** OpenScape 2023
** camera.rs
** File description:
** camera.rs
*/

pub struct Camera {
    pub x: i32,
    pub y: i32,
}

impl Camera {
    pub fn new(x_pos: i32, y_pos: i32) -> Camera {
        Camera {x: x_pos, y: y_pos}
    }
}
