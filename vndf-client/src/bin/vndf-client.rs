use std::thread;

use vndf_client as client;
use vndf_server::server::Server;
use vndf_shared::{
    main_loop::main_loop,
    net,
};


fn main() -> Result<(), Error> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_client=info")
    );

    let mut server = Server::start_local()
        .map_err(|err| Error::Init(err))?;
    let     addr   = server.addr();

    thread::spawn(move || main_loop(|| server.update()));
    client::start(addr, client::Frontend::Ggez)
        .map_err(|err| Error::Run(err))
}


#[derive(Debug)]
pub enum Error {
    Init(net::Error),
    Run(client::Error),
}
