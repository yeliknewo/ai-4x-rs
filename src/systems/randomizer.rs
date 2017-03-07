use art::Sprite;
use art::main::{blue, blue_green, dark_grey, dark_orange, dark_purple, dark_red, dark_yellow, green, grey, light_grey, mint_choc, orange, pink, purple, purple_red, red, yellow};
use components::{Randomized, RenderData};
use rand::{Rng, thread_rng};
use specs::{Join, RunArg, System};

#[derive(Debug)]
pub struct RandomizerSystem {
    sprites: Vec<Sprite>,
    i: u64,
}

impl RandomizerSystem {
    pub fn new() -> RandomizerSystem {
        let mut sprites: Vec<Sprite> = vec![];

        for sprite in blue::ARRAY.iter()
            .chain(blue_green::ARRAY)
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

        RandomizerSystem {
            sprites: sprites,
            i: 0,
        }
    }

    fn get_random_rect(&self) -> Sprite {
        self.sprites[thread_rng().gen_range(0, self.sprites.len())]
    }
}

impl System<f64> for RandomizerSystem {
    fn run(&mut self, arg: RunArg, delta_time: f64) {
        let (randomizeds, mut render_datas) = arg.fetch(|world| (world.read::<Randomized>(), world.write::<RenderData>()));

        for (randomized, mut render_data) in (&randomizeds, &mut render_datas).iter() {
            if self.i % randomized.get_i() == 0 {
                render_data.set_spritesheet_rect(self.get_random_rect());
            }
        }

        self.i += 1;
    }
}
