use std::env;
use std::io;
use std::str;

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
    let res = reqwest::blocking::get(&address);
    let mut res = match res {
        Ok(r) => r,
        Err(err) => {
            return Err(anyhow!("failed to make a request: {:#}", err));
        }
    };
    res.copy_to(&mut io::stdout())?;
    Ok(true)
}
