use std::process::{Command};
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use crate::{builtins, rl};

pub fn main_loop(mut rl: &mut rustyline::Editor<rl::MyHelper, DefaultHistory>) {
    loop{
        let line = rl::parse_input(&mut rl);
        let command = line.split_whitespace().collect::<Vec<&str>>();
        exec(command);
    }
}

fn execute(command: Vec<&str>) -> rustyline::Result<()> {
    match command[0] {
        "mcd" => {match builtins::builtins::mcd(command){
            Ok(result) => Ok(result),
            Err(e) => Err(ReadlineError::Io(e))
        }},
        "mecho" => {match builtins::builtins::mecho(command){
            Ok(result) => Ok(result),
            Err(e) => Err(ReadlineError::Io(e))
        }}
        "mexit" => {match builtins::builtins::mexit(command) {
            Ok(result) => Ok(result),
            Err(e) => Err(ReadlineError::Io(e))
        }}
        _ => {
            create_child(command)
        }
    }
}

fn create_child(command: Vec<&str>) -> rustyline::Result<()> {
    match Command::new(command[0]).args(&command[1..]).spawn() {
        Ok(mut child) => {
            let _ = child.wait().expect("");
            Ok(())
        }
        Err(e) => Err(ReadlineError::Io(e))
    }
}

fn exec(command: Vec<&str>) {
    match command.len() {
        0 => return,
        _ => match execute(command){
            Ok(_) => return,
            Err(e) => println!("Error: {}", e)
        },
    }
}