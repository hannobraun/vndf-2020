use std::{
    collections::VecDeque,
    time::{
        Duration,
        Instant,
    },
};

use crate::shared::{
    cgs::Handle,
    game::{
        WORLD_SIZE,
        Diagnostics,
        base::{
            Component,
            ComponentHandle,
        },
        players::PlayerId,
    },
    net::data::Data,
};


pub struct State {
    pub own_id:      Option<PlayerId>,
    pub diagnostics: Option<Diagnostics>,
    pub statistics:  Statistics,
    pub data:        Data,
    pub frame_time:  FrameTime,
}

impl State {
    pub fn new() -> Self {
        Self {
            own_id:      None,
            diagnostics: None,
            statistics:  Statistics::new(),
            data:        Data::new(),
            frame_time:  FrameTime::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.statistics.update();

        for body in self.data.bodies.values_mut() {
            body.enforce_boundary(
                WORLD_SIZE,
                &self.data.positions,
                &mut self.data.velocities,
            );
            body.update(
                dt,
                &mut self.data.directions,
                &mut self.data.positions,
                &mut self.data.velocities,
            );
        }
        for craft in self.data.crafts.values_mut() {
            craft.apply_thrust(
                dt,
                &mut self.data.bodies,
                &self.data.directions,
                &mut self.data.fuels,
            );
        }
        for explosion in self.data.explosions.values_mut() {
            explosion.update(dt);
        }
    }

    pub fn update_component(&mut self, handle: Handle, component: Component) {
        self.statistics.updates.push_back(Instant::now());
        self.data.update(handle, component);
    }

    pub fn remove_component(&mut self, handle: ComponentHandle) {
        self.statistics.removals.push_back(Instant::now());
        self.data.remove(handle);
    }
}


pub struct Statistics {
    pub updates:  VecDeque<Instant>,
    pub removals: VecDeque<Instant>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            updates:  VecDeque::new(),
            removals: VecDeque::new(),
        }
    }

    pub fn update(&mut self) {
        while let Some(instant) = self.updates.front() {
            if instant.elapsed() > Duration::from_secs(1) {
                self.updates.pop_front();
            }
            else {
                break;
            }
        }
        while let Some(instant) = self.removals.front() {
            if instant.elapsed() > Duration::from_secs(1) {
                self.removals.pop_front();
            }
            else {
                break;
            }
        }
    }
}


pub struct FrameTime(VecDeque<f32>);

impl FrameTime {
    const MAX_LEN: usize = 60;

    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, time: f32) {
        self.0.push_back(time);
        while self.0.len() > Self::MAX_LEN {
            self.0.pop_front();
        }
    }

    pub fn report(&self) -> Report {
        let mut report = Report {
            latest: 0.0,
            avg_1:  0.0,
            avg_2:  0.0,
            avg_3:  0.0,
        };

        let mut sum = 0.0;

        for (i, &time) in self.0.iter().enumerate() {
            report.latest = time;
            sum += time;

            if i + 1 == Self::MAX_LEN / 4 {
                report.avg_1 = sum / (Self::MAX_LEN / 4) as f32;
            }
            if i + 1 == Self::MAX_LEN / 2 {
                report.avg_2 = sum / (Self::MAX_LEN / 2) as f32;
            }
            if i + 1 == Self::MAX_LEN {
                report.avg_3 = sum / Self::MAX_LEN as f32;
            }
        }

        report
    }
}


pub struct Report {
    pub latest: f32,
    pub avg_1:  f32,
    pub avg_2:  f32,
    pub avg_3:  f32,
}
