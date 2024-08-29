use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::time::TimeImpl;

pub struct HostTime(Instant);

impl HostTime {
    pub fn new() -> Self {
        Self(Instant::now())
    }
}

impl TimeImpl for HostTime {
    fn now(&self) -> Duration {
        self.0.elapsed()
    }

    fn sleep(&self, duration: Duration) {
        sleep(duration);
    }
}
