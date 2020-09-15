use std::convert::From;
use std::path::PathBuf;

pub struct Filename {
    pub name: String,
    pub terminal: String,
    pub extension: String,
}

impl From<&PathBuf> for Filename {
    fn from(filename: &PathBuf) -> Filename {
        let filename = filename.to_str().unwrap();
        let filename: Vec<&str> = filename.split_terminator('.').collect();
        let name = String::from(filename[0]);
        let terminal = String::from(filename[1]);
        let extension = String::from(filename[2]);

        Filename {
            name,
            terminal,
            extension
        }
    }
}
