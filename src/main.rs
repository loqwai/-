use std::io::{self, Read, BufRead};
mod lib;

fn main() -> Result<(), io::Error> {
    println!("👋");
    let stdin = io::stdin();
    let mut commands:Vec<String> = Vec::new();
    for command in stdin.lock().lines() {
        // let cmd = command?.as_str();       
        commands.push(command?);
        let result = lib::eval(&commands);
        println!("{}", result);
    }
    Ok(())
    // loop {
    //     let _command = next_command()?;
    //     let result = lib::eval(command.split("\n"));
    //     // println!("{}", result);
    // }
}
