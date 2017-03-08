use components::{Camera, RenderDataSpritesheet, RenderDataText, Transform};
use events::{MainFromRender, MainToRender};
use gfx::Primitive;
use gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
use gfx::traits::{Factory, FactoryExt};
use graphics::{NGEncoder, NGFactory, NGResources, NGTexture, OutColor, OutDepth, RenderType, Shaders, pipeline_spritesheet, pipeline_text};
use specs::{RunArg, System};
use std::collections::HashMap;
use std::sync::Arc;
use utils::DuoChannel;

pub struct RenderSystem<ID>
    where ID: Send + Eq
{
    done: bool,
    main_channel: DuoChannel<MainFromRender<ID>, MainToRender<ID>>,
    out_color: OutColor,
    out_depth: OutDepth,
    bundles_spritesheet: Arc<Vec<pipeline_spritesheet::Bundle<NGResources>>>,
    bundles_text: Arc<Vec<pipeline_text::Bundle<NGResources>>>,
    character_map: HashMap<char, usize>,
    shaders: HashMap<RenderType, Shaders>,
}

impl<ID> RenderSystem<ID>
    where ID: Send + Eq
{
    pub fn new(main_channel: DuoChannel<MainFromRender<ID>, MainToRender<ID>>, out_color: OutColor, out_depth: OutDepth) -> RenderSystem<ID> {
        RenderSystem {
            done: false,
            main_channel: main_channel,
            out_color: out_color,
            out_depth: out_depth,
            bundles_spritesheet: Arc::new(vec![]),
            bundles_text: Arc::new(vec![]),
            character_map: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    pub fn add_render_text(&mut self, factory: &mut NGFactory, packet: &pipeline_text::Packet, character: char) {
        let shaders = {
            if self.shaders.contains_key(&RenderType::Text) {
                &self.shaders[&RenderType::Text]
            } else {
                self.shaders.insert(RenderType::Text, pipeline_text::make_shaders());
                &self.shaders[&RenderType::Text]
            }
        };

        let shader_set = factory.create_shader_set(shaders.get_vertex_shader(), shaders.get_fragment_shader())
            .unwrap_or_else(|err| panic!("Create Shader Set Error: {:?}", err));

        let program = factory.create_program(&shader_set)
            .unwrap_or_else(|err| panic!("Create Program Error: {:?}", err));

        let pso = factory.create_pipeline_from_program(&program, Primitive::TriangleList, packet.get_rasterizer(), pipeline_text::pipe::new())
            .unwrap_or_else(|err| panic!("Create Pipeline from Program Error: {:?}", err));

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(packet.get_vertices(), packet.get_indices());

        let data = pipeline_text::pipe::Data {
            vbuf: vbuf,
            projection_data: factory.create_constant_buffer(1),
            model_data: factory.create_constant_buffer(1),
            offset: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };

        let mut bundles = Arc::get_mut(&mut self.bundles_text).unwrap_or_else(|| panic!("Arc Shit"));

        let id = bundles.len();

        self.character_map.insert(character, id);

        bundles.push(pipeline_text::Bundle::new(slice, pso, data));
    }

    fn get_character_index(&self, character: char) -> usize {
        *self.character_map.get(&character).unwrap_or_else(|| panic!("Character not in Character Map"))
    }

    pub fn add_render_spritesheet(&mut self, factory: &mut NGFactory, packet: &pipeline_spritesheet::Packet, texture: NGTexture) -> usize {
        let shaders = {
            if self.shaders.contains_key(&RenderType::Spritesheet) {
                &self.shaders[&RenderType::Spritesheet]
            } else {
                self.shaders.insert(RenderType::Spritesheet, pipeline_spritesheet::make_shaders());
                &self.shaders[&RenderType::Spritesheet]
            }
        };

        let shader_set = factory.create_shader_set(shaders.get_vertex_shader(), shaders.get_fragment_shader())
            .unwrap_or_else(|err| panic!("Create Shader Set Error: {:?}", err));

        let program = factory.create_program(&shader_set)
            .unwrap_or_else(|err| panic!("Create Program Error: {:?}", err));

        let pso = factory.create_pipeline_from_program(&program, Primitive::TriangleList, packet.get_rasterizer(), pipeline_spritesheet::pipe::new())
            .unwrap_or_else(|err| panic!("Create Pipeline from Program Error: {:?}", err));

        let sampler_info = SamplerInfo::new(FilterMethod::Scale, WrapMode::Mirror);

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(packet.get_vertices(), packet.get_indices());

        let data = pipeline_spritesheet::pipe::Data {
            vbuf: vbuf,
            spritesheet: (texture, factory.create_sampler(sampler_info)),
            texture_data: factory.create_constant_buffer(1),
            projection_data: factory.create_constant_buffer(1),
            model_data: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };

        let mut bundles = Arc::get_mut(&mut self.bundles_spritesheet).unwrap_or_else(|| panic!("Arc Shit"));

        let id = bundles.len();

        bundles.push(pipeline_spritesheet::Bundle::new(slice, pso, data));

        id
    }

    fn render(&mut self, arg: &RunArg, mut encoder: NGEncoder, encoder_id: ID) {
        use specs::Join;

        let (mut transforms, mut cameras, mut render_datas_spritesheet, mut render_datas_text) = arg.fetch(|w| (w.write::<Transform>(), w.write::<Camera>(), w.write::<RenderDataSpritesheet>(), w.write::<RenderDataText>()));

        encoder.clear(&self.out_color, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.out_depth, 1.0);
        let (dirty_cam, view, proj) = {
            let camera = {
                let mut camera_opt = None;

                for camera in (&mut cameras).iter() {
                    if camera.is_main() {
                        camera_opt = Some(camera);
                    }
                }

                camera_opt.unwrap_or_else(|| panic!("No Main Camera Entity"))
            };

            (camera.take_dirty(), camera.get_view(), camera.get_proj())
        };

        //render spritesheets
        {
            let mut datas: Vec<_> = vec![];

            for (transform, render_data_spritesheet) in (&mut transforms, &mut render_datas_spritesheet).iter() {
                let mut camera_data = None;

                if dirty_cam {
                    camera_data = Some(pipeline_spritesheet::CameraData {
                        view: view.into(),
                        proj: proj.into(),
                    });
                }

                let model_data = pipeline_spritesheet::ModelData {
                    model: transform.get_model().into(),
                };

                let texture_data = pipeline_spritesheet::TextureData {
                    tint: render_data_spritesheet.get_tint(),
                    spritesheet_rect: render_data_spritesheet.get_spritesheet_rect(),
                    spritesheet_size: render_data_spritesheet.get_spritesheet_size(),
                };

                datas.push((render_data_spritesheet.get_render_id_num(), render_data_spritesheet.get_layer(), texture_data, camera_data, model_data));
            }

            datas.sort_by_key(|k| k.1);

            for data in datas {
                let b = self.bundles_spritesheet
                    .get(data.0)
                    .unwrap_or_else(|| panic!("No Bundle found"));

                encoder.update_constant_buffer(&b.get_data().texture_data, &data.2);

                if let Some(projection_data) = data.3 {
                    encoder.update_constant_buffer(&b.get_data().projection_data, &projection_data);
                }

                encoder.update_constant_buffer(&b.get_data().model_data, &data.4);

                b.encode(&mut encoder);
            }
        }

        //render text
        {
            let mut datas: Vec<_> = vec![];

            for (transform, render_data_text) in (&mut transforms, &mut render_datas_text).iter() {
                let mut camera_data = None;

                if dirty_cam {
                    camera_data = Some(pipeline_text::CameraData {
                        view: view.into(),
                        proj: proj.into(),
                    });
                }

                let model_data = pipeline_text::ModelData {
                    model: transform.get_model().into(),
                };

                let mut i = 0;

                for character in render_data_text.get_text().chars() {
                    datas.push((self.get_character_index(character), render_data_text.get_layer(), camera_data, model_data));
                    i += 1;
                }
            }

            datas.sort_by_key(|k| k.1);

            for data in datas {
                let b = self.bundles_text
                    .get(data.0)
                    .unwrap_or_else(|| panic!("No Bundle found"));

                if let Some(projection_data) = data.2 {
                    encoder.update_constant_buffer(&b.get_data().projection_data, &projection_data);
                }

                encoder.update_constant_buffer(&b.get_data().model_data, &data.3);

                b.encode(&mut encoder);
            }
        }


        self.main_channel.send(MainFromRender::Encoder(encoder, encoder_id));
    }

    fn process_event(&mut self, arg: &RunArg, event: MainToRender<ID>) -> bool {
        match event {
            MainToRender::Encoder(encoder, encoder_id) => {
                self.render(arg, encoder, encoder_id);
                false
            }
            MainToRender::Exit => {
                self.main_channel.send(MainFromRender::Exited);
                arg.fetch(|_| {});
                self.done = true;
                false
            }
        }
    }
}

impl<ID> System<f64> for RenderSystem<ID>
    where ID: Send + Eq
{
    fn run(&mut self, arg: RunArg, _delta_time: f64) {
        if self.done {
            arg.fetch(|_| {});
            return;
        }
        let mut event = self.main_channel.try_recv();
        while self.process_event(&arg,
                                 match event {
                                     Some(event) => event,
                                     None => {
                                         arg.fetch(|_| {});
                                         return;
                                     }
                                 }) {
            event = self.main_channel.try_recv();
        }
    }
}
