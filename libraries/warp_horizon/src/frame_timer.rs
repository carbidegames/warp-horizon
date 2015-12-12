use time::{PreciseTime, Duration};

pub trait TickDelta {
    fn scale(&self, value: f32) -> f32;
}

impl TickDelta for Duration {
    fn scale(&self, value: f32) -> f32 {
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

#[test]
fn delta_measuring() {
    let mut timer = FrameTimer::start();
    ::std::thread::sleep(::std::time::Duration::from_millis(100));

    let duration_difference = timer.tick() - Duration::milliseconds(100);
    assert!(duration_difference < Duration::milliseconds(10));
    assert!(duration_difference > -Duration::milliseconds(10));
}
