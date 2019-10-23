use std::io::{self, BufRead};
mod game;

fn main() -> Result<(), io::Error> {
    println!("👋");
    let stdin = io::stdin();
    let mut commands:Vec<&'static str> = Vec::new();
    for command in stdin.lock().lines() {    
        commands.push(Box::leak(command?.into_boxed_str()));
        let result = game::turn(&commands);
        println!("{}", result);
    }
    Ok(())    
}
