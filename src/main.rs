use std::io::{self, Read};
mod lib;

fn main() -> Result<(), io::Error> {
    println!("👋");
    
    loop {
        let command = next_command()?;
        let result = lib::eval(command);
        println!("{}", result);
    }
}

fn next_command() -> io::Result<String> {
    let mut buffer = String::new();
    let _result = io::stdin().read_to_string(&mut buffer);
    return Ok(buffer);
}

