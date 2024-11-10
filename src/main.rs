mod services;

use services::{file_handler::FileHandler, io_handler::IOHandler, logger::Logger};
use std::time::Instant;

const LOG_FILE: &str = "./log.txt";

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();

    let io = IOHandler::new();
    io.send_message("Enter the path to your directory:")?;

    let file_handler = FileHandler::new();

    let origin_directory_content = file_handler.read_directory("./test_origin")?;
    let logger = Logger::new();
    logger.clear_log(&LOG_FILE)?;

    file_handler.log_directory_struct(&origin_directory_content, &LOG_FILE)?;
    let target_path = "./test_destination";

    file_handler.sync_directories(&origin_directory_content, "./test_origin", &target_path)?;

    let duration = start.elapsed();
    println!("Script completed in: {:?}", duration);

    Ok(())
}
