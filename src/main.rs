use machine::{Machine, MachineError};
use std::process::ExitCode;

mod config;
mod machine;
mod snapshot;
mod teletype;

fn main() -> ExitCode {
    if run().is_err() {
        return ExitCode::from(1);
    }
    ExitCode::from(0)
}

fn run() -> Result<(), MachineError> {
    let mut m = Machine::new()?;
    m.run()?;
    Ok(())
}
