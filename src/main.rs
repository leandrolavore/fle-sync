mod services;

use services::{file_handler::FileHandler, io_handler::IOHandler};

fn main() -> Result<(), std::io::Error> {
    let mut io = IOHandler::new();
    io.send_message("Enter the path to your directory:")?;

    let path = io.get_file_path();
    let mut file_handler = FileHandler::new();

    let origin_directory_content = file_handler.read_directory(&path)?;
    let target_directory_content = file_handler.read_directory("./test_destination")?;

    file_handler.log_directory_struct(&origin_directory_content);

    Ok(())
}
