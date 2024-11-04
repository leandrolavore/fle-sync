mod services;

use services::{file_handler::FileHandler, io_handler::IOHandler};

fn main() -> Result<(), std::io::Error> {
    let mut io = IOHandler::new();
    io.send_message("Enter the path to your directory:")?;

    let path = io.get_file_path();
    let mut file_handler = FileHandler::new();

    match file_handler.read_directory(&path) {
        Ok(content) => println!(
            "🚀 ~ file: main.rs ~ line 12 ~ fn main ~ content: {:?}",
            content
        ),
        Err(e) => eprintln!("Error reading directory: {}", e),
    };
    Ok(())
}
