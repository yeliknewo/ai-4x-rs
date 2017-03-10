use art;
use cgmath::{Euler, Point2, Point3, Rad, Vector3};
use components::{Button, Camera, RenderDataSpritesheet, RenderDataText, Transform};
use core::BackEventClump;
use events::{GameFromMainMenu, GameToMainMenu, MainFromGame, MainToGame};
use specs::{Planner, System, World};
use std::collections::HashMap;
use systems::{ControlSystem, FpsCounterSystem, MainMenuSystem, RenderSystem};
use time::precise_time_ns;
use utils::{DuoChannel, OrthographicHelper};

const SYSTEM_NAME_MAIN_MENU: &'static str = "main_menu";
const SYSTEM_NAME_RENDERER: &'static str = "renderer";
const SYSTEM_NAME_CONTROL: &'static str = "control";
const SYSTEM_NAME_FPS_COUNTER: &'static str = "fps_counter";
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
            world.register::<RenderDataSpritesheet>();
            world.register::<RenderDataText>();
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

        planner.add_system(ControlSystem::new(back_event_clump.take_main_x_control().unwrap_or_else(|| panic!("Main X Control was none")), screen_size), SYSTEM_NAME_CONTROL, 20);

        planner.add_system(MainMenuSystem::new(play_button, play_button_text, back_event_clump.take_back_game_x_main_menu().unwrap_or_else(|| panic!("Back Game X Main Menu was none"))), SYSTEM_NAME_MAIN_MENU, 15);

        planner.add_system(render_system, SYSTEM_NAME_RENDERER, 10);

        planner.add_system(FpsCounterSystem::new(WARN_FPS), SYSTEM_NAME_FPS_COUNTER, 5);

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
                    for y in -10..10i32 {
                        for x in -10..10i32 {
                            self.planner
                                .mut_world()
                                .create_now()
                                .with(Transform::new(Vector3::new(x as f32, y as f32, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
                                .with(RenderDataSpritesheet::new(self.main_render, art::layers::PLAYER, *art::main::DEFAULT_TINT, art::main::yellow::BLANK, art::main::SIZE))
                                .build();
                        }
                    }

                    // for x in -5..5i32 {
                    //     for y in -5..5i32 {
                    //         planner.mut_world()
                    //             .create_now()
                    //             .with(Transform::new(Vector3::new(x as f32, y as f32, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
                    //             .with(RenderDataSpritesheet::new(main_render, art::layers::PLAYER, *art::main::DEFAULT_TINT, art::main::yellow::BLANK, art::main::SIZE))
                    //             .with(Button::new())
                    //             .build();
                    //     }
                    // }
                    //add main game scene
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
