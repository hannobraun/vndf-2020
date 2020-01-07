use std::thread;

use vndf_client as client;
use vndf_shared::{
    self as shared,
    main_loop::main_loop,
};


fn main() -> Result<(), client::Error> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_client=info")
    );

    #[cfg(feature="production")]
    let (mut server, addr) = (DummyServer, ("reineke.hannobraun.de", 34480));

    #[cfg(not(feature = "production"))]
    let (mut server, addr) = {
        let server = Server::start_local()?;
        let addr   = server.addr();
        (server, addr)
    };

    thread::spawn(move || main_loop(|| server.update()));
    client::start(addr)
}


#[cfg(feature = "production")]
type Server = DummyServer;

#[cfg(not(feature = "production"))]
type Server = shared::Server;
