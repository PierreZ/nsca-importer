use config::Server;
use std::net::SocketAddr;

use std::net::AddrParseError;
use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_codec::LinesCodec;

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
            .for_each(move |socket| {
                let framed_sock = socket.framed(LinesCodec::new());
                framed_sock
                    .for_each(|line| {
                        println!("Received line {}", line);
                        Ok(())
                    })
                    .map_err(|e| println!("failed to connect; err = {:?}", e));
                Ok(())
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
