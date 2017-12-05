use std::env;
use std::process;

fn main() {
    let filepath = env::args().nth(1);

    match filepath {
        Some(path) => println!("noice, you want to burn {:?}", path),
        None => {
            println!("`burn` should be called with one path to a file to be burnt.");
            process::exit(1);
        }
    }
}
