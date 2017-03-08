mod square;
mod text;
mod types;
pub mod main;
pub mod layers;
pub mod colors;

pub use self::square::make_square_render;
pub use self::text::make_text_render;
pub use self::types::{Layer, Name, Size, Sprite, Tint, Color};
