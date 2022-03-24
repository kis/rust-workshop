pub mod error;

use std::path::Path;
use std::fs::read;
use log::info;
use error::Error;

pub struct Module {
}

impl Module {
    pub fn new(bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        info!("Loading wasm file from path {:?}", path.as_ref());
        let bytes = read(path.as_ref()).map_err(
            |e| Error::FileNotReadable(
                path.as_ref().to_path_buf(),
                e.to_string()
            )
        )?;
        Self::new(&bytes);
        Ok(Self {})
    }

    pub fn greet() {
        println!("Greet from lib");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn loads_wasm_file() {
        let result = Module::from_path("./tests/test.wasm");
        assert!(result.is_ok());
    }
}