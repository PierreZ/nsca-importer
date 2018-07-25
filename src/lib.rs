#[macro_use]
extern crate serde_derive;
extern crate byteorder;
extern crate config as configrs;
extern crate futures;
extern crate tokio;

pub mod config;
pub mod init;

use config::Server;

use std::net::AddrParseError;
use std::net::SocketAddr;

use tokio::prelude::*;

pub struct NSCAServer {
    addr: SocketAddr,
}

pub fn new(server: Server) -> Result<NSCAServer, AddrParseError> {
    let addr = server.address.parse()?;
    Ok(NSCAServer { addr: addr })
}

impl NSCAServer {
    pub fn listen(&self) {
        let listener = tokio::net::TcpListener::bind(&self.addr).unwrap();

        let server = listener
            .incoming()
            .for_each(|stream| {
                init::send_init_packet(stream).and_then(|data_stream| {
                    println!("sent init packet, waiting for data...");
                    Ok(())
                })
            })
            .map_err(|err| {
                // Handle error by printing to STDOUT.
                println!("accept error = {:?}", err);
            });

        println!("server running");

        // Start the server
        //
        // This does a few things:
        //
        // * Start the Tokio runtime (reactor, threadpool, etc...)
        // * Spawns the `server` task onto the runtime.
        // * Blocks the current thread until the runtime becomes idle, i.e. all
        //   spawned tasks have completed.
        tokio::run(server);
    }
}
