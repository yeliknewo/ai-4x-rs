use art::main::{blue, yellow};
use cgmath::{MetricSpace, Point2, Vector2};
use components::{Button, Camera, RenderData, Transform};
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
        let (cameras, transforms, mut buttons, mut render_datas) = arg.fetch(|w| (w.read::<Camera>(), w.read::<Transform>(), w.write::<Button>(), w.write::<RenderData>()));

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

                    for (transform, mut button, mut render_data) in (&transforms, &mut buttons, &mut render_datas).iter() {
                        let pos = transform.get_pos();

                        let pos2 = Point2::new(pos.x, pos.y) + Vector2::new(0.5, 0.5);

                        if mouse_in_world.distance(pos2) < 0.5 {
                            button.set_button_state(mouse_button.clone(), state.clone());
                            render_data.set_spritesheet_rect(blue::X);
                        } else {
                            //render_data.set_spritesheet_rect(yellow::BLANK);
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
