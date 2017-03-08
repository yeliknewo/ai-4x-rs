mod ng_glutin;
pub mod pipeline_spritesheet;
pub mod pipeline_text;
// mod ng_sdl2;
mod shaders;
mod texture;
mod types;
mod window;


pub use self::ng_glutin::Extras as GlutinExtras;
pub use self::ng_glutin::Window as GlutinWindow;
pub use self::ng_glutin::build_window as glutin_build_window;
pub use self::shaders::Shaders;
pub use self::texture::load_texture;
pub use self::types::{ColorFormat, DepthFormat, NGDevice, NGEncoder, NGFactory, NGResources, NGTexture, OutColor, OutDepth, RenderType, WindowSettings};
pub use self::window::GfxWindow;

pub use gfx::state::Rasterizer;
