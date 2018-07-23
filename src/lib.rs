#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate futures;
extern crate config as configrs;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_codec;

pub mod config;
pub mod server;