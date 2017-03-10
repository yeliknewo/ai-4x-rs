use components::Button;
use events::{GameFromMainMenu, GameToMainMenu};
use specs::{Entity, Join, RunArg, System};
use utils::{ButtonState, DuoChannel, MouseButton};

#[derive(Debug)]
pub struct MainMenuSystem {
    game_channel: DuoChannel<GameFromMainMenu, GameToMainMenu>,
    play_button: Option<Entity>,
    play_button_text: Option<Entity>,
}

impl MainMenuSystem {
    pub fn new(game_channel: DuoChannel<GameFromMainMenu, GameToMainMenu>) -> MainMenuSystem {
        MainMenuSystem {
            game_channel: game_channel,
            play_button: None,
            play_button_text: None,
        }
    }

    pub fn set_button(&mut self, play_button: Entity, play_button_text: Entity) {
        self.play_button = Some(play_button);
        self.play_button_text = Some(play_button_text);
    }
}

impl System<f64> for MainMenuSystem {
    fn run(&mut self, arg: RunArg, _delta_time: f64) {
        if self.play_button.is_some() && self.play_button_text.is_some() {

            let buttons = arg.fetch(|w| (w.read::<Button>()));

            if let Some(play_button) = buttons.get(self.play_button.unwrap()) {
                match play_button.get_button_state(MouseButton::Left) {
                    ButtonState::Pressed => {
                        arg.delete(self.play_button.take().unwrap());
                        arg.delete(self.play_button_text.take().unwrap());
                    }
                    ButtonState::Released => (),
                }
            }
        } else {

        }
    }
}
