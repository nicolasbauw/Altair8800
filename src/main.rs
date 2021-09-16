use std::{ env, error::Error, process };
use intel8080::*;
use console::{Term, Key};

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let term = Term::stdout();
    let  a: Vec<String> = env::args().collect();
    let mut c = CPU::new();
    // Loads assembled program into memory
    c.bus.load_bin(&a[1], 0x0)?;

    // Setting up Altair switches for 88-SIO (4K BASIC 3.2)
    c.bus.set_io_in(255, 0x00);

    loop {
        //c.debug = true;
        c.execute();
        //if c.pc == 0xffff { break };

        match getch(&term) {
            Some(ch) => { c.bus.set_io_in(0, 0); c.bus.set_io_in(1, ch as u8) },
            _ => {}
            }
        

        if c.bus.get_io_out(1).is_some() {
            let mut value = c.bus.get_io_out(1).unwrap();
            value = value & 0x7f;
            if value >= 32 && value <=125 || value == '\n' as u8 {
                print!("{}", value as char);
                c.bus.clear_io_out();
            }
        }
    }
    
    Ok(())
}

fn getch(term: &console::Term) -> Option<char> {
    match term.read_key().unwrap() {
        Key::Char(c) => Some(c),
        _ => None
    }
}
