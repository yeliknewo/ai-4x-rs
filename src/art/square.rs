use graphics::{Packet, Rasterizer, Vertex};

pub fn make_square_render() -> Packet {
    let vertices = vec![Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]), Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]), Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]), Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0])];

    let indices = vec![0, 3, 2, 2, 1, 0];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}
