use std::{ env, error::Error, process, thread, sync::mpsc, io::stdout, io::Write };
use intel8080::*;
use console::{Term, Key};

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let term = Term::stdout();
    let  a: Vec<String> = env::args().collect();
    let mut c = CPU::new();
    
    // Loads assembled program into memory
    c.bus.load_bin(&a[1], 0x0)?;

    // Setting up Altair switches for 88-SIO (4K BASIC 3.2)
    c.bus.set_io_in(255, 0x00);

    // Since the console crate read key function is blocking, we spawn a thread
    thread::spawn(move || {
        loop {
            if let Some(ch) = getch(&term) {
                tx.send(ch).unwrap()
            }
        } 
    });

    loop {
        c.execute();
        if c.pc == 0xffff { break };

        if let Ok(ch) = rx.try_recv() {
            c.bus.set_io_in(0, 0);
            c.bus.set_io_in(1, ch as u8);
        }
    
        
        // Data sent to device 1 (OUT) ? we display it
        if let Some(v) = c.bus.get_io_out(1) {
            let value = v & 0x7f;
            if value >= 32 && value <=125 || value == 0x0a || value == 0x0d {
                print!("{}", value as char);
                stdout().flush()?;
                // Clearing IO (in and out) to be ready for next key press
                c.bus.clear_io_out();
                c.bus.set_io_in(0, 1);
            }
        }
    }
    Ok(())
}

fn getch(term: &console::Term) -> Option<char> {
    match term.read_key() {
        Ok(k) => match k {
            Key::Char(c) => Some(c),
            Key::Enter => Some(0x0d as char),
            _ => None
        },
        Err(_) => None
    }
}
