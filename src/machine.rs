use console::Term;
use intel8080::CPU;
use std::{
    env, error::Error, io::stdout, io::Write, process, sync::mpsc, thread, time::Duration,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let term = Term::stdout();
    let mut a = env::args();
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

    let mut teletype = crate::teletype::Teletype::new();

    // Since the console crate read key function is blocking, we spawn a thread
    thread::spawn(move || loop {
        if let Some(ch) = crate::teletype::getch(&term, &tx) {
            tx.send(ch).unwrap()
        }
    });

    loop {
        // Checking if data was sent from the "teletype" thread
        if let Ok(ch) = rx.try_recv() {
            // A key has been pressed ? device 0 sends 0 (output device ready)
            teletype.control = 0;
            // Then, device 1 sends the key code
            teletype.data = ch;
        }

        // Will likely never happen. There just to meet function return type requirement.
        if c.pc == 0xffff {
            return Ok(());
        };

        let pc = c.pc;
        let opcode = c.bus.read_byte(pc);
        match opcode {
            // IN
            0xdb => {
                let port = c.bus.read_byte(pc + 1);
                c.reg.a = match port {
                    0xFF => 0, // Altair switches set up for 88-SIO (4K BASIC 3.2)
                    0x00 => teletype.control,
                    0x01 => teletype.data,
                    _ => c.reg.a,
                }
            }
            // OUT
            0xd3 => {
                let port = c.bus.read_byte(pc + 1);
                if port == 0x01 {
                    let value = c.reg.a & 0x7f;
                    if value >= 32 && value <= 125 || value == 0x0a || value == 0x0d {
                        print!("{}", value as char);
                        stdout().flush()?;
                    }
                }
                // Reset teletype control register
                teletype.control = 1;
            }
            _ => {}
        }

        if let Some(t) = c.execute_timed() {
            thread::sleep(Duration::from_millis(t.into()))
        }
    }
}
