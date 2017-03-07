use specs::{Component, VecStorage};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Randomized {}

impl Randomized {
    pub fn new() -> Randomized {
        Randomized {}
    }
}

impl Component for Randomized {
    type Storage = VecStorage<Randomized>;
}
