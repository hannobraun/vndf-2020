use std::{
    thread,
    time::{Duration, Instant},
};

use crate::world::FRAME_TIME;

pub fn main_loop<F>(mut action: F) -> !
where
    F: FnMut(),
{
    let frame_time = Duration::from_millis((FRAME_TIME * 1000.0) as u64);
    let mut last_update = Instant::now();

    loop {
        let now = Instant::now();

        let sleep_time = frame_time.checked_sub(now.duration_since(last_update));
        if let Some(sleep_time) = sleep_time {
            thread::sleep(sleep_time);
        }

        last_update = now;

        action();
    }
}
