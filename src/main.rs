extern crate termion;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::io::prelude::*;
use std::process;

use termion::{clear, cursor};
use termion::raw::IntoRawMode;

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
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", clear::All).unwrap();

    let filepath = get_filepath()?;

    println!("noice, you want to burn {:?}", filepath);
    match fs::File::open(filepath) {
        Ok(mut file) => {
            // println!("found an existing link");
            // TODO: read x bytes of file where x = (term height • term width)
            let termsize = termion::terminal_size().expect("could not read terminal size");
            let bytes_to_read = (termsize.0 * termsize.1) as usize;

            let mut buf: Vec<u8> = vec![0; bytes_to_read];

            file.read(&mut buf).expect("failed to read file contents");

            stdout.write(&buf).expect("failed to write to stdout");
            stdout.write(b"wrote file contents").unwrap();
            stdout.flush().unwrap();

            // FIXME
            std::thread::sleep(std::time::Duration::from_millis(1000));

            Ok(())
        }
        Err(_) => {
            Err(BurnError("failed to open file"))
        }
    }
}

fn get_filepath() -> Result<String> {
    let mut args = env::args();

    match args.len() {
        2 => Ok(args.nth(1).unwrap()),
        _ => Err(BurnError("`burn` should be called with a single filepath."))
    }
}

// TODO: this doesn't actually check whether the file can be unlinked in UNIX.
// either remove this altogether and rely on OS exception or read parent dir permissions too
fn check_can_unlink_file(file: fs::File) -> Result<()> {
    let metadata = file.metadata().expect("failed to read file metadata");

    if metadata.permissions().readonly() {
        Err(BurnError("file lacks write permission"))
    } else {
        Ok(())
    }
}
