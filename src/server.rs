use config::Server;

use std::io::Error;
use std::net::AddrParseError;
use std::net::SocketAddr;
use tokio;
use tokio::prelude::*;
use tokio_codec::{FramedRead, FramedWrite};

use codec::data;
use codec::init;

pub struct NSCAServer {
    addr: SocketAddr,
}

pub fn new(server: Server) -> Result<NSCAServer, AddrParseError> {
    let addr = server.address.parse()?;
    Ok(NSCAServer { addr: addr })
}

impl NSCAServer {
    pub fn listen(&self) -> Result<(), Box<Error>> {
        let listener = tokio::net::TcpListener::bind(&self.addr)?;

        let server = listener
            .incoming()
            .map_err(|e| eprintln!("Failed to establish connection: {:?}", e))
            .for_each(|sock| {
                println!("A sock!");

                let (r, w) = sock.split();
                let r_framed = FramedRead::new(r, data::Codec::new());
                let w_framed = FramedWrite::new(w, init::Codec::new());

                // client is waiting for an init packet before sending data
                // let's push it
                // ?

                // reading data in a dedicated future
                let reading_future = r_framed
                    .map_err(|e| eprintln!("Problem with connection: {:?}", e))
                    .for_each(|data| {
                        println!("{:?}", data);
                        Ok(())
                    });
                tokio::spawn(reading_future);

                Ok(())
            });

        tokio::run(server);

        Ok(())
    }
}
