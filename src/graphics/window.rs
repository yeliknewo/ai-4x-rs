use gfx::Resources;
use gfx::handle::{DepthStencilView, RenderTargetView};
use graphics::{ColorFormat, DepthFormat, ng_glutin, ng_sdl2};

pub struct GfxWindow<W, T, D, F, R>
    where R: Resources
{
    out_color: RenderTargetView<R, ColorFormat>,
    out_depth: DepthStencilView<R, DepthFormat>,
    device: D,
    factory: F,
    window: W,
    extras: T,
}

impl<T, D, F, R> GfxWindow<ng_glutin::Window, T, D, F, R>
    where R: Resources
{
    pub fn swap_buffers(&mut self) {
        self.get_mut_window().swap_buffers().unwrap_or_else(|err| panic!("{:?}", err));
    }
}

// impl<T, D, F, R> GfxWindow<ng_sdl2::Window, T, D, F, R>
//     where R: Resources
// {
//     pub fn swap_buffers(&mut self) {
//         self.get_mut_window().gl_swap_buffers();
//     }
// }

impl<W, T, D, F, R> GfxWindow<W, T, D, F, R>
    where R: Resources
{
    pub fn new(out_color: RenderTargetView<R, ColorFormat>, out_depth: DepthStencilView<R, DepthFormat>, device: D, factory: F, window: W, extras: T) -> GfxWindow<W, T, D, F, R> {
        GfxWindow {
            out_color: out_color,
            out_depth: out_depth,
            device: device,
            factory: factory,
            window: window,
            extras: extras,
        }
    }

    pub fn get_out_color(&self) -> &RenderTargetView<R, ColorFormat> {
        &self.out_color
    }

    pub fn get_out_depth(&self) -> &DepthStencilView<R, DepthFormat> {
        &self.out_depth
    }

    pub fn get_device(&self) -> &D {
        &self.device
    }

    pub fn get_factory(&self) -> &F {
        &self.factory
    }

    pub fn get_window(&self) -> &W {
        &self.window
    }

    pub fn get_extras(&self) -> &T {
        &self.extras
    }

    pub fn get_mut_device(&mut self) -> &mut D {
        &mut self.device
    }

    pub fn get_mut_factory(&mut self) -> &mut F {
        &mut self.factory
    }

    pub fn get_mut_window(&mut self) -> &mut W {
        &mut self.window
    }

    pub fn get_mut_extras(&mut self) -> &mut T {
        &mut self.extras
    }
}
