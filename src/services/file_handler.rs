use super::logger::Logger;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct FileHandler;

#[derive(Debug)]
pub enum VecContent {
    File {
        name: PathBuf,
    },
    Directory {
        name: PathBuf,
        content: Vec<VecContent>,
    },
}

impl FileHandler {
    pub fn new() -> FileHandler {
        FileHandler
    }

    pub fn read_directory(&self, path: &str) -> Result<VecContent, std::io::Error> {
        let mut directory_content = Vec::new();
        let paths = fs::read_dir(path)?;

        for entry in paths {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                match self.read_directory(entry_path.to_str().unwrap()) {
                    Ok(contents) => directory_content.push(VecContent::Directory {
                        name: entry_path.clone(),
                        content: match contents {
                            VecContent::Directory { content, .. } => content,
                            _ => vec![],
                        },
                    }),
                    Err(e) => return Err(e),
                }
            } else {
                match self.get_file_data(entry_path.to_str().unwrap()) {
                    Ok(file_data) => directory_content.push(file_data),
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(VecContent::Directory {
            name: PathBuf::from(path),
            content: directory_content,
        })
    }

    pub fn get_file_data(&self, path: &str) -> Result<VecContent, std::io::Error> {
        let file_data = VecContent::File {
            name: PathBuf::from(path),
        };

        Ok(file_data)
    }

    pub fn log_directory_struct(
        &self,
        vec: &VecContent,
        log_file: &str,
    ) -> Result<(), std::io::Error> {
        let logger = Logger::new();

        match vec {
            VecContent::File { name } => {
                let file_path = &name.to_string_lossy();
                logger.write_log(file_path, &log_file)?;
            }
            VecContent::Directory { name, content } => {
                let dir_path = &name.to_string_lossy();
                logger.write_log(dir_path, &log_file)?;

                for sub_item in content {
                    self.log_directory_struct(sub_item, &log_file)?;
                }
            }
        }

        Ok(())
    }

    pub fn sync_directories(
        &self,
        origin: &VecContent,
        origin_path: &str,
        target_path: &str,
    ) -> io::Result<()> {
        match origin {
            VecContent::Directory { name, content } => {
                let relative_path = name.strip_prefix(origin_path).unwrap();
                let target_dir_path = Path::new(target_path).join(relative_path);

                fs::create_dir_all(&target_dir_path)?;

                for item in content {
                    self.sync_directories(item, origin_path, target_path)?;
                }
            }
            VecContent::File { name } => {
                let relative_path = name.strip_prefix(origin_path).unwrap();
                let target_file_path = Path::new(target_path).join(relative_path);

                if !target_file_path.exists() {
                    fs::copy(name, &target_file_path)?;
                }
            }
        }

        Ok(())
    }
}
