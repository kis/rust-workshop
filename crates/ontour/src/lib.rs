pub mod error;

use std::path::Path;
use std::fs::read;
use log::{info, trace, debug};
use error::Error;
use wapc::WapcHost;

pub struct Module {
    host: WapcHost,
}

impl Module {
    pub fn new(bytes: &[u8]) -> Result<Self, Error> {
        let engine = wasmtime_provider::WasmtimeEngineProvider::new(bytes, None);

        let host = WapcHost::new(Box::new(engine), |_id, binding, ns, operation, payload| {
            trace!(
                "Guest called: binding={}, namespace={}, operation={}, payload={:?}",
                binding,
                ns,
                operation,
                payload
            );
            Err("Not implemented".into())
        })?;
        Ok(Module { host })
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        info!("Loading wasm file from path {:?}", path.as_ref());
        let bytes = read(path.as_ref()).map_err(
            |e| Error::FileNotReadable(
                path.as_ref().to_path_buf(),
                e.to_string()
            )
        )?;
        Self::new(&bytes)
    }

    pub fn greet() {
        println!("Greet from lib");
    }

    pub fn run(&self, operation: &str, payload: &[u8]) -> Result<Vec<u8>, Error> {
        debug!("Invoking {}", operation);
        let result = self.host.call(operation, payload)?;
        Ok(result)
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

    #[test]
    fn loads_wasm_file() {
        let result = Module::from_path("./tests/test.wasm");
        assert!(result.is_ok());
    }

    #[test]
    fn runs_operation() -> Result<(), Error> {
        let module = Module::from_path("./tests/test.wasm")?;

        let bytes = rmp_serde::to_vec("World").unwrap();
        let payload = module.run("hello", &bytes)?;
        let unpacked: String = rmp_serde::decode::from_read_ref(&payload).unwrap();
        assert_eq!(unpacked, "Hello, World.");
        Ok(())
    }
}