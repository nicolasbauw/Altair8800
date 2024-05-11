use crate::config;
use console::{style, Key, Term};
use std::{error::Error, fs, process, thread};

pub struct Teletype {
    pub control: u8, // Device 0
    pub data: u8,    // Device 1
}

pub enum ConsoleMsg {
    Char(u8),
    LoadSnap,
    SaveSnap,
}

pub struct Console {}

impl Teletype {
    pub fn new() -> Self {
        Self {
            control: 0,
            data: 0,
        }
    }
}

impl Console {
    pub fn spawn(tx: std::sync::mpsc::Sender<ConsoleMsg>) {
        let term = Term::stdout();
        term.clear_screen().unwrap();
        // Since the console crate read key function is blocking, we spawn a thread
        thread::spawn(move || loop {
            if let Some(ch) = Console::getch(&term, &tx) {
                tx.send(ConsoleMsg::Char(ch)).unwrap()
            }
        });
    }

    pub fn getch(term: &console::Term, tx: &std::sync::mpsc::Sender<ConsoleMsg>) -> Option<u8> {
        match term.read_key() {
            Ok(k) => match k {
                Key::Char(c) => Some(c as u8),
                Key::Enter => Some(0x0d),
                Key::Escape => {
                    if let Err(e) = Console::toggle_menu(term, tx) {
                        println!("{}", e)
                    };
                    return None;
                }
                _ => None,
            },
            Err(_) => None,
        }
    }

    pub fn toggle_menu(
        term: &console::Term,
        tx: &std::sync::mpsc::Sender<ConsoleMsg>,
    ) -> Result<(), Box<dyn Error>> {
        let config = config::load_config_file()?;
        term.move_cursor_to(0, 0)?;
        term.clear_screen()?;
        println!(
            "{}uit\t{}uto typing\t{}ave Snapshot\t{}oad Snapshot\t{}Toggle menu",
            style("[Q]").magenta(),
            style("[A]").magenta(),
            style("[S]").magenta(),
            style("[L]").magenta(),
            style("[ESC]").magenta(),
        );
        loop {
            match term.read_key()? {
                Key::Escape => {
                    term.clear_screen().unwrap();
                    println!("Emulation resumed !");
                    thread::sleep(std::time::Duration::from_secs(1));
                    term.clear_screen().unwrap();
                    return Ok(());
                }
                Key::Char('Q') => process::exit(0),
                Key::Char('A') => {
                    term.clear_screen()?;
                    term.write_line("File ? ")?;
                    let file = term.read_line()?;
                    let bas = fs::read_to_string(file)?;
                    for line in bas.lines() {
                        for c in line.chars() {
                            tx.send(ConsoleMsg::Char(c as u8))?;
                            thread::sleep(std::time::Duration::from_millis(
                                config.keyboard.char_delay,
                            ));
                        }
                        tx.send(ConsoleMsg::Char(0x0d))?;
                        thread::sleep(std::time::Duration::from_millis(config.keyboard.line_delay));
                    }
                    return Ok(());
                }
                Key::Char('S') => {
                    tx.send(ConsoleMsg::SaveSnap)?;
                }
                Key::Char('L') => {
                    tx.send(ConsoleMsg::LoadSnap)?;
                }
                _ => {}
            }
        }
    }
}
