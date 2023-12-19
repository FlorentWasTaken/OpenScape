/*
** OpenScape 2023
** world.rs
** File description:
** world.rs
*/

pub mod square;
pub mod script;
pub mod command;
use self::square::Square;
use std::fs::File;
use std::io::{self, BufRead, Write};
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

fn reset_world() {
    let mut vect = GLOBAL_VECT.lock().unwrap();

    for row in vect.iter_mut() {
        row.clear();
    }
}

pub fn load_world(file_path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut result = Vec::new();
    let mut x = 0;
    let mut y = 0;

    //reset_world();
    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        for character in line.chars() {
            if character == '1' {
                create_block(x * CUBE_SIZE, y * CUBE_SIZE);
            }
            x += 1;
        }
        x = 0;
        y += 1;
        result.push(tokens);
    }
    Ok(result)
}

pub fn save_world(file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let vect = GLOBAL_VECT.lock().unwrap();

    //reset_world();
    for row in vect.iter() {
        for square_option in row {
            match square_option {
                None => write!(file, "0")?,
                Some(_) => write!(file, "1")?,
            }
        }
        writeln!(file)?;
    }

    Ok(())
}
