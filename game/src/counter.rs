use std::time::Duration;

pub struct FrameCounter {
    time: std::time::Instant,
    fps: u64,
    counter: u64,
}

impl FrameCounter {
    pub fn new(fps: u64) -> FrameCounter {
        FrameCounter {
            time: std::time::Instant::now(),
            fps: fps,
            counter: 0,
        }
    }
}

impl FrameCounter {
    fn time_elapsed(&self) -> Duration {
        self.time.elapsed()
    }

    fn frame_time(&self) -> Duration {
        std::time::Duration::from_micros(1000000 / self.fps)
    }

    pub fn run(&mut self) -> bool {
        self.counter = (self.time_elapsed().as_micros() / self.frame_time().as_micros()) as u64;
        true
    }

    pub fn count(&self) -> u64 {
        self.counter
    }
}
