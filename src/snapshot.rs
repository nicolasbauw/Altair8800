use crate::Machine;
use std::{fs, path::PathBuf};

impl Machine {
    pub fn save_snapshot(&mut self) -> std::io::Result<()> {
        // We stop the CPU while building the snapshot
        self.cpu.halt = true;

        let mut snapshot: Vec<u8> = Vec::new();
        let mut file = PathBuf::from(&self.config.snapshot.dir);
        file.push("test.snapshot");

        // Magic number
        snapshot.extend_from_slice(&[0x41, 0x4c, 0x54, 0x52]);

        // Snapshot version + 3 null bytes
        snapshot.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);

        // accumulator + 3 register pairs
        snapshot.push(self.cpu.reg.a);
        snapshot.push(self.cpu.reg.b);
        snapshot.push(self.cpu.reg.c);
        snapshot.push(self.cpu.reg.d);
        snapshot.push(self.cpu.reg.e);
        snapshot.push(self.cpu.reg.h);
        snapshot.push(self.cpu.reg.l);

        // Flags
        snapshot.push(self.cpu.flags.as_byte());

        // pc
        snapshot.push(((self.cpu.pc & 0xFF00) >> 8) as u8);
        snapshot.push((self.cpu.pc & 0x00FF) as u8);

        // sp
        snapshot.push(((self.cpu.sp & 0xFF00) >> 8) as u8);
        snapshot.push((self.cpu.sp & 0x00FF) as u8);

        // int
        snapshot.push((self.cpu.int.0) as u8);
        snapshot.push(self.cpu.int.1);

        // inte
        snapshot.push((self.cpu.inte) as u8);

        fs::write(file, snapshot)?;
        self.cpu.halt = false;
        Ok(())
    }
}
