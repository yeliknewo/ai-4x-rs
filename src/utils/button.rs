
#[derive(Debug, Clone)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum MouseButton {
    Left,
    Right,
}
