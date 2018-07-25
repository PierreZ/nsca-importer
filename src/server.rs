use config::Server;
use std::net::SocketAddr;

use codec;
use std::net::AddrParseError;
use tokio;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;

pub struct NSCAServer {
    addr: SocketAddr,
}

pub fn new(server: Server) -> Result<NSCAServer, AddrParseError> {
    let addr = server.address.parse()?;
    Ok(NSCAServer { addr: addr })
}

impl NSCAServer {
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        let server = listener
            .incoming()
            .for_each(|socket| Ok(()))
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
