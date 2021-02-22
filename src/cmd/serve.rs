use super::Cmd;

use crate::cache;

use anyhow::{anyhow, Result};
use clap::Clap;
use simple_server::{Method, Server, StatusCode};

/// Start the server that acts like a clipboard
#[derive(Clap, Debug)]
pub struct Serve {
    /// The port the server listens on
    #[clap(short, long, default_value = "9090")]
    port: u32,
}

impl Cmd for Serve {
    fn run(&self) -> Result<()> {
        // TODO: Treat as a Cache trait.
        let cache = cache::memory_cache::MemoryCache::new();
        let server =
            Server::new(
                |request, mut response| match (request.method(), request.uri().path()) {
                    (&Method::GET, "/") => {
                        cache.get();
                        Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
                    }
                    (&Method::PUT, "/") => {
                        Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
                    }
                    (_, _) => {
                        response.status(StatusCode::NOT_FOUND);
                        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
                    }
                },
            );

        server.listen("0.0.0.0", &self.port.to_string());
    }
}
