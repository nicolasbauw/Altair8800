use std::process;

mod config;
mod console;
mod teletype;

fn main() {
    if let Err(e) = teletype::run() {
        println!("{}", e);
        process::exit(1);
    }
}
