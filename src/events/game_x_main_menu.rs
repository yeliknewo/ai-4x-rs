use specs::Entity;

#[derive(Debug)]
pub enum GameToMainMenu {
    SetEntitiesPlayButton(Entity, Entity),
    Cleanup,
}

#[derive(Debug)]
pub enum GameFromMainMenu {
    CreateMainGameScene,
    CleanupDone,
}
