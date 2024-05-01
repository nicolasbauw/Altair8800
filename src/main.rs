use std::process;

mod config;
mod machine;
mod teletype;

use machine::Machine;

fn main() {
    let mut m = Machine::new();
    if let Err(e) = m.run() {
        println!("{}", e);
        process::exit(1);
    }
}
