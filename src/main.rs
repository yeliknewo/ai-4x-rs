#[macro_use]
extern crate log;
extern crate cgmath;
extern crate rustc_serialize;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
// extern crate gfx_window_sdl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
// extern crate sdl2;
extern crate find_folder;
extern crate specs;
extern crate time;
extern crate env_logger;

mod art;
mod components;
mod core;
mod events;
mod graphics;
mod systems;
mod utils;

fn main() {
    env_logger::init().unwrap();

    core::start();
}
