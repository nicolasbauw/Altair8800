use crate::{config, MachineError};
use console::{style, Key, Term};
use std::{fs, thread};

pub struct Teletype {
    pub control: u8, // Device 0
    pub data: u8,    // Device 1
}

pub enum ConsoleMsg {
    Char(u8),
    LoadSnap,
    SaveSnap,
    ResetCpu,
    Quit,
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
        term.show_cursor().unwrap();
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
                    None
                }
                _ => None,
            },
            Err(_) => None,
        }
    }

    pub fn toggle_menu(
        term: &console::Term,
        tx: &std::sync::mpsc::Sender<ConsoleMsg>,
    ) -> Result<(), MachineError> {
        let config = config::load_config_file()?;
        let term_geometry = term.size();        // (rows, columns)
        term.hide_cursor()?;
        term.move_cursor_to(0, 0)?;
        term.clear_line()?;
        if term_geometry.1 < 80 {
            println!("Terminal < 80 columns ! Press ESC");
        } else {
            println!(
                "{}uit {}eset {}uto typing {}ave Snapshot {}oad Snapshot {}Toggle menu",
                style("[Q]").magenta(),
                style("[R]").magenta(),
                style("[A]").magenta(),
                style("[S]").magenta(),
                style("[L]").magenta(),
                style("[ESC]").magenta(),
            );
        }
        loop {
            match term.read_key()? {
                Key::Escape => {
                    term.move_cursor_to(0, 0)?;
                    term.clear_line()?;
                    term.move_cursor_to(0, 255)?;
                    term.show_cursor()?;
                    return Ok(());
                }
                Key::Char('Q') => {
                    term.clear_screen()?;
                    term.show_cursor()?;
                    tx.send(ConsoleMsg::Quit)?;
                }
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
                    term.show_cursor()?;
                    return Ok(());
                }
                Key::Char('S') => {
                    term.move_cursor_to(0, 255)?;
                    term.show_cursor()?;
                    tx.send(ConsoleMsg::SaveSnap)?;
                    return Ok(());
                }
                Key::Char('L') => {
                    term.move_cursor_to(0, 255)?;
                    term.show_cursor()?;
                    tx.send(ConsoleMsg::LoadSnap)?;
                    return Ok(());
                }
                Key::Char('R') => {
                    tx.send(ConsoleMsg::ResetCpu)?;
                    term.clear_screen()?;
                    term.show_cursor()?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
