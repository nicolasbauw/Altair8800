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
        
        // CPU registers
        self.cpu.reg.a = snapshot[0x08];
        self.cpu.reg.b = snapshot[0x09];
        self.cpu.reg.c = snapshot[0x0A];
        self.cpu.reg.d = snapshot[0x0B];
        self.cpu.reg.e = snapshot[0x0C];
        self.cpu.reg.h = snapshot[0x0D];
        self.cpu.reg.l = snapshot[0x0E];

        // CPU flags
        self.cpu.flags.from_byte(snapshot[0x0F]);

        // PC
        self.cpu.pc = u16::from_be_bytes([snapshot[0x10], snapshot[0x11]]);

        // SP
        self.cpu.sp = u16::from_be_bytes([snapshot[0x12], snapshot[0x13]]);

        // int
        self.cpu.int = (snapshot[0x14] != 0, snapshot[0x15]);

        // inte
        self.cpu.inte = snapshot[0x16] != 0;

        // slice_duration
        let slice_duration = u32::from_be_bytes([snapshot[0x18], snapshot[0x19], snapshot[0x1A], snapshot[0x1B]]);
        
        // slice_max_cycles
        let slice__max_cycles = u32::from_be_bytes([snapshot[0x1C], snapshot[0x1D], snapshot[0x1E], snapshot[0x1F]]);

        Ok(())
    }
}
