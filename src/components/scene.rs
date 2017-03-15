use specs::{Component, Entity, VecStorage};

#[derive(Debug)]
pub struct Scene {
    entities: Vec<Entity>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            entities: vec![],
        }
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn get_mut_entities(&mut self) -> &mut Vec<Entity> {
        &mut self.entities
    }
}

impl Component for Scene {
    type Storage = VecStorage<Scene>;
}
