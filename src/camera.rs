/*
** OpenScape 2023
** camera.rs
** File description:
** camera.rs
*/

use crate::GLOBAL_CAMERA;

pub struct Camera {
    pub x: i32,
    pub y: i32,
}

impl Camera {
    pub fn new(x_pos: i32, y_pos: i32) -> Camera {
        Camera {x: x_pos, y: y_pos}
    }
}

pub fn get_camera_pos() -> (i32, i32) {
    let camera = GLOBAL_CAMERA.lock().unwrap();

    (camera.x, camera.y)
}

pub fn set_camera_pos(x: i32, y: i32) {
    let mut camera = GLOBAL_CAMERA.lock().unwrap();

    camera.x = x;
    camera.y = y;
}
