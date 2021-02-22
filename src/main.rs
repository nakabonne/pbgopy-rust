mod cache;
mod cmd;
mod error;

use crate::cmd::{App, Cmd};
use crate::error::SilentExit;

use clap::Clap;

use std::io::{self, Write};
use std::process;

pub fn main() {
    if let Err(e) = App::parse().run() {
        match e.downcast::<SilentExit>() {
            Ok(SilentExit { code }) => process::exit(code),
            Err(e) => {
                let _ = writeln!(io::stderr(), "pbgopy: {:?}", e);
                process::exit(1);
            }
        }
    }
}
