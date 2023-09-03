use std::process;

pub mod config;
mod teletype;

fn main() {
    if let Err(e) = teletype::run() {
        println!("{}", e);
        process::exit(1);
    }
}
