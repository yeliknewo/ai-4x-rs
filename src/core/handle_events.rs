
pub mod glutin {
    use core::FrontEventClump;
    use events::MainToControl;
    use glutin::{self, ElementState, Event, VirtualKeyCode};
    use graphics::{GfxWindow, GlutinExtras, GlutinWindow, NGDevice, NGFactory, NGResources};
    use utils::{ButtonState, MouseButton};

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
                Event::MouseMoved(x, y) => {
                    front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send(MainToControl::MouseMoved(x, y));
                }
                Event::MouseInput(state, button) => {
                    let new_state = match state {
                        ElementState::Pressed => ButtonState::Pressed,
                        ElementState::Released => ButtonState::Released,
                    };

                    let new_button_opt = match button {
                        glutin::MouseButton::Left => Some(MouseButton::Left),
                        glutin::MouseButton::Right => Some(MouseButton::Right),
                        _ => None,
                    };

                    if let Some(new_button) = new_button_opt {
                        front_event_clump.get_mut_control().unwrap_or_else(|| panic!("Control was none")).send(MainToControl::MouseButton(new_state, new_button));
                    }
                }
                Event::Closed => return true,
                _ => (),
            }
        }

        false
    }
}
