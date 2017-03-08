use specs::{Component, VecStorage};
use std::collections::HashMap;
use utils::{ButtonState, MouseButton};

#[derive(Debug)]
pub struct Button {
    button_states: HashMap<MouseButton, ButtonState>,
}

impl Button {
    pub fn new() -> Button {
        Button {
            button_states: HashMap::new(),
        }
    }

    pub fn get_button_state(&self, mouse_button: MouseButton) -> ButtonState {
        if let Some(button_state) = self.button_states.get(&mouse_button) {
            button_state.clone()
        } else {
            ButtonState::Released
        }
    }

    pub fn set_button_state(&mut self, mouse_button: MouseButton, button_state: ButtonState) {
        self.button_states.insert(mouse_button, button_state);
    }
}

impl Component for Button {
    type Storage = VecStorage<Button>;
}
