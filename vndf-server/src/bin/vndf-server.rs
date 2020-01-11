use vndf_shared::{
    Server,
    main_loop::main_loop,
    net,
};


fn main() -> net::Result {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_server=info")
    );

    let mut server = Server::start_default()?;
    main_loop(|| server.update());
}
