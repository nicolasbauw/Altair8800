use std::process::ExitCode;
use machine::{Machine, MachineError};

mod config;
mod machine;
mod snapshot;
mod teletype;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::from(0),
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), MachineError> {
    let mut m = Machine::new()?;
    m.run()?;
    Ok(())
}
