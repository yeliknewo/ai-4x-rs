use components::Button;
use specs::{Entity, Join, RunArg, System};
use utils::{ButtonState, MouseButton};

#[derive(Debug)]
pub struct MainMenuSystem {
    play_button: Entity,
    play_button_text: Entity,
}

impl MainMenuSystem {
    pub fn new(play_button: Entity, play_button_text: Entity) -> MainMenuSystem {
        MainMenuSystem {
            play_button: play_button,
            play_button_text: play_button_text,
        }
    }
}

impl System<f64> for MainMenuSystem {
    fn run(&mut self, arg: RunArg, _delta_time: f64) {
        let buttons = arg.fetch(|w| (w.read::<Button>()));

        if let Some(play_button) = buttons.get(self.play_button) {
            match play_button.get_button_state(MouseButton::Left) {
                ButtonState::Pressed => {
                    arg.delete(self.play_button);
                    arg.delete(self.play_button_text);
                }
                ButtonState::Released => (),
            }
        }
    }
}
