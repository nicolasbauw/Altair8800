use std::{ env, error::Error, process, thread, time, io::Write, io::stdout, fs};
use intel8080::CPU;
use console::{Term, Key, style};

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = intel8080::crossbeam_channel::bounded::<u8>(1);
    let term = Term::stdout();
    let  mut a = env::args();
    let mut c = CPU::new();

    /* This byte of ROM at the end of address space is there to meet basic 3.2 initialization code requirement
    otherwise automatic RAM detection routine loops forever */
    c.bus.set_romspace(0xffff, 0xffff);

    // Loads assembled program into memory
    if let Some(f) = a.nth(1) {
        c.bus.load_bin(&f, 0x0)?;
    } else {
        println!("No file specified");
        process::exit(1);
    }

    let device0_req_receiver = c.bus.io_req.1.clone();
    let device0_sender = c.bus.io_in.0.clone();

    let altair_switches_req_receiver = c.bus.io_req.1.clone();
    let altair_switches_sender = c.bus.io_in.0.clone();

    let device1_sender = c.bus.io_in.0.clone();
    let device1_receiver = c.bus.io_out.1.clone();

    // Device 0 : teletype control channel
    thread::spawn(move || {
        loop {
            // CPU ran an IN instruction ?
            if let Ok(device) = device0_req_receiver.recv() {
                // IN for device 0 ?
                if device == 0 {
                    match rx.try_recv() {
                        // No key have been pressed ? device 0 sends 1
                        Err(_) => {
                            device0_sender.send((0,1)).unwrap();
                        },
                        // A key has been pressed ? device 0 sends 0 (output device ready)
                        // Then, device 1 sends the key code
                        Ok(ch) => {
                            device0_sender.send((0,0)).unwrap();
                            device1_sender.send((1,ch)).unwrap();
                        }
                    }
                }
            }
        }
    });

    // Device 1 : send and receive ASCII data from the teletype
    thread::spawn(move || {
        loop {
            if let Ok((device, data)) = device1_receiver.recv() {
                // Device 1 received data ? Let's print it
                if device == 1 {
                    let value = data & 0x7f;
                    if value >= 32 && value <=125 || value == 0x0a || value == 0x0d {
                        print!("{}", value as char);
                        stdout().flush().unwrap();
                    }
                }
            }
        }
    });

    // Device 255 : Altair switches
    thread::spawn(move || {
        if let Ok(device) = altair_switches_req_receiver.recv() {
            if device == 255 {
                // All switches down : that's the 88-SIO setting for MS Basic 3.2
                altair_switches_sender.send((255, 0)).unwrap();
            }
        }
    });

    // Since the console crate read key function is blocking, we spawn a thread
    thread::spawn(move || {
        loop {
            if let Some(ch) = getch(&term, &tx) {
                tx.send(ch).unwrap()
            }
        } 
    });

    // CPU loop
    loop {
        c.execute_slice();
    }

}

fn getch(term: &console::Term, tx: &intel8080::crossbeam_channel::Sender<u8>) -> Option<u8> {
    match term.read_key() {
        Ok(k) => match k {
            Key::Char(c) => Some(c as u8),
            Key::Enter => Some(0x0d),
            Key::Escape => {
                if let Err(e) = toggle_menu(term, tx) { println!("{}", e) };
                return None
            },
            _ => None
        },
        Err(_) => None
    }
}

fn toggle_menu(term: &console::Term, tx: &intel8080::crossbeam_channel::Sender<u8>) -> Result<(), Box<dyn Error>> {
    let delay = time::Duration::from_millis(25);
    term.move_cursor_to(0, 0)?;
    term.clear_screen().unwrap();
    println!("{}uit\t{}oad", style("[Q]").magenta(), style("[L]").magenta());
    loop {
        match term.read_key()? {
            Key::Escape => { term.clear_screen().unwrap(); return Ok(())},
            Key::Char('Q') => { process::exit(0) },
            Key::Char('L') => {
                term.clear_screen()?;
                term.write_line("File ? ")?;
                let file = term.read_line()?;
                let bas = fs::read_to_string(file)?;
                for line in bas.lines() {
                    for c in line.chars() {
                        tx.send(c as u8)?;
                        thread::sleep(delay);
                    }
                    tx.send(0x0d)?;
                    thread::sleep(delay*10);
                }
                return Ok(());
            }
            Key::Char('C') => {
                tx.send(0x03)?;
            }
            _ => {}
        }
    }
}