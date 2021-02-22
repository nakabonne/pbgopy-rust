use super::{Cmd, PBGOPY_SERVER_KEY};

use std::env;
use std::io;
use std::io::prelude::*;

use anyhow::{anyhow, Result};
use clap::Clap;

/// Paste to stdout
#[derive(Clap, Debug)]
pub struct Paste {
    // TODO: Make optional
    // TODO: Allow short.
    /// Password to derive the symmetric-key to be used for decryption
    #[clap(short, long, default_value = "")]
    password: String,
}

impl Cmd for Paste {
    fn run(&self) -> Result<()> {
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
        Ok(())
    }
}
