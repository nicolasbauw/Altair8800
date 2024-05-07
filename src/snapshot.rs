use crate::Machine;
use std::{fs, path::PathBuf, fmt, error};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SnapshotError {
    InvalidHeader
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Snapshot error : ")?;
        f.write_str(match self {
            SnapshotError::InvalidHeader => "Invalid header",
        })
    }
}

impl From<std::io::Error> for SnapshotError {
    fn from(_e: std::io::Error) -> SnapshotError {
        SnapshotError::InvalidHeader
    }
}

impl error::Error for SnapshotError {}

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

    pub fn load_snapshot(&mut self) -> Result<(), SnapshotError> {
        let mut file = PathBuf::from(&self.config.snapshot.dir);
        file.push("test.snapshot");

        let snapshot = fs::read(file)?;
        if snapshot[0..3] != [0x41, 0x4c, 0x54, 0x52] { return Err(SnapshotError::InvalidHeader) }
        Ok(())
    }
}
