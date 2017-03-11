use art;
use cgmath::{Euler, Point2, Point3, Rad, Vector2, Vector3};
use components::{Button, Camera, City, Map, RenderDataSpritesheet, RenderDataText, Tile, Transform};
use core::BackEventClump;
use events::{GameFromMainMenu, GameToMainMenu, MainFromGame, MainToGame};
use specs::{Planner, System, World};
use std::collections::HashMap;
use systems::{CitySystem, ControlSystem, FpsCounterSystem, MainMenuSystem, RenderSystem};
use time::precise_time_ns;
use utils::{DuoChannel, OrthographicHelper};

const SYSTEM_NAME_MAIN_MENU: &'static str = "main_menu";
const SYSTEM_NAME_RENDERER: &'static str = "renderer";
const SYSTEM_NAME_CONTROL: &'static str = "control";
const SYSTEM_NAME_FPS_COUNTER: &'static str = "fps_counter";
const SYSTEM_NAME_CITY: &'static str = "city";

const SYSTEM_PRIORITY_MAIN_MENU: i32 = 15;
const SYSTEM_PRIORITY_RENDERER: i32 = 10;
const SYSTEM_PRIORITY_CONTROL: i32 = 20;
const SYSTEM_PRIORITY_FPS_COUNTER: i32 = 5;
const SYSTEM_PRIORITY_CITY: i32 = 15;

const WARN_FPS: u32 = 55;
const TARGET_FPS: f64 = 60.0;

pub struct Game {
    last_time: u64,
    planner: Planner<f64>,
    main_channel: DuoChannel<MainFromGame, MainToGame>,
    main_menu_channel: DuoChannel<GameToMainMenu, GameFromMainMenu>,
    inactive_systems: HashMap<&'static str, Box<System<f64>>>,
    main_render: usize,
}

impl Game {
    pub fn new<ID>(render_system: RenderSystem<u64>, main_render: usize, mut back_event_clump: BackEventClump<ID>, ortho: OrthographicHelper, screen_size: Point2<f32>) -> Game
        where ID: 'static + Eq + Send
    {
        let mut planner = {
            let mut world = World::new();

            world.register::<Button>();
            world.register::<Camera>();
            world.register::<City>();
            world.register::<Map>();
            world.register::<RenderDataSpritesheet>();
            world.register::<RenderDataText>();
            world.register::<Tile>();
            world.register::<Transform>();

            Planner::<f64>::new(world)
        };

        planner.mut_world().create_now().with(Camera::new(Point3::new(0.0, 0.0, 2.0), Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), ortho, true)).build();

        let play_button = planner.mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(-0.5 * 1.2, -2.0 * 1.2, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(5.0 * 1.2, 5.0 * 1.2, 1.0)))
            .with(RenderDataSpritesheet::new(main_render, art::layers::GUI, *art::main::DEFAULT_TINT, art::main::grey::BLANK, art::main::SIZE))
            .with(Button::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)))
            .build();

        let play_button_text = planner.mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(RenderDataText::new(art::layers::GUI, "play".into(), art::colors::WHITE, 1.2))
            .build();

        planner.add_system(ControlSystem::new(back_event_clump.take_main_x_control().unwrap_or_else(|| panic!("Main X Control was none")), screen_size), SYSTEM_NAME_CONTROL, SYSTEM_PRIORITY_CONTROL);

        planner.add_system(MainMenuSystem::new(play_button, play_button_text, back_event_clump.take_back_game_x_main_menu().unwrap_or_else(|| panic!("Back Game X Main Menu was none"))), SYSTEM_NAME_MAIN_MENU, SYSTEM_PRIORITY_MAIN_MENU);

        planner.add_system(render_system, SYSTEM_NAME_RENDERER, SYSTEM_PRIORITY_RENDERER);

        planner.add_system(FpsCounterSystem::new(WARN_FPS), SYSTEM_NAME_FPS_COUNTER, SYSTEM_PRIORITY_FPS_COUNTER);

        Game {
            last_time: precise_time_ns(),
            main_channel: back_event_clump.take_main_x_game().unwrap_or_else(|| panic!("Main X Game was none")),
            main_menu_channel: back_event_clump.take_front_game_x_main_menu().unwrap_or_else(|| panic!("Front Game X Main Menu was none")),
            inactive_systems: HashMap::new(),
            planner: planner,
            main_render: main_render,
        }
    }

    pub fn frame(&mut self) -> bool {
        while let Some(event) = self.main_channel.try_recv() {
            match event {
                MainToGame::Exit => {
                    self.main_channel.send(MainFromGame::Exited);
                    return false;
                }
            }
        }
        while let Some(event) = self.main_menu_channel.try_recv() {
            match event {
                GameFromMainMenu::CreateMainGameScene => {
                    for i in 0..self.planner.systems.len() {
                        if self.planner.systems[i].name == SYSTEM_NAME_MAIN_MENU {
                            self.inactive_systems.insert(SYSTEM_NAME_MAIN_MENU, self.planner.systems.remove(i).object);
                            break;
                        }
                    }

                    let min = Vector2::new(-10, -10);
                    let max = Vector2::new(10, 10);

                    let map = self.planner.mut_world().create_now().with(Map::new(min, max)).build();

                    for y in min.y..max.y {
                        for x in min.x..max.x {
                            let food = 1;
                            let iron = 1;
                            let gold = 1;

                            let tile = self.planner
                                .mut_world()
                                .create_now()
                                .with(Transform::new(Vector3::new(x as f32, y as f32, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
                                .with(RenderDataSpritesheet::new(self.main_render, art::layers::PLAYER, *art::main::DEFAULT_TINT, art::main::yellow::BLANK, art::main::SIZE))
                                .with(Tile::new(food, iron, gold))
                                .build();

                            self.planner.mut_world().write_w_comp_id::<Map>(()).get_mut(map).unwrap_or_else(|| panic!("Unable to Get Mut Map")).set_tile(Vector2::new(x, y), tile);
                        }
                    }

                    self.planner
                        .mut_world()
                        .create_now()
                        .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
                        .with(RenderDataSpritesheet::new(self.main_render, art::layers::PLAYER, *art::main::DEFAULT_TINT, art::main::green::STAR, art::main::SIZE))
                        .with(City::new(Vector2::new(0, 0), 1))
                        .build();

                    self.planner.add_system(CitySystem::new(map), SYSTEM_NAME_CITY, SYSTEM_PRIORITY_CITY);
                }
            }
        }
        let new_time = precise_time_ns();
        let delta = (new_time - self.last_time) as f64 / 1e9;
        if delta <= 1.0 / TARGET_FPS {
            true
        } else {
            self.last_time = new_time;

            self.planner.dispatch(delta);
            true
        }
    }
}
