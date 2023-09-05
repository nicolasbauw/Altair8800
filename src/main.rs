use std::process;

mod config;
mod machine;
mod teletype;

fn main() {
    if let Err(e) = machine::run() {
        println!("{}", e);
        process::exit(1);
    }
}
