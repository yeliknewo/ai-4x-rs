use gfx;
use gfx_device_gl;

pub type WindowSettings<'a> = (&'a str, u32, u32);
pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub type NGDevice = gfx_device_gl::Device;
pub type NGFactory = gfx_device_gl::Factory;
pub type NGResources = gfx_device_gl::Resources;
pub type NGCommandBuffer = gfx_device_gl::CommandBuffer;
pub type NGTexture = gfx::handle::ShaderResourceView<NGResources, [f32; 4]>;
pub type NGEncoder = gfx::Encoder<NGResources, NGCommandBuffer>;

pub type OutColor = gfx::handle::RenderTargetView<NGResources, ColorFormat>;
pub type OutDepth = gfx::handle::DepthStencilView<NGResources, DepthFormat>;
