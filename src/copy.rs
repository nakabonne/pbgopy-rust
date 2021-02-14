use std::env;
use std::io;
use std::io::prelude::*;
use std::process;

use anyhow::{anyhow, Result};

use crate::exit_codes::ExitCode;

const PBGOPY_SERVER_KEY: &str = "PBGOPY_SERVER";

pub fn run() -> Result<ExitCode> {
    let mut data = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let address = match env::var(PBGOPY_SERVER_KEY) {
        Ok(val) => val,
        Err(err) => {
            println!(
                "put the pbgopy server's address into {} environment variable: {}",
                PBGOPY_SERVER_KEY, err
            );
            process::exit(1);
        }
    };
    handle.read_to_string(&mut data)?;

    let client = reqwest::blocking::Client::new();
    let res = client.put(&address).body(data).send()?;
    return Err(anyhow!("not implemented",));
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
