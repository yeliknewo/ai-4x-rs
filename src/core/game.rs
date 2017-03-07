use art;
use cgmath::{Euler, Point3, Rad, Vector3};
use components::{Camera, RenderData, RenderId, Transform};
use core::BackEventClump;
use find_folder::Search;
use graphics::{NGFactory, OutColor, OutDepth, load_texture};
use specs::{Planner, World};
use systems::{ControlSystem, RenderSystem};
use time::precise_time_ns;
use utils::{FpsCounter, OrthographicHelper};

pub struct Game {
    last_time: u64,
    planner: Planner<f64>,
    fps_counter: FpsCounter,
}

impl Game {
    pub fn new<ID>(factory: &mut NGFactory, mut back_event_clump: BackEventClump<ID>, ortho: OrthographicHelper, out_color: OutColor, out_depth: OutDepth) -> Game
        where ID: 'static + Eq + Send
    {
        let mut planner = {
            let mut world = World::new();

            world.register::<Transform>();
            world.register::<RenderId>();
            world.register::<Camera>();
            world.register::<RenderData>();

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

        for i in -10..10 {
            planner.mut_world()
                .create_now()
                .with(Transform::new(Vector3::new(i as f32, 0.0, 0.0), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0)))
                .with(main_render.clone())
                .with(RenderData::new(art::layers::PLAYER, *art::main::DEFAULT_TINT, art::main::BOX, art::main::SIZE))
                .build();
        }

        planner.add_system(ControlSystem::new(back_event_clump.take_control().unwrap_or_else(|| panic!("Control was none"))), "control", 15);

        planner.add_system(renderer, "renderer", 10);

        Game {
            last_time: precise_time_ns(),
            planner: planner,
            fps_counter: FpsCounter::new(50),
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = precise_time_ns();
        let delta = (new_time - self.last_time) as f64 / 1e9;
        self.last_time = new_time;

        self.planner.dispatch(delta);
        self.fps_counter.frame(delta);
        true
    }
}
