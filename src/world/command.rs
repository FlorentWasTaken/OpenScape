/*
** OpenScape 2023
** command.rs
** File description:
** command.rs
*/

use std::io::{self, Write};
use std::collections::HashSet;
use std::thread;
use crate::world::create_block;
use crate::world::remove_block;

fn place_func(input: &str) {
    let mut iter = input.split_whitespace().skip(1);

    if let (Some(p1), Some(p2)) = (iter.next(), iter.next()) {
        if let Ok(x) = p1.parse::<i32>() {
            if let Ok(y) = p2.parse::<i32>() {
                create_block(x, y);
            }
        }
    } else {
        println!("Place needs 2 parameters");
    }
}

fn remove_func(input: &str) {
    let mut iter = input.split_whitespace().skip(1);

    if let (Some(p1), Some(p2)) = (iter.next(), iter.next()) {
        if let Ok(x) = p1.parse::<i32>() {
            if let Ok(y) = p2.parse::<i32>() {
                remove_block(x, y);
            }
        }
    } else {
        println!("Remove needs 2 parameters");
    }
}

pub fn listen_commands() {
    thread::spawn(|| {
        let valid_commands: HashSet<&str> = ["place", "remove"].iter().cloned().collect();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unvalid command");

            if let Some(command) = input.split_whitespace().next() {
                if valid_commands.contains(command) {
                    match command {
                        "place" => place_func(&input),
                        "remove" => remove_func(&input),
                        _ => {
                            println!("Invalid command : {}", command);
                        }
                    }
                }
            }
        }

    });
}
