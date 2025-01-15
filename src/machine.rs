use crate::teletype::Console;
use crate::teletype::{ConsoleMsg, Teletype};
use intel8080::cpu::CPU;
use std::{
    error, fmt, io::stdout, io::Write, sync::mpsc, sync::mpsc::SendError, thread, time::Duration,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MachineError {
    ConfigFile,
    ConfigFileFmt,
    IOError,
    SendMsgError,
    SnapshotError,
}

impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Machine error: ")?;
        f.write_str(match self {
            MachineError::ConfigFileFmt => "Bad config file format",
            MachineError::ConfigFile => "Can't load config file",
            MachineError::IOError => "I/O Error",
            MachineError::SendMsgError => "Message not sent",
            MachineError::SnapshotError => "Snapshot I/O error",
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

impl From<SendError<ConsoleMsg>> for MachineError {
    fn from(_e: SendError<ConsoleMsg>) -> MachineError {
        MachineError::SendMsgError
    }
}

impl From<intel8080::bus::SnapshotError> for MachineError {
    fn from(_e: intel8080::bus::SnapshotError) -> MachineError {
        MachineError::SnapshotError
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

        // Loads configured ROM to memory
        if self.cpu.bus.load_bin(&self.config.memory.rom, 0x0).is_err() {
            println!("Can't load ROM file {} !", &self.config.memory.rom);
            return Err(MachineError::IOError);
        }

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
                    ConsoleMsg::LoadSnap => match self.load_snapshot() {
                        Ok(_) => {
                            println!("Snapshot loaded !");
                        }
                        Err(_) => {
                            println!(
                                "Can't load snapshot file {}altair.snapshot",
                                self.config.snapshot.dir
                            );
                        }
                    },
                    ConsoleMsg::SaveSnap => match self.save_snapshot() {
                        Ok(_) => {
                            println!("Snapshot saved !");
                        }
                        Err(_) => {
                            println!(
                                "Can't save snapshot file {}altair.snapshot",
                                self.config.snapshot.dir
                            );
                        }
                    },
                    ConsoleMsg::ResetCpu => {
                        self.cpu.pc = 0;
                    }
                    ConsoleMsg::Quit => {
                        return Ok(());
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
                        if (32..=125).contains(&value) || value == 0x0a || value == 0x0d {
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
