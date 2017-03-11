use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Tile {
    food: i32,
    iron: i32,
    gold: i32,
}

impl Tile {
    pub fn new(food: i32, iron: i32, gold: i32) -> Tile {
        Tile {
            food: food,
            iron: iron,
            gold: gold,
        }
    }
}

impl Component for Tile {
    type Storage = VecStorage<Tile>;
}
