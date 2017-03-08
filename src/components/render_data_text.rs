use art::Color;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct RenderDataText {
    layer: u8,
    text: String,
    color: Color,
}

impl RenderDataText {
    pub fn new(layer: u8, text: String, color: Color) -> RenderDataText {
        RenderDataText {
            layer: layer,
            text: text,
            color: color,
        }
    }

    pub fn get_layer(&self) -> u8 {
        self.layer
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Component for RenderDataText {
    type Storage = VecStorage<RenderDataText>;
}
