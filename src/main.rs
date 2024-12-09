mod builtins;

use std::process::exit;
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Helper, Result};

use std::process::Command;
use rustyline::history::DefaultHistory;

fn main() -> Result<()> {
    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .auto_add_history(true)
        .completion_type(CompletionType::List)
        .build();
    let mut rl = rustyline::Editor::with_config(config)?;
    loop{
        let line = parse_input(&mut rl);
        let command = line.split_whitespace().collect::<Vec<&str>>();
        match command.len() {
            0 => continue,
            _ => match execute(command){
                Ok(_) => continue,
                Err(e) => println!("Error: {}", e)
            },
        }
    }
}

fn execute(command: Vec<&str>) -> Result<()>{
    match command[0] {
        "mcd" => {match builtins::builtins::mcd(command){
            Ok(result) => Ok(result),
            Err(e) => Err(ReadlineError::Io(e))
        }}
        _ => {
            match Command::new(command[0]).args(&command[1..]).spawn() {
                Ok(mut child) => {
                    let _ = child.wait().expect("");
                    Ok(())
                }
                Err(e) => Err(ReadlineError::Io(e))
            }
        }
    }
}

fn parse_input(rl: &mut rustyline::Editor<(), DefaultHistory>) -> String {
    let mut prompt = std::env::current_dir().unwrap().display().to_string();
    prompt.push_str(" $ ");
    match rl.readline(prompt.as_str()) {
        Ok(line) => {
            rl.add_history_entry(line.as_str()).expect("");
            line
        },
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            exit(1);
        },
        Err(ReadlineError::Interrupted) => {
            "".to_string()
        }
        Err(err) => {
            println!("Error: {:?}", err);
            exit(1);
        }
    }.trim().to_string()
}

