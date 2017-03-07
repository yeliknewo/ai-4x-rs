use specs::{Component, VecStorage};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Cell {
    x: i64,
    y: i64,
    class: u64,
}

impl Cell {
    pub fn new(x: i64, y: i64, class: u64) -> Cell {
        Cell {
            x: x,
            y: y,
            class: class,
        }
    }

    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }

    pub fn get_class(&self) -> u64 {
        self.class
    }

    pub fn set_class(&mut self, class: u64) {
        self.class = class;
    }
}

impl Component for Cell {
    type Storage = VecStorage<Cell>;
}
