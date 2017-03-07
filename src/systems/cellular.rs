use art::Sprite;
use art::main::{blue, blue_green, dark_green, dark_grey, dark_orange, dark_purple, dark_red, dark_yellow, green, grey, light_grey, mint_choc, orange, pink, purple, purple_red, red, yellow};
use components::{Cell, RenderData};
use specs::{Join, RunArg, System};

#[derive(Debug)]
pub struct CellSystem {
    sprites: Vec<Sprite>,
}

impl CellSystem {
    pub fn new() -> CellSystem {
        let mut sprites: Vec<Sprite> = vec![];

        for sprite in blue::ARRAY.iter()
            .chain(blue_green::ARRAY)
            .chain(dark_green::ARRAY)
            .chain(dark_grey::ARRAY)
            .chain(dark_orange::ARRAY)
            .chain(dark_purple::ARRAY)
            .chain(dark_red::ARRAY)
            .chain(dark_yellow::ARRAY)
            .chain(green::ARRAY)
            .chain(grey::ARRAY)
            .chain(light_grey::ARRAY)
            .chain(mint_choc::ARRAY)
            .chain(orange::ARRAY)
            .chain(pink::ARRAY)
            .chain(purple::ARRAY)
            .chain(purple_red::ARRAY)
            .chain(red::ARRAY)
            .chain(yellow::ARRAY) {
            sprites.push(sprite);
        }

        CellSystem {
            sprites: sprites,
        }
    }
}

impl System<f64> for CellSystem {
    fn run(&mut self, arg: RunArg, delta_time: f64) {
        let (mut cells, mut render_datas) = arg.fetch(|world| (world.write::<Cell>(), world.write::<RenderData>()));

        for (mut cell, mut render_data) in (&mut cells, &mut render_datas).iter() {
            let next_class = (cell.get_class() as i64 + cell.get_x() * cell.get_y()) as u64;
            cell.set_class(next_class);

            let index: usize = cell.get_class() as usize % self.sprites.len();
            render_data.set_spritesheet_rect(self.sprites[index]);
        }
    }
}
