use crate::config;
use console::{style, Key};
use std::{
    error::Error, fs, process, thread,
};

pub fn getch(term: &console::Term, tx: &std::sync::mpsc::Sender<u8>) -> Option<u8> {
    match term.read_key() {
        Ok(k) => match k {
            Key::Char(c) => Some(c as u8),
            Key::Enter => Some(0x0d),
            Key::Escape => {
                if let Err(e) = toggle_menu(term, tx) {
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
    tx: &std::sync::mpsc::Sender<u8>,
) -> Result<(), Box<dyn Error>> {
    let config = config::load_config_file()?;
    term.move_cursor_to(0, 0)?;
    term.clear_screen().unwrap();
    println!(
        "{}uit\t{}oad",
        style("[Q]").magenta(),
        style("[L]").magenta()
    );
    loop {
        match term.read_key()? {
            Key::Escape => {
                term.clear_screen().unwrap();
                return Ok(());
            }
            Key::Char('Q') => process::exit(0),
            Key::Char('L') => {
                term.clear_screen()?;
                term.write_line("File ? ")?;
                let file = term.read_line()?;
                let bas = fs::read_to_string(file)?;
                for line in bas.lines() {
                    for c in line.chars() {
                        tx.send(c as u8)?;
                        thread::sleep(std::time::Duration::from_millis(config.keyboard.char_delay));
                    }
                    tx.send(0x0d)?;
                    thread::sleep(std::time::Duration::from_millis(config.keyboard.line_delay));
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