use super::{Cmd, PBGOPY_SERVER_KEY};

use std::env;
use std::io;
use std::io::prelude::*;

use anyhow::{anyhow, Result};
use clap::Clap;

/// Copy from stdin
#[derive(Clap, Debug)]
pub struct Copy {
    // TODO: Make optional
    // TODO: Allow short.
    /// Password to derive the symmetric-key to be used for encryption
    #[clap(long)]
    password: String,
}

// TODO: Moduralize

impl Cmd for Copy {
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
        Ok(())
    }
}
