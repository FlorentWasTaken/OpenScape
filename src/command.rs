/*
** OpenScape 2023
** command.rs
** File description:
** command.rs
*/

use std::io::{self, Write};
use std::thread;

pub fn listen_commands() {
    thread::spawn(|| {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unvalid command");

            println!("Command : {}", input.trim());
        }

    });
}
