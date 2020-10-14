use std::thread;

use structopt::StructOpt;

use vndf_client as client;
use vndf_server::server::Server;
use vndf_shared::{main_loop::main_loop, net};

#[derive(StructOpt)]
struct Options {
    #[structopt(short, long, default_value = "auto")]
    graphics: client::Graphics,
}

fn main() -> Result<(), Error> {
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("vndf_shared=info,vndf_client=info"),
    );

    let options = Options::from_args();

    let mut server = Server::start_local().map_err(|err| Error::Init(err))?;
    let addr = server.addr();

    thread::spawn(move || main_loop(|| server.update()));
    client::start(addr, options.graphics).map_err(|err| Error::Run(err))
}

#[derive(Debug)]
pub enum Error {
    Init(net::Error),
    Run(client::Error),
}
