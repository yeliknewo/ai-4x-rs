use art;
use cgmath::Point2;
use core::{Game, make_event_clumps};
use core::handle_events::glutin::handle_events;
use events::{MainFromGame, MainFromRender, MainToGame, MainToRender};
use find_folder::Search;
use gfx::Device;
use graphics::load_texture;
use std::thread;
use systems::RenderSystem;
use utils::OrthographicHelper;

const VALID_CHARACTERS: &'static str = "abcdefghijklmnopqrstuvwxyz1234567890";

pub fn start() {
    let (mut front_event_clump, mut back_event_clump) = make_event_clumps();

    let (width, height): (u32, u32) = (640, 480);

    let mut gfx_window = {
        let title = "Twitch Game";

        use graphics::glutin_build_window as build_window;

        build_window((title, width, height))
    };

    let game_handle = {
        let screen_size = Point2::new(width as f32, height as f32);

        {
            let mut render_event_core = front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none"));

            render_event_core.send(MainToRender::Encoder(gfx_window.get_mut_factory().create_command_buffer().into(), 0));
            render_event_core.send(MainToRender::Encoder(gfx_window.get_mut_factory().create_command_buffer().into(), 1));
        }

        let out_color = gfx_window.get_out_color().clone();
        let out_depth = gfx_window.get_out_depth().clone();

        let aspect_ratio = width as f32 / height as f32;
        let left = -20.0;
        let right = 20.0;
        let near = 0.0;
        let far = 10.0;

        let ortho = OrthographicHelper::new(aspect_ratio, left, right, near, far);

        let mut renderer = RenderSystem::new(back_event_clump.take_main_x_render().unwrap_or_else(|| panic!("Main X Render was none")), out_color, out_depth);

        let packet_square = art::make_square_render();

        let assets = Search::ParentsThenKids(2, 2)
            .for_folder("assets")
            .unwrap_or_else(|err| panic!("Did you forget to make an assets folder? Err: {:?}", err));

        let factory = gfx_window.get_mut_factory();

        let main_render = {
            let texture = load_texture(factory, assets.join(art::main::NAME));
            renderer.add_render_spritesheet(factory, &packet_square, texture)
        };

        for character in VALID_CHARACTERS.chars() {
            let packet = art::make_text_render(character);
            renderer.add_render_text(factory, &packet, character);
        }

        thread::spawn(move || {
            let main_render = main_render.clone();
            let mut game = Game::new(renderer, main_render, back_event_clump, ortho.as_ref().clone(), screen_size);
            while game.frame() {}
        })
    };

    'main: loop {
        if let Some(event) = front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).try_recv() {
            match event {
                MainFromRender::Encoder(mut encoder, encoder_id) => {
                    if handle_events(&mut gfx_window, &mut front_event_clump) {
                        front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).send(MainToRender::Exit);
                        continue;
                    }

                    encoder.flush(gfx_window.get_mut_device());
                    front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).send(MainToRender::Encoder(encoder, encoder_id));
                    gfx_window.swap_buffers();
                    gfx_window.get_mut_device().cleanup();
                }
                MainFromRender::Exited => {
                    front_event_clump.get_mut_game().unwrap_or_else(|| panic!("Game was none")).send(MainToGame::Exit);
                }
            }
        }
        if let Some(event) = front_event_clump.get_mut_game().unwrap_or_else(|| panic!("Game was none")).try_recv() {
            match event {
                MainFromGame::Exited => {
                    break 'main;
                }
            }
        }
    }

    game_handle.join().unwrap_or_else(|err| panic!("Join Error: {:?}", err));
}
