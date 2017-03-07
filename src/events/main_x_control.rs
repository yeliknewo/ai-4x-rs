use utils::{ButtonState, MouseButton};

#[derive(Debug)]
pub enum MainToControl {
    MouseMoved(i32, i32),
    MouseButton(ButtonState, MouseButton),
}


#[derive(Debug)]
pub enum MainFromControl {

}
