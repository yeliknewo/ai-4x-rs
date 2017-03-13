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

    pub fn get_food(&self) -> i32 {
        self.food
    }

    pub fn get_iron(&self) -> i32 {
        self.iron
    }

    pub fn get_gold(&self) -> i32 {
        self.gold
    }
}

impl Component for Tile {
    type Storage = VecStorage<Tile>;
}
