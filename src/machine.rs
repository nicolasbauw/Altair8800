use crate::teletype::Console;
use crate::teletype::{ConsoleMsg, Teletype};
use intel8080::cpu::CPU;
use std::{fmt, error, io::stdout, io::Write, sync::mpsc, thread, time::Duration};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MachineError {
    ConfigFile,
    ConfigFileFmt,
    IOError,
}

impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Snapshot error : ")?;
        f.write_str(match self {
            MachineError::ConfigFileFmt => "Bad config file format",
            MachineError::ConfigFile => "Can't load config file",
            MachineError::IOError => "I/O Error",
        })
    }
}

impl From<std::io::Error> for MachineError {
    fn from(_e: std::io::Error) -> MachineError {
        MachineError::IOError
    }
}

impl From<toml::de::Error> for MachineError {
    fn from(_e: toml::de::Error) -> MachineError {
        MachineError::ConfigFileFmt
    }
}

impl From<MachineError> for std::io::Error {
    fn from(e: MachineError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    }
}

impl error::Error for MachineError {}

pub struct Machine {
    pub cpu: CPU,
    pub config: crate::config::Config,
}

impl Machine {
    pub fn new() -> Result<Machine, MachineError> {
        let config = crate::config::load_config_file()?;
        Ok(Self {
            cpu: CPU::new(config.memory.ram),
            config,
        })
    }

    pub fn run(&mut self) -> Result<(), MachineError> {
        let (tx, rx) = mpsc::channel();
        /* This byte of ROM at the end of address space is there to meet basic 3.2 initialization code requirement
        otherwise automatic RAM detection routine loops forever */
        self.cpu
            .bus
            .set_romspace(self.config.memory.ram, self.config.memory.ram);

        // Loads assembled program into memory
        self.cpu.bus.load_bin(&self.config.memory.rom, 0x0)?;

        // Spawns the console thread
        Console::spawn(tx);

        let mut teletype = Teletype::new();

        loop {
            // Checking if data was sent from the "console" thread
            if let Ok(msg) = rx.try_recv() {
                match msg {
                    ConsoleMsg::Char(ch) => {
                        // A key has been pressed ? device 0 sends 0 (output device ready)
                        teletype.control = 0;
                        // Then, device 1 sends the key code
                        teletype.data = ch;
                    }
                    ConsoleMsg::LoadSnap => {
                        match self.load_snapshot() {
                            Ok(_) => { println!("Snapshot loaded !"); },
                            Err(e) => { println!("{}", e); }
                        }
                    }
                    ConsoleMsg::SaveSnap => {
                        match self.save_snapshot() {
                            Ok(_) => { println!("Snapshot saved !"); },
                            Err(e) => { println!("{}", e); }
                        }
                    }
                }
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
