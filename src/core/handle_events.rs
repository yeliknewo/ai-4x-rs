
pub mod glutin {
    use core::FrontEventClump;
    use glutin::{Event, VirtualKeyCode};
    use graphics::{GfxWindow, GlutinExtras, GlutinWindow, NGDevice, NGFactory, NGResources};

    pub fn handle_events<ID>(gfx_window: &mut GfxWindow<GlutinWindow, GlutinExtras, NGDevice, NGFactory, NGResources>, front_event_clump: &mut FrontEventClump<ID>) -> bool
        where ID: Send + Eq
    {
        while let Some(event) = gfx_window.get_mut_window().poll_events().next() {
            match event {
                Event::KeyboardInput(_, _, Some(key_code)) => {
                    match key_code {
                        VirtualKeyCode::Escape => return true,
                        _ => (),
                    }
                }
                Event::Closed => return true,
                _ => (),
            }
        }

        false
    }
}
