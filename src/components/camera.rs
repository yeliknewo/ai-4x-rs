use cgmath::{Matrix4, Point2, Point3, Vector3};
use specs::{Component, VecStorage};
use utils::OrthographicHelper;

#[derive(Debug)]
pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    ortho_helper: OrthographicHelper,
    is_main: bool,
    dirty_1: bool,
    dirty_2: bool,
}

impl Camera {
    pub fn new(eye: Point3<f32>, target: Point3<f32>, up: Vector3<f32>, ortho_helper: OrthographicHelper, is_main: bool) -> Camera {
        Camera {
            eye: eye,
            target: target,
            up: up,
            ortho_helper: ortho_helper,
            is_main: is_main,
            dirty_1: true,
            dirty_2: true,
        }
    }

    pub fn set_offset(&mut self, offset: Point2<f32>) {
        self.set_eye(Point3::new(offset.x, offset.y, 2.0));
        self.set_target(Point3::new(offset.x, offset.y, 0.0));
        self.set_dirty();
    }

    fn set_eye(&mut self, eye: Point3<f32>) {
        self.eye = eye;
    }

    fn set_target(&mut self, target: Point3<f32>) {
        self.target = target;
    }

    fn get_eye(&self) -> Point3<f32> {
        self.eye
    }

    fn get_target(&self) -> Point3<f32> {
        self.target
    }

    fn get_up(&self) -> Vector3<f32> {
        self.up
    }

    pub fn set_proj(&mut self, ortho_helper: OrthographicHelper) {
        self.ortho_helper = ortho_helper;
        self.set_dirty();
    }

    pub fn get_offset(&self) -> Point2<f32> {
        Point2::new(self.get_eye().x, self.get_eye().y)
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.get_eye(), self.get_target(), self.get_up())
    }

    pub fn get_proj(&self) -> Matrix4<f32> {
        self.ortho_helper.build_matrix()
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }

    pub fn screen_to_world_point(&self, screen_point: Point2<f32>) -> Point2<f32> {
        let view_depth = self.ortho_helper.get_view_depth();

        Point2::new((((screen_point.x * 2.0) - 1.0) * view_depth) * 4.0 / 5.0 + self.get_offset().x, (((1.0 - screen_point.y) * 2.0 - 1.0) * view_depth / self.ortho_helper.get_aspect_ratio()) * 4.0 / 5.0 + self.get_offset().y)
    }

    fn set_dirty(&mut self) {
        self.dirty_1 = true;
        self.dirty_2 = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        self.dirty_1 = false;
        if self.dirty_2 {
            self.dirty_2 = false;
            true
        } else {
            false
        }
    }
}

impl Component for Camera {
    type Storage = VecStorage<Camera>;
}
