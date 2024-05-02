use std::error::Error;

mod config;
mod machine;
mod teletype;

use machine::Machine;

fn main() -> Result<(), Box<dyn Error>> {
    let mut m = Machine::new()?;
    m.run()?;
    Ok(())
}
