use specs::{RunArg, System};
use utils::DuoChannel;

#[derive(Debug)]
pub struct RandomizerSystem {}

impl RandomizerSystem {
    pub fn new() -> RandomizerSystem {
        RandomizerSystem {}
    }
}

impl System<f64> for RandomizerSystem {
    fn run(&mut self, arg: RunArg, delta_time: f64) {
        arg.fetch(|_| ());
    }
}
