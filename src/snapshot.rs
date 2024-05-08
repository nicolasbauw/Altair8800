use crate::Machine;
use std::{error, fmt, fs, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SnapshotError {
    InvalidHeader,
    IOError,
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Snapshot error : ")?;
        f.write_str(match self {
            SnapshotError::InvalidHeader => "Invalid header",
            SnapshotError::IOError => "I/O Error",
        })
    }
}

impl From<std::io::Error> for SnapshotError {
    fn from(_e: std::io::Error) -> SnapshotError {
        SnapshotError::IOError
    }
}

impl error::Error for SnapshotError {}

impl Machine {
    pub fn save_snapshot(&mut self) -> std::io::Result<()> {
        // We create the snapshot file
        let mut file = PathBuf::from(&self.config.snapshot.dir);
        file.push("test.snapshot");

        // CPU snapshot creates the 0x2F first bytes of the snapshot
        let snapshot = self.cpu.export_snapshot();

        fs::write(file, snapshot)?;
        Ok(())
    }

    pub fn load_snapshot(&mut self) -> Result<(), SnapshotError> {
        let mut file = PathBuf::from(&self.config.snapshot.dir);
        file.push("test.snapshot");

        let snapshot = fs::read(file)?;
        // TODO fix always invalid check
        if snapshot[0..3] != [0x41, 0x4c, 0x54, 0x52] {
            return Err(SnapshotError::InvalidHeader);
        }

        self.cpu.import_snapshot(snapshot);

        Ok(())
    }
}
