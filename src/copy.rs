use std::env;
use std::io;
use std::io::prelude::*;

use anyhow::{anyhow, Result};

// TODO: Moduralize
const PBGOPY_SERVER_KEY: &str = "PBGOPY_SERVER";

pub fn run() -> Result<bool> {
    let address = match env::var(PBGOPY_SERVER_KEY) {
        Ok(val) => val,
        Err(err) => {
            return Err(anyhow!(
                "put the pbgopy server's address into {} environment variable: {}",
                PBGOPY_SERVER_KEY,
                err
            ));
        }
    };

    let mut data = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut data)?;

    let client = reqwest::blocking::Client::new();
    let r = client.put(&address).body(data).send();
    let _ = match r {
        Ok(_) => (),
        Err(err) => {
            return Err(anyhow!("failed to make a request: {:#}", err));
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
