use ontour::Module;
use log::{debug, info};
use structopt::{clap::AppSettings, StructOpt};
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(
    name = "wasm-runner",
    about = "Ontour",
    global_settings(&[
        AppSettings::ColoredHelp
    ]),
)]
struct CliOptions {
    /// The WebAssembly file to load.
    #[structopt(parse(from_os_str))]
    pub(crate) file_path: PathBuf,

     /// The operation to invoke in the WASM file.
    #[structopt()]
    pub(crate) operation: String,

    /// The data to pass to the operation
    #[structopt()]
    pub(crate) data: String,
}

// https://docs.rs/env_logger/0.9.0/env_logger/index.html#enabling-logging

fn main() {
    env_logger::init();
    debug!("Initialized logger");

    let options = CliOptions::from_args();

    match Module::from_path("crates/ontour/tests/test.wasm") {
        Ok(_) => {
            println!("Module loaded");
        }
        Err(e) => {
            println!("Module failed to load: {}", e);
        }
    }

    match run(options) {
        Ok(output) => {
            println!("{}", output);
            info!("Done");
        }
        Err(e) => {
            println!("Module failed to load: {}", e);
            std::process::exit(1);
        }
    }

    let _result = Module::from_path("./tests/test.wasm");
    Module::greet();
    println!("Hello, world!");
}

fn run(options: CliOptions) -> anyhow::Result<String> {
    let module = Module::from_path(&options.file_path)?;
    info!("Module loaded");

    let bytes = rmp_serde::to_vec(&options.data)?;
    let result = module.run(&options.operation, &bytes)?;
    let unpacked: String = rmp_serde::from_read_ref(&result)?;

    Ok(unpacked)
}