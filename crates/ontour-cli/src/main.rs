use ontour::Module;
use log::debug;
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
}

// https://docs.rs/env_logger/0.9.0/env_logger/index.html#enabling-logging

fn main() {
    env_logger::init();
    debug!("Initialized logger");

    let _options = CliOptions::from_args();

    match Module::from_path("./module.wasm") {
        Ok(_) => {
            println!("Module loaded");
        }
        Err(e) => {
            println!("Module failed to load: {}", e);
        }
    }

    let _result = Module::from_path("./tests/test.wasm");
    Module::greet();
    println!("Hello, world!");
}
