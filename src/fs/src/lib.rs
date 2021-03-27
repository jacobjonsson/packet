use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub trait FS {
    fn read_file(&self, path: &str) -> Result<String, std::io::Error>;
}

pub struct FSMock {
    files: HashMap<String, String>,
}

impl FSMock {
    pub fn new(files: HashMap<String, String>) -> FSMock {
        FSMock { files }
    }
}

impl FS for FSMock {
    fn read_file(&self, path: &str) -> Result<String, Error> {
        match self.files.get(path) {
            Some(file) => Ok(file.clone()),
            None => Err(Error::new(ErrorKind::NotFound, "File not found")),
        }
    }
}

pub struct FSImpl;

impl FS for FSImpl {
    fn read_file(&self, _: &str) -> Result<String, std::io::Error> {
        todo!()
    }
}
