use specs::Entity;

#[derive(Debug)]
pub enum GameToMainMenu {
    SetPlayButtonEntities(Entity, Entity),
}

#[derive(Debug)]
pub enum GameFromMainMenu {
    CreateMainGameScene,
}
