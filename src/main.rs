extern crate rand;
extern crate termion;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;

use termion::raw::IntoRawMode;

mod layers;
mod state;
mod ui;

use layers::{BasicLayer, Compositor, Layerable};
use state::{FireState, FireLayer};
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
    let stdout = stdout.lock(); //.into_raw_mode().unwrap();

    let filepath = get_filepath()?;

    match fs::File::open(filepath) {
        Ok(file) => {
            let (term_cols, term_rows) = termion::terminal_size().expect("could not read terminal size");
            let (term_cols, term_rows) = ((term_cols - 2) as usize, (term_rows - 2) as usize);

            let filebuf = io::BufReader::new(file);

            let file_lines: Vec<Vec<u8>> = filebuf.lines()
                .take(term_rows)
                .map(|maybe_line| maybe_line.map(|line| line.into_bytes()).unwrap())
                .collect();

            let compositor = Compositor {
                rows: term_rows,
                cols: term_cols,
            };

            let base_layer = BasicLayer::create(
                term_rows,
                term_cols,
                file_lines.into_iter().map(|row| row.into_iter().map(|byte| format!("{}", byte as char).into()).collect()).collect(),
            );

            let mut ui = Ui::create(stdout);
            let mut state = state::FireState::new(term_rows, term_cols);

            state.start_fire();

            // TODO: yuck. Make this expression nicer, maybe allow composing the compositor into the ui from the get-go
            ui.draw(&compositor.composite(&[&base_layer, &(state.as_layer())]));

            while !state.is_saturated() {
                state = state.get_next();
                ui.draw(&compositor.composite(&[&base_layer, &(state.as_layer())]));
                // TODO: do this better and configure duration with a more obvious constant
                std::thread::sleep(std::time::Duration::from_millis(80));
            }

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
