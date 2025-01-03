use std::time::Instant;

pub struct TimeManager {
    pub current_time: u64,
    pub end_time: u64,
    pub replay_speed: f64,
    pub playing: bool,
    actual_time: Instant,
    accum_elapsed: u128,
    pub elapsed: u128,
}

impl TimeManager {
    pub fn new(end_time: u64, replay_speed: f64) -> Self {
        Self {
            current_time: 0,
            end_time,
            replay_speed,
            playing: false,
            actual_time: Instant::now(),
            accum_elapsed: 0,
            elapsed: 0,
        }
    }

    pub fn update(&mut self) {
        self.elapsed = self.actual_time.elapsed().as_millis() - self.accum_elapsed;
        self.accum_elapsed += self.elapsed;

        if self.playing {
            self.current_time += (self.elapsed as f64 * self.replay_speed) as u64;
            // Adjust increment by replay speed
        }

        if self.current_time > self.end_time {
            self.current_time = 0;
        }
    }

    pub fn reset(&mut self) {
        self.current_time = 0;
        self.accum_elapsed = 0;
        self.elapsed = 0;
        self.actual_time = Instant::now();
    }
}