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

        // CPU snapshot
        snapshot.extend_from_slice(self.cpu.export_snapshot().as_slice());

        // 12 null bytes
        snapshot.extend_from_slice(&[0x00; 12]);

        // ROM start / end
        let r = self.cpu.bus.get_romspace();
        snapshot.extend_from_slice(r.0.to_be_bytes().as_slice());
        snapshot.extend_from_slice(r.1.to_be_bytes().as_slice());

        // RAM
        snapshot.extend_from_slice(self.cpu.bus.export_address_space().as_slice());

        fs::write(file, snapshot)?;
        self.cpu.halt = false;
        Ok(())
    }

    pub fn load_snapshot(&mut self) -> std::io::Result<()> {
        let mut file = PathBuf::from(&self.config.snapshot.dir);
        file.push("test.snapshot");

        let snapshot = fs::read(file)?;
        Ok(())
    }
}
