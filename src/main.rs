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

    thread::spawn(move || {
        loop {match getch(&term) {
            Some(ch) => tx.send(ch).unwrap(),
            _ => {}
        }}
            
        }
    );

    loop {
        //c.debug = true;
        c.execute();
        if c.pc == 0xffff { break };

        match rx.try_recv() {
            // key pressed ? control device sends (0), and the pressed key is sent by I/O device (1), that's an IN for the CPU
            Ok(ch) => { c.bus.set_io_in(0, 0); c.bus.set_io_in(1, ch as u8) },
            _ => {}
        };
        
        // Data sent to device 1 (OUT) ? we display it
        if c.bus.get_io_out(1).is_some() {
            let mut value = c.bus.get_io_out(1).unwrap();
            value = value & 0x7f;
            if value >= 32 && value <=125 || value == '\n' as u8 {
                print!("{}", value as char);
                stdout().flush().unwrap();
                c.bus.clear_io_out();
                c.bus.set_io_in(0, 1);
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
