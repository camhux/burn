extern crate termion;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;

use termion::raw::IntoRawMode;

mod ui;

use ui::Ui;

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
    let stdout = stdout.lock().into_raw_mode().unwrap();

    let filepath = get_filepath()?;

    match fs::File::open(filepath) {
        Ok(file) => {
            let term_size = termion::terminal_size().expect("could not read terminal size");
            let term_rows = term_size.1;
            let grid_bytes = (term_size.0 * term_size.1) as usize;

            let filebuf = io::BufReader::new(file);

            let lines: Vec<Vec<u8>> = filebuf.lines()
                .take(term_rows as usize)
                .map(|maybe_line| maybe_line.map(|line| line.into_bytes()).unwrap())
                .collect();

            let mut ui = Ui::create(stdout, lines);

            ui.draw();

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
