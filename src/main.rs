#[macro_use]
extern crate structopt;

#[macro_use]
extern crate log;
extern crate nsca;
extern crate simplelog;

use nsca::config;
use simplelog::*;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "nsca-importer")]
struct Opt {
    /// config file
    #[structopt(short = "c", long = "config")]
    config: String,
}

fn main() -> Result<(), Box<Error>> {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default()).unwrap(),
    ]).unwrap();

    let opt = Opt::from_args();
    let setting = config::Settings::from(opt.config)?;

    info!("Starting nsca-importer with {:?}", setting);

    let server = nsca::server::new(setting.server)?;

    server.run();

    Ok(())
}
