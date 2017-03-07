use gfx::{Factory, Resources};
use gfx::handle::ShaderResourceView;
use gfx::texture::{AaMode, Kind, Size};

use graphics::ColorFormat;
use image;
use std::path::Path;

pub fn load_texture<R, F, P>(factory: &mut F, path: P) -> ShaderResourceView<R, [f32; 4]>
    where P: AsRef<Path>,
          F: Factory<R>,
          R: Resources
{
    let image = match image::open(path) {
            Ok(image) => image,
            Err(err) => panic!("image load error: {}", err),
        }
        .to_rgba();

    let (width, height) = image.dimensions();
    let kind = Kind::D2(width as Size, height as Size, AaMode::Single);
    let (_, view) = match factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&image]) {
        Ok(data) => data,
        Err(err) => panic!("factory create texture const error: {}", err),
    };
    view
}
