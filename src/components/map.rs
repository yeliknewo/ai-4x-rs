use cgmath::Point2;
use specs::{Component, Entity, VecStorage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    map: HashMap<Point2<i32>, Entity>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            map: HashMap::new(),
        }
    }

    pub fn get_tile(&self, point: &Point2<i32>) -> Option<&Entity> {
        self.map.get(point)
    }

    pub fn set_tile(&mut self, point: Point2<i32>, entity: Entity) {
        self.map.insert(point, entity);
    }
}

impl Component for Map {
    type Storage = VecStorage<Map>;
}
