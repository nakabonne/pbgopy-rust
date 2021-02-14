mod app;
mod exit_codes;

use std::env;
use std::process;

use anyhow::{anyhow, Result};

use crate::exit_codes::ExitCode;

fn run() -> Result<ExitCode> {
    let matches = app::build_app().get_matches_from(env::args_os());

    return Err(anyhow!(
        "Hey",
    ));
}

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            eprintln!("[pbgopy error]: {:#}", err);
            process::exit(ExitCode::GeneralError.into());
        }
    }
}
