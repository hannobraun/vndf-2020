use std::collections::VecDeque;

use time::{
    Duration,
    Instant,
};

use crate::{
    camera::Camera,
    game::{
        coords::World,
        input,
    },
    shared::{
        data,
        math::Pnt2,
        world::{
            planets::Planets,
            players::PlayerId,
        },
    },
};


pub struct State {
    pub own_id:      Option<PlayerId>,
    pub diagnostics: Option<data::server::Diagnostics>,
    pub statistics:  Statistics,
    pub data:        data::client::Components,
    pub frame_time:  FrameTime,
    pub camera:      Camera,
}

impl State {
    pub fn new() -> Self {
        Self {
            own_id:      None,
            diagnostics: None,
            statistics:  Statistics::new(),
            data:        data::client::Components::new(),
            frame_time:  FrameTime::new(),
            camera:      Camera::new(),
        }
    }

    pub fn update(&mut self, dt: f32, input: &input::Handler) {
        self.statistics.update();

        self.camera.zoom = input.zoom;
        if let Some(own_pos) = self.own_pos() {
            self.camera.center = World(own_pos);
        }

        for body in self.data.bodies.values_mut() {
            body.update(
                dt,
                &Planets(&self.data.planets),
                &mut self.data.positions,
                &mut self.data.velocities,
            );
        }
        for craft in self.data.crafts.values_mut() {
            craft.apply_thrust(
                dt,
                &mut self.data.bodies,
                &mut self.data.fuels,
            );
        }
        for explosion in self.data.explosions.values_mut() {
            explosion.update(dt);
        }
    }

    pub fn update_component(&mut self, component: data::client::Component) {
        self.statistics.updates.push_back(Instant::now());
        component.update(&mut self.data);
    }

    pub fn remove_component(&mut self, handle: &data::client::Handle) {
        self.statistics.removals.push_back(Instant::now());
        handle.remove(&mut self.data);
    }

    pub fn own_pos(&self) -> Option<Pnt2> {
        for ship in self.data.ships.values() {
            let craft = self.data.crafts.get(&ship.craft)?;
            let body  = self.data.bodies.get(&craft.body)?;
            let pos   = self.data.positions.get(&body.pos)?;

            if Some(craft.owner) == self.own_id {
                return Some(pos.0);
            }
        }

        None
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
            if instant.elapsed() > Duration::seconds(1) {
                self.updates.pop_front();
            }
            else {
                break;
            }
        }
        while let Some(instant) = self.removals.front() {
            if instant.elapsed() > Duration::seconds(1) {
                self.removals.pop_front();
            }
            else {
                break;
            }
        }
    }
}


pub struct FrameTime(VecDeque<Duration>);

impl FrameTime {
    const MAX_LEN: usize = 60;

    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, time: Duration) {
        self.0.push_back(time);
        while self.0.len() > Self::MAX_LEN {
            self.0.pop_front();
        }
    }

    pub fn report(&self) -> Report {
        let mut report = Report {
            latest: Duration::zero(),
            avg_1:  Duration::zero(),
            avg_2:  Duration::zero(),
            avg_3:  Duration::zero(),
        };

        let mut sum = Duration::zero();

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
    pub latest: Duration,
    pub avg_1:  Duration,
    pub avg_2:  Duration,
    pub avg_3:  Duration,
}
