use cgmath::{Matrix4, Ortho};

#[derive(Clone, Debug)]
pub struct OrthographicHelper {
    aspect_ratio: f32,
    ortho: Ortho<f32>,
}

impl OrthographicHelper {
    pub fn new(aspect_ratio: f32, left: f32, right: f32, near: f32, far: f32) -> OrthographicHelper {
        let bottom = left * aspect_ratio;
        let top = right * aspect_ratio;

        OrthographicHelper {
            aspect_ratio: aspect_ratio,
            ortho: Ortho {
                left: left,
                right: right,
                bottom: bottom,
                top: top,
                near: near,
                far: far,
            },
        }
    }

    pub fn get_left(&self) -> f32 {
        self.ortho.left
    }

    pub fn get_right(&self) -> f32 {
        self.ortho.right
    }

    pub fn get_bottom(&self) -> f32 {
        self.ortho.bottom
    }

    pub fn get_top(&self) -> f32 {
        self.ortho.top
    }

    pub fn get_near(&self) -> f32 {
        self.ortho.near
    }

    pub fn get_far(&self) -> f32 {
        self.ortho.far
    }

    pub fn get_view_depth(&self) -> f32 {
        self.get_far() - self.get_near()
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn build_matrix(&self) -> Matrix4<f32> {
        Matrix4::from(self.ortho)
    }
}

impl AsRef<OrthographicHelper> for OrthographicHelper {
    fn as_ref(&self) -> &OrthographicHelper {
        self
    }
}
