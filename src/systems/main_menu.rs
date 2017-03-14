use components::Button;
use events::{GameFromMainMenu, GameToMainMenu};
use specs::{Entity, RunArg, System};
use utils::{ButtonState, DuoChannel, MouseButton};

#[derive(Debug)]
pub struct SystemMainMenu {
    channel_game: DuoChannel<GameFromMainMenu, GameToMainMenu>,
    entity_opt_play_button: Option<Entity>,
    entity_opt_play_button_text: Option<Entity>,
}

impl SystemMainMenu {
    pub fn new(channel_game: DuoChannel<GameFromMainMenu, GameToMainMenu>) -> SystemMainMenu {
        SystemMainMenu {
            channel_game: channel_game,
            entity_opt_play_button: None,
            entity_opt_play_button_text: None,
        }
    }
}

impl System<f64> for SystemMainMenu {
    fn run(&mut self, arg: RunArg, _delta_time: f64) {
        while let Some(event) = self.channel_game.try_recv() {
            match event {
                GameToMainMenu::SetEntitiesPlayButton(entity_play_button, entity_play_button_text) => {
                    self.entity_opt_play_button = Some(entity_play_button);
                    self.entity_opt_play_button_text = Some(entity_play_button_text);
                }
                GameToMainMenu::Cleanup => {
                    if let Some(entity_play_button) = self.entity_opt_play_button.take() {
                        arg.delete(entity_play_button);
                    }
                    if let Some(entity_play_button_text) = self.entity_opt_play_button_text.take() {
                        arg.delete(entity_play_button_text);
                    }
                    self.channel_game.send(GameFromMainMenu::CleanupDone);
                }
            }
        }

        if self.entity_opt_play_button.is_some() {
            let buttons = arg.fetch(|w| (w.read::<Button>()));

            if let Some(play_button) = buttons.get(self.entity_opt_play_button.unwrap()) {
                match play_button.get_button_state(MouseButton::Left) {
                    ButtonState::Pressed => {
                        self.channel_game.send(GameFromMainMenu::CreateMainGameScene);
                    }
                    ButtonState::Released => (),
                }
            }
        } else {
            arg.fetch(|_| ());
        }
    }
}
