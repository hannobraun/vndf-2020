use std::thread;

use vndf_client as client;
use vndf_shared::{
    Server,
    main_loop::main_loop,
};


fn main() -> Result<(), client::Error> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_client=info")
    );

    let mut server = Server::start_local()?;
    let     addr   = server.addr();

    thread::spawn(move || main_loop(|| server.update()));
    client::start(addr)
}
