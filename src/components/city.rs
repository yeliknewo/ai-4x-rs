use cgmath::Vector2;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct City {
    pos: Vector2<i32>,
    size: usize,
    range: i32,
    pop: u32,
}

impl City {
    pub fn new(pos: Vector2<i32>, size: usize) -> City {
        City {
            pos: pos,
            size: size,
            range: 1,
            pop: 0,
        }
    }

    pub fn get_pos(&self) -> Vector2<i32> {
        self.pos
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_range(&self) -> i32 {
        self.range
    }

    pub fn get_pop(&self) -> u32 {
        self.pop
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_range(&mut self, range: i32) {
        self.range = range;
    }

    pub fn set_pop(&mut self, pop: u32) {
        self.pop = pop;
    }
}

impl Component for City {
    type Storage = VecStorage<City>;
}
