mod services;

use services::{file_handler::FileHandler, io_handler::IOHandler, logger::Logger};

const LOG_FILE: &str = "./log.txt";

fn main() -> Result<(), std::io::Error> {
    let io = IOHandler::new();
    io.send_message("Enter the path to your directory:")?;

    // let path = io.get_file_path();
    let file_handler = FileHandler::new();

    let origin_directory_content = file_handler.read_directory("./test_origin")?;
    println!(
        "🚀 ~ file: main.rs ~ line 13 ~ fnmain ~ origin_directory_content : {:?} ",
        origin_directory_content
    );
    let logger = Logger::new();
    logger.clear_log(&LOG_FILE)?;

    file_handler.log_directory_struct(&origin_directory_content, &LOG_FILE)?;
    let target_path = "./test_destination";

    file_handler.sync_directories(&origin_directory_content, "./test_origin", &target_path)?;

    Ok(())
}
