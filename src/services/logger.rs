use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn new() -> Logger {
        Logger
    }

    pub fn clear_log(&self, file_name: &str) -> std::io::Result<()> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_name)?;
        Ok(())
    }

    pub fn write_log(&self, message_line: &str, file_name: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)?;

        writeln!(file, "{}", message_line)?;
        Ok(())
    }
}
