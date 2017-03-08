use graphics::Rasterizer;
use graphics::pipeline_text::{Packet, Vertex};

pub fn make_text_render(character: char) -> Packet {
    match character {
        'a' => make_a_render(),
        'b' => make_b_render(),
        _ => make_a_render(),
    }

}

fn make_a_render() -> Packet {
    let bot_left_left = Vertex::new([0.0, 0.0, 0.0]); //0
    let bot_left_right = Vertex::new([0.1, 0.0, 0.0]); //1
    let top_mid_left = Vertex::new([0.45, 1.0, 0.0]); //2
    let top_mid_right = Vertex::new([0.55, 1.0, 0.0]); //3
    let mid_left_top = Vertex::new([0.23, 0.3, 0.0]); //4
    let mid_left_bot = Vertex::new([0.18, 0.2, 0.0]); //5
    let mid_right_top = Vertex::new([0.77, 0.3, 0.0]); //6
    let mid_right_bot = Vertex::new([0.82, 0.2, 0.0]); //7
    let bot_right_left = Vertex::new([0.9, 0.0, 0.0]); //8
    let bot_right_right = Vertex::new([1.0, 0.0, 0.0]); //9

    let vertices = vec![bot_left_left, bot_left_right, top_mid_left, top_mid_right, mid_left_top, mid_left_bot, mid_right_top, mid_right_bot, bot_right_left, bot_right_right];

    let indices = vec![0, 1, 3, 3, 2, 0, 4, 5, 7, 7, 6, 4, 8, 9, 3, 3, 2, 8];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_b_render() -> Packet {
    let bot_left = Vertex::new([0.0, 0.0, 0.0]);
    let top_left = Vertex::new([0.0, 1.0, 0.0]);
    let mid_top = Vertex::new([0.25, 0.55, 0.0]);
    let mid_bot = Vertex::new([0.25, 0.45, 0.0]);
    let top_right_top = Vertex::new([0.6, 0.9, 0.0]);
    let top_right_right = Vertex::new([1.0, 0.75, 0.0]);
    let top_right_bot = Vertex::new([0.6, 0.6, 0.0]);
    let bot_right_top = Vertex::new([0.6, 0.4, 0.0]);
    let bot_right_right = Vertex::new([1.0, 0.25, 0.0]);
    let bot_right_bot = Vertex::new([0.6, 0.1, 0.0]);

    let vertices = vec![bot_left, top_left, mid_top, mid_bot];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}
