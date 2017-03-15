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
}

impl Component for Scene {
    type Storage = VecStorage<Scene>;
}
