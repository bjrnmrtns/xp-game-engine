pub struct FrameCounter {
    start: std::time::Instant,
    fps: u64,
    last_frame: u64,
    last_elapsed: std::time::Duration,
}

impl FrameCounter {
    pub fn new(fps: u64) -> FrameCounter {
        FrameCounter {
            start: std::time::Instant::now(),
            fps: fps,
            last_frame: 0,
            last_elapsed: std::time::Duration::new(0, 0),
        }
    }
}

impl FrameCounter {
    pub fn frames(&mut self) -> (std::time::Duration, std::ops::Range<u64>) {
        let elapsed_since_start = self.start.elapsed();
        let elapsed_since_last = elapsed_since_start - self.last_elapsed;
        self.last_elapsed = elapsed_since_start;
        let end = (elapsed_since_start.as_secs_f64() * self.fps as f64) as u64;
        let begin = self.last_frame;
        self.last_frame = end;
        (elapsed_since_last, begin..end)
    }
}
