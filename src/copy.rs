use std::io;
use std::io::prelude::*;

use anyhow::{anyhow, Result};

use crate::exit_codes::ExitCode;

pub fn run() -> Result<ExitCode> {
    let mut data = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut data)?;
    // TODO: Run an HTTP GET method along with the data.
    return Err(anyhow!(
        "not implemented",
    ));
}

/*
struct CopyRunner {
    password: String
}
impl CopyRunner {
    fn encrypt(&self, String) -> String {

    }
}
*/
