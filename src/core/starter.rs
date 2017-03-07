use core::{Game, make_event_clumps};
use core::handle_events::glutin::handle_events;
use events::{MainFromRender, MainToRender};
use gfx::Device;
use std::thread;
use utils::OrthographicHelper;

pub fn start() {
    let (mut front_event_clump, back_event_clump) = make_event_clumps(0, 1, 2, 3);

    let title = "Twitch Game";

    let (width, height): (u32, u32) = (640, 480);

    use graphics::glutin_build_window as build_window;

    let mut gfx_window = build_window((title, width, height));

    {
        let mut render_event_core = front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none"));

        render_event_core.send(MainToRender::Encoder(gfx_window.get_mut_factory().create_command_buffer().into(), 0));
        render_event_core.send(MainToRender::Encoder(gfx_window.get_mut_factory().create_command_buffer().into(), 1));
    }

    let out_color = gfx_window.get_out_color().clone();
    let out_depth = gfx_window.get_out_depth().clone();

    let aspect_ratio = width as f32 / height as f32;
    let left = -10.0;
    let right = 10.0;
    let near = 0.0;
    let far = 10.0;

    let ortho = OrthographicHelper::new(aspect_ratio, left, right, near, far);

    let game = Game::new(gfx_window.get_mut_factory(), back_event_clump, ortho.as_ref().clone(), out_color, out_depth);

    let game_handle = thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });

    'main: loop {
        if let Some(event) = front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).try_recv() {
            match event {
                MainFromRender::Encoder(mut encoder, encoder_id) => {
                    if handle_events(&mut gfx_window, &mut front_event_clump) {
                        front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).send(MainToRender::Encoder(encoder, encoder_id));
                        break 'main;
                    }

                    encoder.flush(gfx_window.get_mut_device());
                    front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).send(MainToRender::Encoder(encoder, encoder_id));
                    gfx_window.swap_buffers();
                    gfx_window.get_mut_device().cleanup();
                }
            }
        }
    }

    // game_handle.join().unwrap_or_else(|err| panic!("Join Error: {:?}", err));
}
