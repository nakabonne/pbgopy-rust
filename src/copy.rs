use std::env;
use std::io;
use std::io::prelude::*;
use std::process;

use anyhow::{anyhow, Result};

const PBGOPY_SERVER_KEY: &str = "PBGOPY_SERVER";

pub fn run() -> Result<bool> {
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
    let r = client.put(&address).body(data).send();
    let r = match r {
        Ok(res) => res,
        Err(error) => {
            return Err(anyhow!("failed to make a request: {:#}", error));
        }
    };
    Ok(true)
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
