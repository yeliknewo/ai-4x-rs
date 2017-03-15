use art;
use cgmath::Vector2;
use components::{City, Map, RenderDataSpritesheet, Tile};
use events::{GameFromMainGameCity, GameToMainGameCity};
use specs::{Entity, Join, RunArg, System};
use utils::DuoChannel;

#[derive(Debug)]
pub struct SystemMainGameCity {
    channel_game: DuoChannel<GameFromMainGameCity, GameToMainGameCity>,
    entity_opt_map: Option<Entity>,
    entity_opt_city: Option<Entity>,
}

impl SystemMainGameCity {
    pub fn new(channel_game: DuoChannel<GameFromMainGameCity, GameToMainGameCity>) -> SystemMainGameCity {
        SystemMainGameCity {
            channel_game: channel_game,
            entity_opt_map: None,
            entity_opt_city: None,
        }
    }
}


impl System<f64> for SystemMainGameCity {
    fn run(&mut self, arg: RunArg, _delta_time: f64) {
        let (mut cities, maps, tiles, mut render_datas_spritesheet) = arg.fetch(|w| (w.write::<City>(), w.read::<Map>(), w.read::<Tile>(), w.write::<RenderDataSpritesheet>()));

        while let Some(event) = self.channel_game.try_recv() {
            match event {
                GameToMainGameCity::Cleanup => {}
                GameToMainGameCity::SetEntityCity(entity_city) => {
                    self.entity_opt_city = Some(entity_city);
                }
                GameToMainGameCity::SetEntityMap(entity_map) => {
                    self.entity_opt_map = Some(entity_map);
                }
            }
        }

        if let Some(entity_map) = self.entity_opt_map {
            if let Some(map) = maps.get(entity_map) {
                for (mut city, mut render_datas_spritesheet) in (&mut cities, &mut render_datas_spritesheet).join() {
                    let city_pos = city.get_pos();
                    let city_size = city.get_size();
                    let city_range = city.get_range();
                    let city_pop = city.get_pop();

                    let mut food = 0;
                    let mut iron = 0;
                    let mut gold = 0;

                    for y in -city_range..city_range {
                        for x in -city_range..city_range {
                            let tile_pos = city_pos + Vector2::new(x, y);

                            if let Some(tile_entity) = map.get_tile(&tile_pos) {
                                let tile = tiles.get(*tile_entity).unwrap_or_else(|| panic!("Tile Entity wasn't in Tiles"));

                                food += tile.get_food();
                                iron += tile.get_iron();
                                gold += tile.get_gold();
                            }
                        }
                    }

                    city.set_range(gold);
                    city.set_pop(food as u32);
                    city.set_size(iron as usize);

                    render_datas_spritesheet.set_spritesheet_rect(art::main::green::ARRAY[city.get_size() & art::main::green::ARRAY.len()]);
                }
            } else {
                self.entity_opt_map = None;
            }
        }
    }
}
