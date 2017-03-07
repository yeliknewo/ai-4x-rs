use art;
use cgmath::{Euler, Point3, Rad, Vector3};
use components::{Button, Camera, RenderData, RenderId, Transform};
use core::BackEventClump;
use events::{MainFromGame, MainToGame};
use find_folder::Search;
use graphics::{NGFactory, OutColor, OutDepth, load_texture};
use specs::{Planner, World};
use systems::{ControlSystem, FpsCounterSystem, RenderSystem};
use time::precise_time_ns;
use utils::{DuoChannel, OrthographicHelper};

pub struct Game {
    last_time: u64,
    planner: Planner<f64>,
    main_channel: DuoChannel<MainFromGame, MainToGame>,
}

impl Game {
    pub fn new<ID>(factory: &mut NGFactory, mut back_event_clump: BackEventClump<ID>, ortho: OrthographicHelper, out_color: OutColor, out_depth: OutDepth) -> Game
        where ID: 'static + Eq + Send
    {
        let mut planner = {
            let mut world = World::new();

            world.register::<Button>();
            world.register::<Camera>();
            world.register::<RenderData>();
            world.register::<RenderId>();
            world.register::<Transform>();

            Planner::<f64>::new(world, 8)
        };

        let mut renderer = RenderSystem::new(back_event_clump.take_render().unwrap_or_else(|| panic!("Render was none")), out_color, out_depth);

        planner.mut_world().create_now().with(Camera::new(Point3::new(0.0, 0.0, 2.0), Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), ortho, true)).build();

        let packet = art::make_square_render();

        let assets = Search::ParentsThenKids(5, 3)
            .for_folder("assets")
            .unwrap_or_else(|err| panic!("Did you forget to make an assets folder? Err: {:?}", err));

        let main_render = {
            let texture = load_texture(factory, assets.join(art::main::NAME));
            renderer.add_render(factory, &packet, texture)
        };

        for x in -10..10 {
            for y in -10..10 {
                planner.mut_world()
                    .create_now()
                    .with(Transform::new(Vector3::new(x as f32, y as f32, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
                    .with(main_render.clone())
                    .with(RenderData::new(art::layers::PLAYER, *art::main::DEFAULT_TINT, art::main::yellow::BLANK, art::main::SIZE))
                    .build();
            }
        }

        planner.add_system(ControlSystem::new(back_event_clump.take_control().unwrap_or_else(|| panic!("Control was none"))), "control", 15);

        planner.add_system(renderer, "renderer", 10);

        planner.add_system(FpsCounterSystem::new(55), "fps_counter", 5);

        Game {
            last_time: precise_time_ns(),
            main_channel: back_event_clump.take_game().unwrap_or_else(|| panic!("Game was none")),
            planner: planner,
        }
    }

    pub fn frame(&mut self) -> bool {
        if let Some(event) = self.main_channel.try_recv() {
            match event {
                MainToGame::Exit => {
                    self.main_channel.send(MainFromGame::Exited);
                    return false;
                }
            }
        }
        let new_time = precise_time_ns();
        let delta = (new_time - self.last_time) as f64 / 1e9;
        if delta <= 1.0 / 60.0 {
            true
        } else {
            self.last_time = new_time;

            self.planner.dispatch(delta);
            true
        }
    }
}
