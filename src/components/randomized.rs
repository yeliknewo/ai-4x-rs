use specs::{Component, VecStorage};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Randomized {
    i: u64,
}

impl Randomized {
    pub fn new(i: u64) -> Randomized {
        Randomized {
            i: i,
        }
    }

    pub fn get_i(&self) -> u64 {
        self.i
    }
}

impl Component for Randomized {
    type Storage = VecStorage<Randomized>;
}
