use specs::Entity;

#[derive(Debug)]
pub enum GameToMainGamePauseMenu {
    SetEntitiesExitButton(Entity, Entity),
    Cleanup,
}

#[derive(Debug)]
pub enum GameFromMainGamePauseMenu {
    CreateMainMenuScene,
    CleanupDone,
}
