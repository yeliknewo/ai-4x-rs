use gfx_device_gl::{Device, Factory, Resources};
use gfx_window_glutin;
use glutin::{self, WindowBuilder};
use graphics::{ColorFormat, DepthFormat, GfxWindow, WindowSettings};

pub type Window = glutin::Window;
pub type Extras = ();

pub fn build_window(window_settings: WindowSettings) -> GfxWindow<Window, Extras, Device, Factory, Resources> {
    let (title, width, height) = window_settings;

    let builder = WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height)
        .with_vsync();

    let (window, device, factory, out_color, out_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    GfxWindow::new(out_color, out_depth, device, factory, window, ())
}
