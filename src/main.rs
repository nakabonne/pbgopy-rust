mod app;
mod copy;
mod paste;

use std::env;
use std::process;

use anyhow::{anyhow, Result};

fn run() -> Result<bool> {
    let matches = app::build_app().get_matches_from(env::args_os());
    if let Some(ref sub_m) = matches.subcommand_matches("copy") {
        // TODO: Give password.
        println!("password: {:?}", sub_m.values_of("password"));
        return copy::run();
    }
    if let Some(ref sub_m) = matches.subcommand_matches("paste") {
        // TODO: Give password.
        println!("password: {:?}", sub_m.values_of("password"));
        return paste::run();
    }

    // TODO: Show help message.
    return Err(anyhow!("Hey",));
}

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            eprintln!("[pbgopy error]: {:#}", err);
            process::exit(1);
        }
    }
}
