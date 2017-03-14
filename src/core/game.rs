use art;
use cgmath::{Euler, Point2, Point3, Rad, Vector2, Vector3};
use components::{Button, Camera, City, Map, RenderDataSpritesheet, RenderDataText, Tile, Transform};
use core::BackEventClump;
use events::{GameFromMainGameCity, GameFromMainGameMap, GameFromMainGamePauseMenu, GameFromMainMenu, GameToMainGameCity, GameToMainGameMap, GameToMainGamePauseMenu, GameToMainMenu, MainFromGame, MainToGame};
use specs::{Planner, System, SystemInfo, World};
use std::collections::HashMap;
use systems::{SystemControl, SystemFpsCounter, SystemMainGameCity, SystemMainGameMap, SystemMainGamePauseMenu, SystemMainMenu, SystemRender};
use time::precise_time_ns;
use utils::{DuoChannel, OrthographicHelper};

// const SYSTEM_NAME_MAIN_MENU: &'static str = "main_menu";
// const SYSTEM_NAME_RENDERER: &'static str = "renderer";
// const SYSTEM_NAME_CONTROL: &'static str = "control";
// const SYSTEM_NAME_FPS_COUNTER: &'static str = "fps_counter";
// const SYSTEM_NAME_CITY: &'static str = "city";
// const SYSTEM_NAME_MAIN_GAME_PAUSE_MENU: &'static str = "main_game_pause_menu";
//
// const SYSTEM_PRIORITY_MAIN_MENU: i32 = 15;
// const SYSTEM_PRIORITY_RENDERER: i32 = 10;
// const SYSTEM_PRIORITY_CONTROL: i32 = 100;
// const SYSTEM_PRIORITY_FPS_COUNTER: i32 = 5;
// const SYSTEM_PRIORITY_CITY: i32 = 15;
// const SYSTEM_PRIORITY_MAIN_GAME_PAUSE_MENU: i32 = 20;

type SystemIdent = &'static (&'static str, i32);

const SYSTEM_MAIN_MENU: SystemIdent = &("main_menu", 15);
const SYSTEM_RENDERER: SystemIdent = &("renderer", 10);
const SYSTEM_CONTROL: SystemIdent = &("control", 100);
const SYSTEM_FPS_COUNTER: SystemIdent = &("fps_counter", 5);
const SYSTEM_MAIN_GAME_CITY: SystemIdent = &("city", 15);
const SYSTEM_MAIN_GAME_PAUSE_MENU: SystemIdent = &("main_game_pause_menu", 20);
const SYSTEM_MAIN_GAME_MAP: SystemIdent = &("main_game_map", 16);

const WARN_FPS: u32 = 55;
const TARGET_FPS: f64 = 60.0;

pub struct Game {
    last_time: u64,
    planner: Planner<f64>,
    channel_main: DuoChannel<MainFromGame, MainToGame>,
    channel_main_menu: DuoChannel<GameToMainMenu, GameFromMainMenu>,
    channel_main_game_pause_menu: DuoChannel<GameToMainGamePauseMenu, GameFromMainGamePauseMenu>,
    channel_main_game_map: DuoChannel<GameToMainGameMap, GameFromMainGameMap>,
    channel_main_game_city: DuoChannel<GameToMainGameCity, GameFromMainGameCity>,
    inactive_systems: HashMap<SystemIdent, Box<System<f64>>>,
    main_render: usize,
}

impl Game {
    pub fn new<ID>(render_system: SystemRender<u64>, main_render: usize, mut back_event_clump: BackEventClump<ID>, ortho: OrthographicHelper, screen_size: Point2<f32>) -> Game
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

        planner.add_system(SystemControl::new(back_event_clump.take_main_x_control().unwrap_or_else(|| panic!("Main X Control was none")), screen_size), SYSTEM_CONTROL.0, SYSTEM_CONTROL.1);

        planner.add_system(render_system, SYSTEM_RENDERER.0, SYSTEM_RENDERER.1);

        planner.add_system(SystemFpsCounter::new(WARN_FPS), SYSTEM_FPS_COUNTER.0, SYSTEM_FPS_COUNTER.1);

        let mut inactive_systems: HashMap<SystemIdent, Box<System<f64>>> = HashMap::new();

        inactive_systems.insert(SYSTEM_MAIN_GAME_PAUSE_MENU, Box::new(SystemMainGamePauseMenu::new(back_event_clump.take_back_game_x_main_game_pause_menu().unwrap_or_else(|| panic!("Back Game X Main game Pause Menu was none")))));
        inactive_systems.insert(SYSTEM_MAIN_MENU, Box::new(SystemMainMenu::new(back_event_clump.take_back_game_x_main_menu().unwrap_or_else(|| panic!("Back Game X Main Menu was none")))));
        inactive_systems.insert(SYSTEM_MAIN_GAME_CITY, Box::new(SystemMainGameCity::new(back_event_clump.take_back_game_x_main_game_city().unwrap_or_else(|| panic!("Back Game X Main Game City was none")))));
        inactive_systems.insert(SYSTEM_MAIN_GAME_MAP, Box::new(SystemMainGameMap::new(back_event_clump.take_back_game_x_main_game_map().unwrap_or_else(|| panic!("Back Game X Main Game Map was none")))));

