use std::io::{self, Write};

pub struct IOHandler;

impl IOHandler {
    pub fn new() -> IOHandler {
        IOHandler
    }

    pub fn send_message(&self, message: &str) -> io::Result<()> {
        let full_message = format!("{}\n", message);
        io::stdout().write_all(full_message.as_bytes())?;
        io::stdout().flush()?;
        Ok(())
    }

    pub fn get_file_path(self) -> String {
        let mut string = String::new();

        io::stdin()
            .read_line(&mut string)
            .expect("Could not read input");

        match string.trim().parse::<String>() {
            Ok(string) => string,
            Err(_) => {
                println!("Please enter a valid number");
                self.get_file_path()
            }
        }
    }
}
