use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct RenderDataText {
    layer: u8,
    text: String,
}

impl RenderDataText {
    pub fn new(layer: u8, text: String) -> RenderDataText {
        RenderDataText {
            layer: layer,
            text: text,
        }
    }

    pub fn get_layer(&self) -> u8 {
        self.layer
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Component for RenderDataText {
    type Storage = VecStorage<RenderDataText>;
}
