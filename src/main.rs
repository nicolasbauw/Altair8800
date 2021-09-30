use std::{ env, error::Error, process, thread, sync::mpsc, io::stdout, io::Write, fs, time };
use intel8080::*;
use console::{Term, Key, style};

fn main() {
    if let Err(e) = load_execute() {
        println!("{}", e);
        process::exit(1);
    }
}

fn load_execute() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let term = Term::stdout();
    let  mut a = env::args();
    let mut c = CPU::new();

    // Loads assembled program into memory
    if let Some(f) = a.nth(1) {
        c.bus.load_bin(&f, 0x0)?;
    } else {
        println!("No file specified");
        process::exit(1);
    }

    // Setting up Altair switches for 88-SIO (4K BASIC 3.2)
    c.bus.set_io_in(255, 0x00);

    // Since the console crate read key function is blocking, we spawn a thread
    thread::spawn(move || {
        loop {
            if let Some(ch) = getch(&term, &tx) {
                tx.send(ch).unwrap()
            }
        } 
    });

    loop {
        #[cfg(windows)]
        spin_sleep::sleep(time::Duration::from_nanos((c.execute() as u64) * 500));

        #[cfg(not(windows))]
        thread::sleep(time::Duration::from_nanos((c.execute() as u64) * 500));
        if c.pc == 0xffff { break };

        if let Ok(ch) = rx.try_recv() {
            c.bus.set_io_in(0, 0);
            c.bus.set_io_in(1, ch);
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

fn getch(term: &console::Term, tx: &std::sync::mpsc::Sender<u8>) -> Option<u8> {
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

fn toggle_menu(term: &console::Term, tx: &std::sync::mpsc::Sender<u8>) -> Result<(), Box<dyn Error>> {
    //term.hide_cursor().unwrap();
    let delay = time::Duration::from_millis(50);
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
                    thread::sleep(delay);
                }
                return Ok(());
            }
            _ => {}
        }
    }
}