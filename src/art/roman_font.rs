use art::{Name, RenderType, Size, Tint};

pub const NAME: Name = "font.png";
pub const SIZE: Size = &[930.0, 161.0];
pub const DEFAULT_TINT: Tint = &[1.0, 1.0, 1.0, 1.0];
pub const ID: RenderType = 1;

pub mod alphabet {
    use art::Sprite;

    pub const A: Sprite = &[8.0, 3.0, 16.0, 15.5];
    pub const B: Sprite = &[27.0, 3.0, 13.0, 15.5];
}
