use art::Color;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct RenderDataText {
    layer: u8,
    text: String,
    color: Color,
    spacing: f32,
}

impl RenderDataText {
    pub fn new(layer: u8, text: String, color: Color, spacing: f32) -> RenderDataText {
        RenderDataText {
            layer: layer,
            text: text,
            color: color,
            spacing: spacing,
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

    pub fn get_spacing(&self) -> f32 {
        self.spacing
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Component for RenderDataText {
    type Storage = VecStorage<RenderDataText>;
}
