use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process;

#[derive(Debug)]
struct BurnError(&'static str);

impl fmt::Display for BurnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Error for BurnError {
    fn description(&self) -> &str {
        self.0
    }
}

type Result<T> = std::result::Result<T, BurnError>;

fn main() {
    match try_main() {
        Ok(()) => process::exit(0),
        Err(error) => {
            println!("{}", error.description());
            process::exit(1)
        }
    }
}

fn try_main() -> Result<()> {
    let filepath = env::args().nth(1);

    match filepath {
        Some(path) => {
            println!("noice, you want to burn {:?}", path);
            match fs::File::open(path) {
                Ok(ref mut file) => {
                    println!("found an existing link");
                    Ok(())
                }
                Err(_) => {
                    Err(BurnError("failed to open file"))
                }
            }
        }
        None => {
            Err(BurnError("`burn` should be called with one path to a file to be burnt."))
        }
    }
}

fn check_argc() -> Result<()> {
    match env::args().len() {
        1 => Ok(()),
        _ => Err(BurnError("`burn` should be called with a single filepath."))
    }
}

// fn get_valid_filepath() ->