        let mut game = Game {
            last_time: precise_time_ns(),
            channel_main: back_event_clump.take_main_x_game().unwrap_or_else(|| panic!("Main X Game was none")),
            channel_main_menu: back_event_clump.take_front_game_x_main_menu().unwrap_or_else(|| panic!("Front Game X Main Menu was none")),
            channel_main_game_pause_menu: back_event_clump.take_front_game_x_main_game_pause_menu().unwrap_or_else(|| panic!("Front Game X Main Game Pause Menu was none")),
            channel_main_game_map: back_event_clump.take_front_game_x_main_game_map().unwrap_or_else(|| panic!("Front Game X Main Game Map was none")),
            channel_main_game_city: back_event_clump.take_front_game_x_main_game_city().unwrap_or_else(|| panic!("Front Game X Main Game City was none")),
            inactive_systems: inactive_systems,
            planner: planner,
            main_render: main_render,
        };
        game.create_main_menu_scene();

        game
    }

    fn create_main_game_scene(&mut self) {
        self.activate_system(SYSTEM_MAIN_GAME_CITY);
        self.activate_system(SYSTEM_MAIN_GAME_PAUSE_MENU);
        self.activate_system(SYSTEM_MAIN_GAME_MAP);

        let min = Vector2::new(-10, -10);
        let max = Vector2::new(10, 10);

        let map = self.planner.mut_world().create_now().with(Map::new(min, max)).build();

        self.channel_main_game_map.send(GameToMainGameMap::SetEntityMap(map));
        self.channel_main_game_city.send(GameToMainGameCity::SetEntityMap(map));

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

        let city = self.planner
            .mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(RenderDataSpritesheet::new(self.main_render, art::layers::PLAYER, *art::main::DEFAULT_TINT, art::main::green::STAR, art::main::SIZE))
            .with(City::new(Vector2::new(0, 0), 1))
            .build();

        self.channel_main_game_city.send(GameToMainGameCity::SetEntityCity(city));

        let exit_x_offset = -15.0;
        let exit_y_offset = 15.0;

        let exit_button = self.planner
            .mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(-0.5 * 1.2 + exit_x_offset, -2.0 * 1.2 + exit_y_offset, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(5.0 * 1.2, 5.0 * 1.2, 1.0)))
            .with(RenderDataSpritesheet::new(self.main_render, art::layers::GUI, *art::main::DEFAULT_TINT, art::main::red::BLANK, art::main::SIZE))
            .with(Button::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)))
            .build();

        let exit_button_text = self.planner
            .mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(exit_x_offset, exit_y_offset, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(RenderDataText::new(art::layers::GUI, "exit".into(), art::colors::WHITE, 1.2))
            .build();

        self.channel_main_game_pause_menu.send(GameToMainGamePauseMenu::SetEntitiesExitButton(exit_button, exit_button_text));
    }

    fn cleanup_main_game_scene(&mut self) {
        self.channel_main_game_pause_menu.send(GameToMainGamePauseMenu::Cleanup);
        self.channel_main_game_map.send(GameToMainGameMap::Cleanup);
    }

    fn exit_main_game_scene(&mut self) {
        self.deactivate_system(SYSTEM_MAIN_GAME_PAUSE_MENU);
        self.deactivate_system(SYSTEM_MAIN_GAME_CITY);
        self.deactivate_system(SYSTEM_MAIN_GAME_MAP);
    }

    fn create_main_menu_scene(&mut self) {
        self.activate_system(SYSTEM_MAIN_MENU);

        let play_button = self.planner
            .mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(-0.5 * 1.2, -2.0 * 1.2, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(5.0 * 1.2, 5.0 * 1.2, 1.0)))
            .with(RenderDataSpritesheet::new(self.main_render, art::layers::GUI, *art::main::DEFAULT_TINT, art::main::grey::BLANK, art::main::SIZE))
            .with(Button::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)))
            .build();

        let play_button_text = self.planner
            .mut_world()
            .create_now()
            .with(Transform::new(Vector3::new(0.0, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
            .with(RenderDataText::new(art::layers::GUI, "play".into(), art::colors::WHITE, 1.2))
            .build();

        self.channel_main_menu.send(GameToMainMenu::SetEntitiesPlayButton(play_button, play_button_text));
    }

    fn cleanup_main_menu_scene(&mut self) {
        self.channel_main_menu.send(GameToMainMenu::Cleanup);
    }

    fn exit_main_menu_scene(&mut self) {
        self.deactivate_system(SYSTEM_MAIN_MENU);
    }

    fn deactivate_system(&mut self, system_ident: SystemIdent) {
        for i in 0..self.planner.systems.len() {
            if self.planner.systems[i].name == system_ident.0 {
                self.inactive_systems.insert(system_ident, self.planner.systems.remove(i).object);
                break;
            }
        }
    }

    fn activate_system(&mut self, system_ident: SystemIdent) {
        if let Some(system) = self.inactive_systems.remove(system_ident) {
            self.planner.systems.push(SystemInfo {
                name: system_ident.0.into(),
                priority: system_ident.1,
                object: system,
            });
        }
    }

    pub fn frame(&mut self) -> bool {
        while let Some(event) = self.channel_main.try_recv() {
            match event {
                MainToGame::Exit => {
                    self.channel_main.send(MainFromGame::Exited);
                    return false;
                }
            }
        }
        while let Some(event) = self.channel_main_menu.try_recv() {
            match event {
                GameFromMainMenu::CreateMainGameScene => {
                    self.cleanup_main_menu_scene();
                }
                GameFromMainMenu::CleanupDone => {
                    self.exit_main_menu_scene();
                    self.create_main_game_scene();
                }
            }
        }
        while let Some(event) = self.channel_main_game_pause_menu.try_recv() {
            match event {
                GameFromMainGamePauseMenu::CreateMainMenuScene => {
                    self.cleanup_main_game_scene();
                }
                GameFromMainGamePauseMenu::CleanupDone => {
                    self.exit_main_game_scene();
                    self.create_main_menu_scene();
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
