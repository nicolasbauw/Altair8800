use std::{error::Error, process::ExitCode};

mod config;
mod machine;
mod snapshot;
mod teletype;

use machine::Machine;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::from(0),
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut m = Machine::new()?;
    m.run()?;
    Ok(())
}
