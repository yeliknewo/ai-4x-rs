use events::{MainFromControl, MainToControl};
use specs::{RunArg, System};
use utils::DuoChannel;

#[derive(Debug)]
pub struct ControlSystem {
    main_channel: DuoChannel<MainFromControl, MainToControl>,
}

impl ControlSystem {
    pub fn new(main_channel: DuoChannel<MainFromControl, MainToControl>) -> ControlSystem {
        ControlSystem {
            main_channel: main_channel,
        }
    }

    fn process_event(&mut self, event: &MainToControl) {
        match event {
            _ => (),
        }
    }
}

impl System<f64> for ControlSystem {
    fn run(&mut self, arg: RunArg, delta_time: f64) {
        arg.fetch(|_| ());
    }
}
