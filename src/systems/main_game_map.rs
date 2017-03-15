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
                    self.entity_opt_map = None;
                }
                GameToMainGameMap::SetEntityMap(entity_map) => {
                    self.entity_opt_map = Some(entity_map);
                }
            }
        }
    }
}
