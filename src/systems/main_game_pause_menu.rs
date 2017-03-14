use components::Button;
use events::{GameFromMainGamePauseMenu, GameToMainGamePauseMenu};
use specs::{Entity, RunArg, System};
use utils::{ButtonState, DuoChannel, MouseButton};

#[derive(Debug)]
pub struct SystemMainGamePauseMenu {
    channel_game: DuoChannel<GameFromMainGamePauseMenu, GameToMainGamePauseMenu>,
    entity_opt_exit_button: Option<Entity>,
    entity_opt_exit_button_text: Option<Entity>,
}

impl SystemMainGamePauseMenu {
    pub fn new(channel_game: DuoChannel<GameFromMainGamePauseMenu, GameToMainGamePauseMenu>) -> SystemMainGamePauseMenu {
        SystemMainGamePauseMenu {
            channel_game: channel_game,
            entity_opt_exit_button: None,
            entity_opt_exit_button_text: None,
        }
    }
}

impl System<f64> for SystemMainGamePauseMenu {
    fn run(&mut self, arg: RunArg, _delta_time: f64) {
        while let Some(event) = self.channel_game.try_recv() {
            match event {
                GameToMainGamePauseMenu::SetEntitiesExitButton(entity_exit_button, entity_exit_button_text) => {
                    self.entity_opt_exit_button = Some(entity_exit_button);
                    self.entity_opt_exit_button_text = Some(entity_exit_button_text);
                }
                GameToMainGamePauseMenu::Cleanup => {
                    if let Some(entity_exit_button) = self.entity_opt_exit_button.take() {
                        arg.delete(entity_exit_button);
                    }
                    if let Some(entity_exit_button_text) = self.entity_opt_exit_button_text.take() {
                        arg.delete(entity_exit_button_text);
                    }
                    self.channel_game.send(GameFromMainGamePauseMenu::CleanupDone);
                }
            }
        }

        if self.entity_opt_exit_button.is_some() {
            let buttons = arg.fetch(|w| (w.read::<Button>()));

            if let Some(play_button) = buttons.get(self.entity_opt_exit_button.unwrap()) {
                match play_button.get_button_state(MouseButton::Left) {
                    ButtonState::Pressed => {
                        self.channel_game.send(GameFromMainGamePauseMenu::CreateMainMenuScene);
                    }
                    ButtonState::Released => (),
                }
            }
        } else {
            arg.fetch(|_| ());
        }
    }
}
