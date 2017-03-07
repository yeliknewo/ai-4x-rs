#[derive(Debug)]
pub struct FpsCounter {
    current_delta: f64,
    frames: u32,
    low_frame_rate: u32,
}

impl FpsCounter {
    pub fn new(low_frame_rate: u32) -> FpsCounter {
        FpsCounter {
            current_delta: 0.0,
            frames: 0,
            low_frame_rate: low_frame_rate,
        }
    }

    pub fn frame(&mut self, delta: f64) {
        self.frames += 1;
        self.current_delta += delta;

        while self.current_delta > 1.0 {
            self.current_delta -= 1.0;
            if self.frames < self.low_frame_rate {
                warn!("FPS Low: {}", self.frames);
            } else {
                debug!("FPS: {}", self.frames);
            }
            self.frames = 0;
        }
    }
}
