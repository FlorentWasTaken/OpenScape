/*
** OpenScape 2023
** world.rs
** File description:
** world.rs
*/

pub mod square;
use self::square::Square;

pub fn init_world() -> Vec<Vec<Option<Square>>> {
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
