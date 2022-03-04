use std::io;

mod domain;

use crate::domain::{Game, Object};

fn main() {
    let mut game = Game::new();

    println!("{}\n", game.look());

    loop {
        println!("{}\n", cmd(read_command(), &mut game));
    }
}

fn read_command() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf
}

fn cmd(line: String, game: &mut Game) -> String {
    let mut sw = line.split_whitespace();

    match sw.next() {
        Some("look") => game.look(),
        Some("walk") => match sw.next() {
            Some(direction) => game.walk(direction),
            None => String::from("walk command needs a arg for direction"),
        },
        Some("pickup") => match sw.next() {
            Some(s) => {
                if let Some(obj) = Object::from_str(s) {
                    game.pickup(obj)
                } else {
                    format!("{} is not object", s)
                }
            }
            None => String::from("pickup command needs a arg for direction"),
        },
        Some("inventory") => game.inventory(),
        Some(command) => format!("{} is not command", command),
        None => String::from("please type command"),
    }
}
