use cgmath::{Euler, Matrix4, Point2, Rad, Vector3};
use cgmath::prelude::Zero;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Transform {
    translation: Vector3<f32>,
    rotation: Euler<Rad<f32>>,
    scale: Vector3<f32>,
    matrix: Matrix4<f32>,
    dirty: bool,
}

impl Transform {
    pub fn new(translation: Vector3<f32>, rotation: Euler<Rad<f32>>, scale: Vector3<f32>) -> Transform {
        Transform {
            translation: translation,
            rotation: rotation,
            scale: scale,
            matrix: Transform::make_matrix(translation, rotation, scale),
            dirty: false,
        }
    }

    fn make_matrix(translation: Vector3<f32>, rotation: Euler<Rad<f32>>, scale: Vector3<f32>) -> Matrix4<f32> {
        Matrix4::from_translation(translation) * Matrix4::from(rotation) * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z)
    }

    pub fn new_identity() -> Transform {
        Transform::new(Vector3::zero(), Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)), Vector3::new(1.0, 1.0, 1.0))
    }

    pub fn set_pos(&mut self, pos: Vector3<f32>) {
        self.translation = pos;
        self.set_dirty();
    }

    pub fn add_pos(&mut self, pos_delta: Vector3<f32>) {
        self.translation += pos_delta;
        self.set_dirty();
    }

    pub fn get_pos(&self) -> Vector3<f32> {
        self.translation
    }

    pub fn update_model(&mut self) {
        if self.take_dirty() {
            self.matrix = Transform::make_matrix(self.translation, self.rotation, self.scale);
        }
    }

    pub fn get_updated_model(&mut self) -> Matrix4<f32> {
        self.update_model();
        self.matrix
    }

    pub fn get_model(&self) -> Matrix4<f32> {
        self.matrix
    }

    pub fn get_gui_offset(&self) -> Point2<f32> {
        let translation = self.get_pos();
        Point2::new(-translation.x, -translation.y)
    }

    fn set_dirty(&mut self) {
        self.dirty = true;
    }

    fn take_dirty(&mut self) -> bool {
        if self.dirty {
            self.dirty = false;
            true
        } else {
            false
        }
    }
}

impl Component for Transform {
    type Storage = VecStorage<Transform>;
}
