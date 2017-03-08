use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct RenderDataSpritesheet {
    render_id_num: usize,
    default_tint: [f32; 4],
    tint: [f32; 4],
    layer: u8,
    spritesheet_rect: &'static [f32; 4],
    spritesheet_size: &'static [f32; 2],
    dirty_1: bool,
    dirty_2: bool,
}

impl RenderDataSpritesheet {
    pub fn new(render_id_num: usize, layer: u8, tint: [f32; 4], spritesheet_rect: &'static [f32; 4], spritesheet_size: &'static [f32; 2]) -> RenderDataSpritesheet {
        RenderDataSpritesheet {
            render_id_num: render_id_num,
            default_tint: tint.clone(),
            tint: tint,
            layer: layer,
            spritesheet_rect: spritesheet_rect,
            spritesheet_size: spritesheet_size,
            dirty_1: true,
            dirty_2: true,
        }
    }

    pub fn get_render_id_num(&self) -> usize {
        self.render_id_num
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.layer = layer;
        self.set_dirty();
    }

    pub fn set_spritesheet_rect(&mut self, spritesheet_rect: &'static [f32; 4]) {
        self.spritesheet_rect = spritesheet_rect;
        self.set_dirty();
    }

    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
        self.set_dirty();
    }

    pub fn reset_tint(&mut self) {
        self.tint = self.default_tint;
        self.set_dirty();
    }

    pub fn get_default_tint(&self) -> [f32; 4] {
        self.default_tint.clone()
    }

    pub fn get_layer(&self) -> u8 {
        self.layer
    }

    pub fn get_tint(&self) -> [f32; 4] {
        self.tint.clone()
    }

    pub fn get_spritesheet_rect(&self) -> [f32; 4] {
        self.spritesheet_rect.clone()
    }

    pub fn get_spritesheet_size(&self) -> [f32; 2] {
        self.spritesheet_size.clone()
    }

    fn set_dirty(&mut self) {
        self.dirty_1 = true;
        self.dirty_2 = true;
    }

    pub fn take_dirty(&mut self) -> bool {
        if self.dirty_1 {
            self.dirty_1 = false;
            true
        } else if self.dirty_2 {
            self.dirty_2 = false;
            true
        } else {
            false
        }
    }
}

impl Component for RenderDataSpritesheet {
    type Storage = VecStorage<RenderDataSpritesheet>;
}
