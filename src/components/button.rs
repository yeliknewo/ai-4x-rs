use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Button {}

impl Button {
    pub fn new() -> Button {
        Button {}
    }
}

impl Component for Button {
    type Storage = VecStorage<Button>;
}
