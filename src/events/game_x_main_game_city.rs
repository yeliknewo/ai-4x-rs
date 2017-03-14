use specs::Entity;

#[derive(Debug)]
pub enum GameToMainGameCity {
    Cleanup,
    SetEntityMap(Entity),
    SetEntityCity(Entity),
}

#[derive(Debug)]
pub enum GameFromMainGameCity {
    CleanupDone,
}
