use cgmath::Point2;
use specs::{Component, VecStorage};
use std::collections::HashMap;
use utils::{ButtonState, MouseButton};

#[derive(Debug)]
pub struct Button {
    button_states: HashMap<MouseButton, ButtonState>,
    bot_left: Point2<f32>,
    top_right: Point2<f32>,
}

impl Button {
    pub fn new(bot_left: Point2<f32>, top_right: Point2<f32>) -> Button {
        Button {
            button_states: HashMap::new(),
            bot_left: bot_left,
            top_right: top_right,
        }
    }

    pub fn get_bot_left(&self) -> Point2<f32> {
        self.bot_left
    }

    pub fn get_top_right(&self) -> Point2<f32> {
        self.top_right
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
