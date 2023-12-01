/*
** OpenScape 2023
** world.rs
** File description:
** world.rs
*/

pub mod square;
pub mod script;
use self::square::Square;
use crate::{GLOBAL_VECT, GLOBAL_CAMERA};

const CUBE_SIZE: i32 = 50;

pub fn init_world<'a>(size: u32) -> Vec<Vec<Option<Square>>> {
    let mut vec: Vec<Vec<Option<Square>>> = Vec::new();

    for _ in 0..size {
        let mut row: Vec<Option<Square>> = Vec::new();

        for _ in 0..size {
            row.push(None);
        }
        vec.push(row);
    }
    vec
}

pub fn create_block<'a>(x: i32, y: i32) {
    let mut vect = GLOBAL_VECT.lock().unwrap();
    let camera = GLOBAL_CAMERA.lock().unwrap();

    if let Some(val) = vect.get_mut((y / CUBE_SIZE + (camera.y * -1) / CUBE_SIZE) as usize) {
        if let Some(bloc) = val.get_mut((x / CUBE_SIZE + (camera.x * -1) / CUBE_SIZE) as usize) {
            *bloc = Some(Square::new(x + ((camera.x * -1) / CUBE_SIZE) * CUBE_SIZE, y + ((camera.y * -1) / CUBE_SIZE) * CUBE_SIZE));
        }
    }
}

pub fn remove_block<'a>(x: i32, y: i32) {
    let camera = GLOBAL_CAMERA.lock().unwrap();
    let mut vect = GLOBAL_VECT.lock().unwrap();

    if let Some(val) = vect.get_mut((y / CUBE_SIZE + (camera.y * -1) / CUBE_SIZE) as usize) {
        if let Some(bloc) = val.get_mut((x / CUBE_SIZE + (camera.x * -1) / CUBE_SIZE) as usize) {
            *bloc = None;
        }
    }
}
