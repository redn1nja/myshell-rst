use std::process::exit;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use std::process::Command;


fn main() -> Result<()> {

    let mut rl = DefaultEditor::new()?;
    loop{
        let line = parse_input(&mut rl);
        let command = line.split_whitespace().collect::<Vec<&str>>();
        Command::new(command[0]).args(&command[1..]).spawn()?.wait().expect("Fork failed");
    }
}



fn parse_input(rl: &mut DefaultEditor) -> String {
    let mut prompt = std::env::current_dir().unwrap().display().to_string();
    prompt.push_str(" $ ");
    let readline = match rl.readline(prompt.as_str()) {
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
    };
    readline.trim().to_string()
}

