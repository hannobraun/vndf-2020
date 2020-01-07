use vndf_client as client;


fn main() -> Result<(), client::Error> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or(
                "vndf_shared=info,vndf_client=info,vndf_launcher=info"
            )
    );

    client::start(("reineke.hannobraun.de", 34480))
}
