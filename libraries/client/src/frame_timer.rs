use std::ops::Mul;
use time::{PreciseTime, Duration};

pub trait UpdateDelta {
    fn scale<Val: Mul<f32>>(&self, value: Val) -> Val::Output;
}

impl UpdateDelta for Duration {
    fn scale<Val: Mul<f32>>(&self, value: Val) -> Val::Output {
        value * (self.num_microseconds().unwrap() as f32 / 1_000_000.0)
    }
}

#[test]
fn value_scaling_by_delta() {
    assert!(Duration::milliseconds(500).scale(1.0) == 0.5);
    assert!(Duration::milliseconds(500).scale(0.5) == 0.25);
    assert!(Duration::milliseconds(250).scale(1.0) == 0.25);
    assert!(Duration::milliseconds(250).scale(0.5) == 0.125);
}

pub struct FrameTimer {
    last_time: PreciseTime,
}

impl FrameTimer {
    pub fn start() -> Self {
        FrameTimer { last_time: PreciseTime::now() }
    }

    pub fn tick(&mut self) -> Duration {
        let time = PreciseTime::now();
        let delta = self.last_time.to(time);
        self.last_time = time;

        delta
    }
}
