use std::{
    fmt::Error,
    fs::{self, read_dir, Metadata},
    path::{Path, PathBuf},
};

pub struct FileHandler;

#[derive(Debug)]
pub enum VecContent {
    File {
        name: PathBuf,
        // metadata: Metadata,
        // contents: Vec<u8>,
    },
    Directory(Vec<VecContent>),
}

impl FileHandler {
    pub fn new() -> FileHandler {
        FileHandler
    }

    pub fn read_directory(&self, path: &str) -> Result<Vec<VecContent>, std::io::Error> {
        let mut directory = Vec::new();
        let paths = fs::read_dir(path)?;

        for entry in paths {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                match self.read_directory(entry_path.to_str().unwrap()) {
                    Ok(contents) => directory.push(VecContent::Directory(contents)),
                    Err(e) => return Err(e),
                }
            } else {
                //get the file metadata and push it to the vector
                match self.get_file_data(entry_path.to_str().unwrap()) {
                    Ok(file_data) => directory.push(file_data),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(directory)
    }

    pub fn get_file_data(&self, path: &str) -> Result<VecContent, std::io::Error> {
        let data = fs::read(path)?;

        let file_data = VecContent::File {
            name: PathBuf::from(path),
            // metadata: fs::metadata(path)?,
            // contents: data,
        };

        Ok(file_data)
    }
}
