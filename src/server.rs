use config::Server;

use std::io::Error;
use std::net::AddrParseError;
use std::net::SocketAddr;
use tokio;
use tokio::prelude::*;
use tokio_codec;

use codec;

pub struct NSCAServer {
    addr: SocketAddr,
}

pub fn new(server: Server) -> Result<NSCAServer, AddrParseError> {
    let addr = server.address.parse()?;
    Ok(NSCAServer { addr: addr })
}

impl NSCAServer {
    pub fn listen(&self) -> Result<(), Box<Error>> {
        let listener = tokio::net::TcpListener::bind(&self.addr).unwrap();

        let server = listener
            .incoming()
            .map_err(|e| eprintln!("Failed to establish connection: {:?}", e))
            .for_each(|sock| {
                println!("A sock!");
                let framed_sock = sock.framed(codec::NSCACodec::new());
                let datas = framed_sock
                    .map_err(|e| eprintln!("Problem with connection: {:?}", e))
                    .for_each(|data| {
                        println!("{:?}", data);
                        Ok(())
                    });

                tokio::spawn(datas);

                Ok(())
            });

        tokio::run(server);

        Ok(())
    }
}
