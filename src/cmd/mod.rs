mod copy;
mod paste;
mod serve;

use anyhow::Result;
use clap::{AppSettings, Clap};

pub use copy::Copy;
pub use paste::Paste;
pub use serve::Serve;

const PBGOPY_SERVER_KEY: &str = "PBGOPY_SERVER";

pub trait Cmd {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Clap)]
#[clap(
    about,
    author,
    global_setting(AppSettings::GlobalVersion),
    global_setting(AppSettings::VersionlessSubcommands),
    version = "0.1.0"
)]
pub enum App {
    Copy(Copy),
    Paste(Paste),
    Serve(Serve),
}

impl Cmd for App {
    fn run(&self) -> Result<()> {
        match self {
            App::Copy(cmd) => cmd.run(),
            App::Paste(cmd) => cmd.run(),
            App::Serve(cmd) => cmd.run(),
        }
    }
}
