use art::main::blue;
use cgmath::{MetricSpace, Point2, Vector2, Vector4};
use components::{Button, Camera, RenderDataSpritesheet, Transform};
use events::{MainFromControl, MainToControl};
use specs::{Join, RunArg, System};
use utils::DuoChannel;

#[derive(Debug)]
pub struct ControlSystem {
    main_channel: DuoChannel<MainFromControl, MainToControl>,
    mouse_pos: Point2<f32>,
    screen_size: Point2<f32>,
}

impl ControlSystem {
    pub fn new(main_channel: DuoChannel<MainFromControl, MainToControl>, screen_size: Point2<f32>) -> ControlSystem {
        ControlSystem {
            main_channel: main_channel,
            mouse_pos: Point2::new(0.0, 0.0),
            screen_size: screen_size,
        }
    }
}

impl System<f64> for ControlSystem {
    fn run(&mut self, arg: RunArg, _delta_time: f64) {
        let (cameras, transforms, mut buttons) = arg.fetch(|w| (w.read::<Camera>(), w.read::<Transform>(), w.write::<Button>()));

        let camera = {
            let mut camera_opt = None;

            for camera in (&cameras).iter() {
                if camera.is_main() {
                    camera_opt = Some(camera);
                    break;
                }
            }

            camera_opt.unwrap_or_else(|| panic!("No Main Camera Entity"))
        };

        let mut event_opt = self.main_channel.try_recv();
        while let Some(event) = event_opt.take() {
            match event {
                MainToControl::MouseButton(state, mouse_button) => {
                    let mouse_in_world = camera.screen_to_world_point(self.mouse_pos);

                    for (transform, mut button) in (&transforms, &mut buttons).iter() {
                        let bot_left = transform.get_model() * Vector4::new(button.get_bot_left().x, button.get_bot_left().y, 1.0, 1.0);

                        let top_right = transform.get_model() * Vector4::new(button.get_top_right().x, button.get_top_right().y, 1.0, 1.0);

                        if mouse_in_world.x >= bot_left.x && mouse_in_world.x <= top_right.x && mouse_in_world.y >= bot_left.y && mouse_in_world.y <= top_right.y {
                            button.set_button_state(mouse_button.clone(), state.clone());
                        }
                    }
                }
                MainToControl::MouseMoved(x, y) => {
                    self.mouse_pos.x = x as f32 / self.screen_size.x;
                    self.mouse_pos.y = y as f32 / self.screen_size.y;
                }
            }

            event_opt = self.main_channel.try_recv();
        }
    }
}
