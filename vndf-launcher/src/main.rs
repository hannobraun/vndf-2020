use structopt::StructOpt;

use vndf_client as client;

#[derive(StructOpt)]
struct Options {
    #[structopt(short, long, default_value = "auto")]
    graphics: client::Graphics,

    #[structopt(short, long)]
    local: bool,
}

fn main() -> Result<(), client::Error> {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_client=info,vndf_launcher=info"),
    );

    let options = Options::from_args();

    let addr = if options.local {
        "localhost"
    } else {
        "reineke.hannobraun.de"
    };

    client::start((addr, 34480), options.graphics)
}
