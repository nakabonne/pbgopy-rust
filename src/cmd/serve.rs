use super::Cmd;

use crate::cache;

use anyhow::Result;
use bytes::Bytes;
use clap::Clap;
use simple_server::{Method, Server, StatusCode};

const DATA_CACHE_KEY: &str = "data";

/// Start the server that acts like a clipboard
#[derive(Clap, Debug)]
pub struct Serve {
    /// The port the server listens on
    #[clap(short, long, default_value = "9090")]
    port: u32,
}

impl Cmd for Serve {
    fn run(&self) -> Result<()> {
        let c = cache::new_cache(cache::Kind::MemoryCache);
        let server = Server::new(move |request, mut response| {
            match (request.method(), request.uri().path()) {
                (&Method::GET, "/") => {
                    match c.get(DATA_CACHE_KEY) {
                        Some(res) => return Ok(response.body(res.to_vec())?),
                        None => {
                            response.status(StatusCode::INTERNAL_SERVER_ERROR);
                            return Ok(response
                                .body("Failed to get data from cache".as_bytes().to_vec())?);
                        }
                    };
                }
                (&Method::PUT, "/") => {
                    // FIXME: doesn't work well.
                    let b = Bytes::from(request.into_body());
                    c.put(DATA_CACHE_KEY.to_string(), b, None);
                    Ok(response.body(Vec::new())?)
                }

                (_, _) => {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body("Not found".as_bytes().to_vec())?)
                }
            }
        });

        server.listen("0.0.0.0", &self.port.to_string());
    }
}
