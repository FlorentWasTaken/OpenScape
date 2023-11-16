/*
** OpenScape 2023
** world.rs
** File description:
** world.rs
*/

pub mod square;
use self::square::Square;

pub fn init_world<'a>() -> Vec<Vec<Option<Square<'a>>>> {
    let mut vec: Vec<Vec<Option<Square<'a>>>> = Vec::new();

    for _ in 1..100 {
        let mut row: Vec<Option<Square<'a>>> = Vec::new();

        for _ in 1..100 {
            row.push(None);
        }
        vec.push(row);
    }
    vec
}
