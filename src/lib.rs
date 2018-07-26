#[macro_use]
extern crate serde_derive;
extern crate byteorder;
extern crate bytes;
extern crate config as configrs;
#[macro_use]
extern crate futures;
extern crate tokio;

pub mod codec;
pub mod config;
pub mod init;

use config::Server;

use futures::future::{self, Either};
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
                init::send_init_packet(stream).and_then(|(_, _, _)| {
                    process(stream);
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

fn process(socket: tokio::net::TcpStream) {
    // Define the task that processes the connection.
    let task = codec::NSCACodec::new(socket);

    let connection = task.into_future()
        // `into_future` doesn't have the right error type, so map the error to
        // make it work.
        // Process the first received line as the client's name.
        .and_then(|(data, lines)| {
            // If `data` is `None`, then the client disconnected without
            // actually sending a line of data.
            //
            // Since the connection is closed, there is no further work that we
            // need to do. So, we just terminate processing by returning
            // `future::ok()`.
            //
            // The problem is that only a single future type can be returned
            // from a combinator closure, but we want to return both
            // `future::ok()` and `Peer` (below).
            //
            // This is a common problem, so the `futures` crate solves this by
            // providing the `Either` helper enum that allows creating a single
            // return type that covers two concrete future types.
            let data = match data {
                Some(data) => data,
                None => {
                    // The remote client closed the connection without sending
                    // any data.
                    return Either::A(future::ok(()));
                }
            };

            println!("`received {:?}`", data);

            // Wrap `peer` with `Either::B` to make the return type fit.
            Either::B(data)
        })
        // Task futures have an error of type `()`, this ensures we handle the
        // error. We do this by printing the error to STDOUT.
        .map_err(|e| {
            println!("connection error = {:?}", e);
        });

    // Spawn the task. Internally, this submits the task to a thread pool.
    tokio::spawn(connection);
}
