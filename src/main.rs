use std::{ env, error::Error, process, thread, time ,time::Duration, io::Write, io::stdout};
use zilog_z80::cpu::CPU;
use console::{Term, Key};

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = zilog_z80::crossbeam_channel::bounded::<u8>(1);
    let term = Term::stdout();
    let  mut a = env::args();
    let mut c = CPU::new(0xFFFF);
    //c.set_freq(0.0000000250);
    /* This byte of ROM at the end of address space is there to meet basic 3.2 initialization code requirement
    otherwise automatic RAM detection routine loops forever */
    c.bus.set_romspace(0xffff, 0xffff);
    //c.debug.io = true;
    //c.debug.instr_in = true;

    // Loads assembled program into memory
    if let Some(f) = a.nth(1) {
        c.bus.load_bin(&f, 0x0)?;
    } else {
        println!("No file specified");
        process::exit(1);
    }

    let device0_req_receiver = c.bus.io_req.1.clone();
    //let device0_req_receiver2 = c.bus.io_req.1.clone();
    let device0_sender = c.bus.io_in.0.clone();
    //let device0_sender1 = c.bus.io_in.0.clone();

    let altair_switches_req_receiver = c.bus.io_req.1.clone();
    let altair_switches_sender = c.bus.io_in.0.clone();

    let device1_sender = c.bus.io_in.0.clone();
    let device1_receiver = c.bus.io_out.1.clone();

    // Setting up Altair switches for 88-SIO (4K BASIC 3.2)
    //c.bus.set_io_in(255, 0x00);

    // Device 0 : teletype control channel
    thread::spawn(move || {
        loop {
            // CPU ran an IN instruction ?
            if let Ok(device) = device0_req_receiver.recv() {
                // IN for device 0 ?
                if device == 0 {
                    // No key has been pressed ? we send 1 to device 0
                    match rx.try_recv() {
                        Err(_) => {
                            device0_sender.send((0,1)).unwrap();
                            //println!("No key pressed, device 0 sends 1");
                        },
                        // A key has been pressed ? we send 0 (output device ready) to device 0
                        // Then, we send the key code to device 1
                        Ok(ch) => {
                            device0_sender.send((0,0)).unwrap();
                            device1_sender.send((1,ch)).unwrap();
                            //println!("Key pressed, device 0 sends 0");
                        }
                    }
                    
                }
            }
        }
    });

    // Device 1 : send and receive ASCII data from the teletype
    thread::spawn(move || {
        // Device 1 received data ? Let's print it
        loop {
            if let Ok((device, data)) = device1_receiver.recv() {
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
                altair_switches_sender.send((255, 0)).unwrap();
            }
        }
    });

    // Since the console crate read key function is blocking, we spawn a thread
    thread::spawn(move || {
        loop {
            if let Some(ch) = getch(&term) {
                tx.send(ch).unwrap()
            }
        } 
    });

    // CPU loop
    loop {
        c.execute_slice();
    }

}

fn getch(term: &console::Term) -> Option<u8> {
    match term.read_key() {
        Ok(k) => match k {
            Key::Char(c) => Some(c as u8),
            Key::Enter => Some(0x0d),
            _ => None
        },
        Err(_) => None
    }
}
