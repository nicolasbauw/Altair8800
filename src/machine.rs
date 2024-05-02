use console::Term;
use intel8080::cpu::CPU;
use std::{error::Error, io::stdout, io::Write, sync::mpsc, thread, time::Duration};

pub struct Machine {
    pub cpu: CPU,
    config: crate::config::Config,
}

impl Machine {
    pub fn new() -> Result<Machine, Box<dyn Error>> {
        let config = crate::config::load_config_file()?;
        Ok(Self {
            cpu: CPU::new(config.memory.ram),
            config,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let (tx, rx) = mpsc::channel();
        let term = Term::stdout();
        /* This byte of ROM at the end of address space is there to meet basic 3.2 initialization code requirement
        otherwise automatic RAM detection routine loops forever */
        self.cpu.bus.set_romspace(self.config.memory.ram, self.config.memory.ram);

        // Loads assembled program into memory
        self.cpu.bus.load_bin(&self.config.memory.rom, 0x0)?;

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
            if self.cpu.pc == 0xffff {
                return Ok(());
            };

            let pc = self.cpu.pc;
            let opcode = self.cpu.bus.read_byte(pc);
            match opcode {
                // IN
                0xdb => {
                    let port = self.cpu.bus.read_byte(pc + 1);
                    self.cpu.reg.a = match port {
                        0xFF => 0, // Altair switches set up for 88-SIO (4K BASIC 3.2)
                        0x00 => teletype.control,
                        0x01 => teletype.data,
                        _ => self.cpu.reg.a,
                    }
                }
                // OUT
                0xd3 => {
                    let port = self.cpu.bus.read_byte(pc + 1);
                    if port == 0x01 {
                        let value = self.cpu.reg.a & 0x7f;
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

            if let Some(t) = self.cpu.execute_timed() {
                thread::sleep(Duration::from_millis(t.into()))
            }
        }
    }
}