use crate::Machine;
use std::{fs, path::PathBuf};
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

    pub fn load_snapshot(&mut self) -> std::io::Result<()> {
        let mut file = PathBuf::from(&self.config.snapshot.dir);
        file.push("test.snapshot");

        let snapshot = fs::read(file)?;
        self.cpu.import_snapshot(snapshot)?;

        Ok(())
    }
}
