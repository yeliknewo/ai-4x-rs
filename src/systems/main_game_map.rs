use components::Map;
use events::{GameFromMainGameMap, GameToMainGameMap};
use specs::{Entity, RunArg, System};
use utils::DuoChannel;

#[derive(Debug)]
pub struct SystemMainGameMap {
    channel_game: DuoChannel<GameFromMainGameMap, GameToMainGameMap>,
    entity_opt_map: Option<Entity>,
}

impl SystemMainGameMap {
    pub fn new(channel_game: DuoChannel<GameFromMainGameMap, GameToMainGameMap>) -> SystemMainGameMap {
        SystemMainGameMap {
            channel_game: channel_game,
            entity_opt_map: None,
        }
    }
}

impl System<f64> for SystemMainGameMap {
    fn run(&mut self, arg: RunArg, _delta_time: f64) {
        let maps = arg.fetch(|w| w.read::<Map>());

        while let Some(event) = self.channel_game.try_recv() {
            match event {
                GameToMainGameMap::Cleanup => {
                    if let Some(map_entity) = self.entity_opt_map.take() {
                        if let Some(map) = maps.get(map_entity) {
                            for tile in map.get_tiles().values() {
                                arg.delete(*tile);
                            }
                        }
                        arg.delete(map_entity);
                    }
                    self.channel_game.send(GameFromMainGameMap::CleanupDone);
                }
                GameToMainGameMap::SetEntityMap(entity_map) => {
                    self.entity_opt_map = Some(entity_map);
                }
            }
        }
    }
}
