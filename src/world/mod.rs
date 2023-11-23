/*
** OpenScape 2023
** world.rs
** File description:
** world.rs
*/

pub mod square;
pub mod script;
use self::square::Square;
use crate::GLOBAL_VECT;

const CUBE_SIZE: i32 = 50;

pub fn init_world<'a>() -> Vec<Vec<Option<Square>>> {
    let mut vec: Vec<Vec<Option<Square>>> = Vec::new();

    for _ in 1..100 {
        let mut row: Vec<Option<Square>> = Vec::new();

        for _ in 1..100 {
            row.push(None);
        }
        vec.push(row);
    }
    vec
}

pub fn create_block<'a>(x: i32, y: i32) {
    let mut vect = GLOBAL_VECT.lock().unwrap();

    if let Some(val) = vect.get_mut((y / CUBE_SIZE) as usize) {
        if let Some(bloc) = val.get_mut((x / CUBE_SIZE) as usize) {
            *bloc = Some(Square::new(x, y));
            println!("{x} {y}");
        }
    }
}
