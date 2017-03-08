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
    let bot_left_out = Vertex::new([0.0, 0.0, 0.0]); //0
    let bot_left_in = Vertex::new([0.1, 0.1, 0.0]); //1
    let top_left_out = Vertex::new([0.0, 1.0, 0.0]); //2
    let top_left_in = Vertex::new([0.1, 0.9, 0.0]); //3
    let mid_top = Vertex::new([0.25, 0.55, 0.0]); //4
    let mid_bot = Vertex::new([0.25, 0.45, 0.0]); //5
    let top_right_top_out = Vertex::new([0.6, 0.9, 0.0]); //6
    let top_right_top_in = Vertex::new([0.6, 0.8, 0.0]); //7
    let top_right_right_out = Vertex::new([1.0, 0.75, 0.0]); //8
    let top_right_right_in = Vertex::new([0.8, 0.75, 0.0]); //9
    let top_right_bot_out = Vertex::new([0.6, 0.6, 0.0]); //10
    let top_right_bot_in = Vertex::new([0.6, 0.65, 0.0]); //11
    let bot_right_top_out = Vertex::new([0.6, 0.4, 0.0]); //12
    let bot_right_top_in = Vertex::new([0.6, 0.45, 0.0]); //13
    let bot_right_right_out = Vertex::new([1.0, 0.25, 0.0]); //14
    let bot_right_right_in = Vertex::new([0.8, 0.25, 0.0]); //15
    let bot_right_bot_out = Vertex::new([0.6, 0.1, 0.0]); //16
    let bot_right_bot_in = Vertex::new([0.6, 0.2, 0.0]); //17

    let vertices = vec![bot_left_out,
                        bot_left_in,
                        top_left_out,
                        top_left_in,
                        mid_top,
                        mid_bot,
                        top_right_top_out,
                        top_right_top_in,
                        top_right_right_out,
                        top_right_right_in,
                        top_right_bot_out,
                        top_right_bot_in,
                        bot_right_top_out,
                        bot_right_top_in,
                        bot_right_right_out,
                        bot_right_right_in,
                        bot_right_bot_out,
                        bot_right_bot_in];

    let indices = vec![0, 1, 3, 3, 2, 0, 0, 16, 17, 17, 1, 0, 16, 14, 15, 15, 17, 16, 14, 12, 13, 13, 15, 14];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}
