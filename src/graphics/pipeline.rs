use gfx;
use gfx::{CommandBuffer, Encoder, PipelineState, Resources, Slice};
use gfx::state::Rasterizer;
use graphics::Shaders;

static VERTEX: &'static [u8] = include_bytes!("shaders/spritesheet_150_v.glsl");
static FRAGMENT: &'static [u8] = include_bytes!("shaders/spritesheet_150_f.glsl");

pub fn make_shaders() -> Shaders {
    debug!("Making Shaders");
    Shaders::new_from_bytes(VERTEX, FRAGMENT)
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant TextureData {
        tint: [f32; 4] = "u_Tint",
        spritesheet_rect: [f32; 4] = "u_SpritesheetRect",
        spritesheet_size: [f32; 2] = "u_SpritesheetSize",
    }

    constant ProjectionData {
        model: [[f32; 4]; 4] = "u_Model",
        view: [[f32; 4]; 4] = "u_View",
        proj: [[f32; 4]; 4] ="u_Proj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        projection_data: gfx::ConstantBuffer<ProjectionData> = "b_ProjData",
        spritesheet: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        texture_data: gfx::ConstantBuffer<TextureData> = "b_TextureData",
        out_color: gfx::BlendTarget<::graphics::ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<::graphics::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex {
            pos: pos,
            uv: uv,
        }
    }
}

pub struct Bundle<R>
    where R: Resources
{
    slice: Slice<R>,
    pso: PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
}

impl<R> Bundle<R>
    where R: Resources
{
    pub fn new(slice: Slice<R>, pso: PipelineState<R, pipe::Meta>, data: pipe::Data<R>) -> Bundle<R> {
        Bundle {
            slice: slice,
            pso: pso,
            data: data,
        }
    }

    pub fn get_data(&self) -> &pipe::Data<R> {
        &self.data
    }

    pub fn get_mut_data(&mut self) -> &mut pipe::Data<R> {
        &mut self.data
    }

    pub fn encode<C>(&self, encoder: &mut Encoder<R, C>)
        where C: CommandBuffer<R>
    {
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}

#[derive(Debug)]
pub struct Packet {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    rasterizer: Rasterizer,
}

impl Packet {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, rasterizer: Rasterizer) -> Packet {
        Packet {
            vertices: vertices,
            indices: indices,
            rasterizer: rasterizer,
        }
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        self.vertices.as_slice()
    }

    pub fn get_indices(&self) -> &[u32] {
        self.indices.as_slice()
    }

    pub fn get_rasterizer(&self) -> Rasterizer {
        self.rasterizer
    }
}
