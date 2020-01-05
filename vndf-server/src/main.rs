pub use vndf_shared as shared;


use std::{
    time::{
        Duration,
        Instant,
    },
    thread,
};

use self::shared::{
    Server,
    game::FRAME_TIME,
    net,
};


fn main() -> net::Result {
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("vndf_shared=info,vndf_server=info")
    );

    let mut server = Server::start_default()?;

    let     frame_time  = Duration::from_millis((FRAME_TIME * 1000.0) as u64);
    let mut last_update = Instant::now();

    loop {
        let now = Instant::now();

        let sleep_time = frame_time
            .checked_sub(now.duration_since(last_update));
        if let Some(sleep_time) = sleep_time {
            thread::sleep(sleep_time);
        }

        last_update = now;

        server.update();
    }
}
