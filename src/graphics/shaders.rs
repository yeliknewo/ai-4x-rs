#[derive(Debug)]
pub struct Shaders {
    vertex: Vec<u8>,
    fragment: Vec<u8>,
}

impl Shaders {
    pub fn new_from_bytes(vertex: &[u8], fragment: &[u8]) -> Shaders {
        let mut v_vec = vec![];
        let mut f_vec = vec![];

        v_vec.extend_from_slice(vertex);
        f_vec.extend_from_slice(fragment);

        Shaders {
            vertex: v_vec,
            fragment: f_vec,
        }
    }

    #[inline]
    pub fn get_vertex_shader(&self) -> &[u8] {
        self.vertex.as_slice()
    }

    #[inline]
    pub fn get_fragment_shader(&self) -> &[u8] {
        self.fragment.as_slice()
    }
}
