use cgmath::Vector2;
use specs::{Component, Entity, VecStorage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    map: HashMap<Vector2<i32>, Entity>,
    min: Vector2<i32>,
    max: Vector2<i32>,
}

impl Map {
    pub fn new(min: Vector2<i32>, max: Vector2<i32>) -> Map {
        Map {
            map: HashMap::new(),
            min: min,
            max: max,
        }
    }

    pub fn get_tile(&self, vector: &Vector2<i32>) -> Option<&Entity> {
        self.map.get(vector)
    }

    pub fn set_tile(&mut self, vector: Vector2<i32>, entity: Entity) {
        self.map.insert(vector, entity);
    }

    pub fn get_tiles(&self) -> &HashMap<Vector2<i32>, Entity> {
        &self.map
    }
}

impl Component for Map {
    type Storage = VecStorage<Map>;
}
