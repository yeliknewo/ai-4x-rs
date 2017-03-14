use specs::Entity;

#[derive(Debug)]
pub enum GameToMainGameMap {
    Cleanup,
    SetEntityMap(Entity),
}

#[derive(Debug)]
pub enum GameFromMainGameMap {
    CleanupDone,
}
