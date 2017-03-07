mod pipeline;
mod ng_glutin;
mod ng_sdl2;
mod shaders;
mod texture;
mod window;
mod types;


pub use self::ng_glutin::Extras as GlutinExtras;
pub use self::ng_glutin::Window as GlutinWindow;
pub use self::ng_glutin::build_window as glutin_build_window;

pub use self::pipeline::{Bundle, Packet, ProjectionData, TextureData, Vertex, make_shaders, pipe};
pub use self::shaders::Shaders;
pub use self::texture::load_texture;

pub use self::types::{ColorFormat, DepthFormat, NGDevice, NGEncoder, NGFactory, NGResources, NGTexture, OutColor, OutDepth, WindowSettings};

pub use self::window::GfxWindow;

pub use gfx::state::Rasterizer;
