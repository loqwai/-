#![feature(try_trait)]
use std::io::{self, BufRead};
mod game;

fn main() -> Result<(), io::Error> {
    println!("👋");
    let stdin = io::stdin();
    let mut commands:Vec<String> = Vec::new();
    for command in stdin.lock().lines() {    
        commands.push(command?);
        let result = game::turn(&commands);
        match result {
            Ok(state) => println!("{}", state),
            Err(_) => println!("{}", "you fucked up"),
        }
    }
    Ok(())    
}
