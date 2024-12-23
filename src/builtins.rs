pub mod builtins {
    use homedir;
    use std::path::{PathBuf};
    enum METHODS {
        MCD,
        MECHO,
        MEXIT

    }

    fn check_help<'a>(args: &'a Vec<&'a str>) -> Option<&'a&'a str> {
        args.iter().find(|&&arg| arg.eq("-h") || arg.eq("--help"))
    }
    fn get_home() -> Option<PathBuf> {
        homedir::my_home().unwrap_or(Some(PathBuf::from("/")))
    }

    fn help(method :METHODS) -> std::io::Result<()> {
        match method {
            METHODS::MCD => { println!("mcd <directory | -h | --help> : changes the directory");
                Ok(())
            }
            METHODS::MECHO => { println!("mcd <args | -h | --help> print the arguments");
                Ok(())
            }
            METHODS::MEXIT => {println!("mexit <code | -h | --help> : exits the myshell_rst with given exit code");
                Ok(())
            }
        }
    }

    pub fn mcd(args: Vec<&str>) -> std::io::Result<()> {
        match args.len() {
            1 => std::env::set_current_dir(get_home().unwrap()),
            2 => {
                match check_help(&args) {
                    Some(_) => {help(METHODS::MCD)}
                    None => {std::env::set_current_dir(args[1])
                    }
                }}
            _ => help(METHODS::MCD)
        }
    }

    pub fn mecho(args: Vec<&str>) -> std::io::Result<()> {
        match check_help(&args) {
            Some(_) => {help(METHODS::MECHO)},
            None => {println!("{}", args[1..].join(" "));
                Ok(())
            }
        }
    }

    pub fn mexit(args: Vec<&str>) -> std::io::Result<()> {
        match check_help(&args) {
            Some(_) => help(METHODS::MEXIT),
            None => {match args.len() {
                1 => std::process::exit(0),
                2 => match args[1].parse::<i32>() {
                    Ok(code) => std::process::exit(code),
                    Err(_) => help(METHODS::MEXIT)
                }
                _ => help(METHODS::MEXIT)
            }}
        }
    }
}