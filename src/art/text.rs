use graphics::Rasterizer;
use graphics::pipeline_text::{Packet, Vertex};

pub fn make_text_render(character: char) -> Packet {
    match character {
        'a' => make_a_render(),
        'b' => make_b_render(),
        'c' => make_c_render(),
        'd' => make_d_render(),
        'e' => make_e_render(),
        'f' => make_f_render(),
        'g' => make_g_render(),
        'h' => make_h_render(),
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
    let mid_top = Vertex::new([0.1, 0.55, 0.0]); //4
    let mid_bot = Vertex::new([0.1, 0.45, 0.0]); //5
    let top_right_top_out = Vertex::new([0.6, 1.0, 0.0]); //6
    let top_right_top_in = Vertex::new([0.6, 0.9, 0.0]); //7
    let top_right_right_out = Vertex::new([1.0, 0.75, 0.0]); //8
    let top_right_right_in = Vertex::new([0.8, 0.75, 0.0]); //9
    let top_right_bot_out = Vertex::new([0.6, 0.6, 0.0]); //10
    let top_right_bot_in = Vertex::new([0.6, 0.5, 0.0]); //11
    let bot_right_top_out = Vertex::new([0.6, 0.4, 0.0]); //12
    let bot_right_top_in = Vertex::new([0.6, 0.5, 0.0]); //13
    let bot_right_right_out = Vertex::new([1.0, 0.25, 0.0]); //14
    let bot_right_right_in = Vertex::new([0.8, 0.25, 0.0]); //15
    let bot_right_bot_out = Vertex::new([0.6, 0.0, 0.0]); //16
    let bot_right_bot_in = Vertex::new([0.6, 0.1, 0.0]); //17

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

    let indices = vec![0, 1, 3, 3, 2, 0, 0, 16, 17, 17, 1, 0, 16, 14, 15, 15, 17, 16, 14, 12, 13, 13, 15, 14, 4, 5, 12, 12, 13, 4, 4, 5, 10, 10, 11, 4, 11, 10, 8, 8, 9, 11, 8, 6, 7, 7, 9, 8, 7, 6, 2, 2, 3, 7];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_c_render() -> Packet {
    let top_right_out = Vertex::new([1.0, 1.0, 0.0]); //0
    let top_right_in = Vertex::new([1.0, 0.8, 0.0]); //1
    let top_left_out = Vertex::new([0.2, 1.0, 0.0]); //2
    let top_left_in = Vertex::new([0.2, 0.9, 0.0]); //3
    let left_middle_out = Vertex::new([0.0, 0.5, 0.0]); //4
    let left_middle_in = Vertex::new([0.2, 0.5, 0.0]); //5
    let bot_left_out = Vertex::new([0.2, 0.0, 0.0]); //6
    let bot_left_in = Vertex::new([0.2, 0.1, 0.0]); //7
    let bot_right_out = Vertex::new([1.0, 0.0, 0.0]); //8
    let bot_right_in = Vertex::new([1.0, 0.2, 0.0]); //9

    let vertices = vec![top_right_out, top_right_in, top_left_out, top_left_in, left_middle_out, left_middle_in, bot_left_out, bot_left_in, bot_right_out, bot_right_in];

    let indices = vec![
        1, 0, 2, //
        2, 3, 1,
        3, 2, 4,
        4, 5, 3,
        5, 4, 6,
        6, 7, 5,
        7, 6, 8,
        8, 9, 7,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_d_render() -> Packet {
    let top_left_out = Vertex::new([0.0, 1.0, 0.0]); //0
    let top_left_in = Vertex::new([0.1, 0.9, 0.0]); //1
    let bot_left_out = Vertex::new([0.0, 0.0, 0.0]); //2
    let bot_left_in = Vertex::new([0.1, 0.1, 0.0]); //3
    let bot_right_out = Vertex::new([0.9, 0.1, 0.0]); //4
    let bot_right_in = Vertex::new([0.8, 0.2, 0.0]); //5
    let right_mid_out = Vertex::new([1.0, 0.5, 0.0]); //6
    let right_mid_in = Vertex::new([0.9, 0.5, 0.0]); //7
    let top_right_out = Vertex::new([0.9, 0.9, 0.0]); //8
    let top_right_in = Vertex::new([0.8, 0.8, 0.0]); //9

    let vertices = vec![top_left_out, top_left_in, bot_left_out, bot_left_in, bot_right_out, bot_right_in, right_mid_out, right_mid_in, top_right_out, top_right_in];

    let indices = vec![
        0, 2, 3, //
        3, 1, 0,
        2, 4, 5,
        5, 3, 2,
        4, 6, 7,
        7, 5, 4,
        6, 8, 9,
        9, 7, 6,
        8, 0, 1,
        1, 9, 8,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_e_render() -> Packet {
    let top_left_out = Vertex::new([0.0, 1.0, 0.0]); //0
    let top_left_in = Vertex::new([0.2, 0.8, 0.0]); //1
    let top_right_top = Vertex::new([1.0, 1.0, 0.0]); //2
    let top_right_bot = Vertex::new([1.0, 0.8, 0.0]); //3
    let mid_left_top = Vertex::new([0.2, 0.6, 0.0]); //4
    let mid_left_bot = Vertex::new([0.2, 0.4, 0.0]); //5
    let mid_right_top = Vertex::new([1.0, 0.6, 0.0]); //6
    let mid_right_bot = Vertex::new([1.0, 0.4, 0.0]); //7
    let bot_left_out = Vertex::new([0.0, 0.0, 0.0]); //8
    let bot_left_in = Vertex::new([0.2, 0.2, 0.0]); //9
    let bot_right_top = Vertex::new([1.0, 0.2, 0.0]); //10
    let bot_right_bot = Vertex::new([1.0, 0.0, 0.0]); //11

    let vertices = vec![top_left_out, top_left_in, top_right_top, top_right_bot, mid_left_top, mid_left_bot, mid_right_top, mid_right_bot, bot_left_out, bot_left_in, bot_right_top, bot_right_bot];

    let indices = vec![
        0, 1, 9,//
        9, 8, 0,
        0, 1, 3,
        3, 2, 0,
        4, 5, 7,
        7, 6, 4,
        8, 11, 10,
        10, 9, 8,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_f_render() -> Packet {
    let top_left_out = Vertex::new([0.0, 1.0, 0.0]); //0
    let top_left_in = Vertex::new([0.2, 0.8, 0.0]); //1
    let top_right_top = Vertex::new([1.0, 1.0, 0.0]); //2
    let top_right_bot = Vertex::new([1.0, 0.8, 0.0]); //3
    let mid_left_top = Vertex::new([0.2, 0.6, 0.0]); //4
    let mid_left_bot = Vertex::new([0.2, 0.4, 0.0]); //5
    let mid_right_top = Vertex::new([1.0, 0.6, 0.0]); //6
    let mid_right_bot = Vertex::new([1.0, 0.4, 0.0]); //7
    let bot_left_left = Vertex::new([0.0, 0.0, 0.0]); //8
    let bot_left_right = Vertex::new([0.2, 0.0, 0.0]); //9

    let vertices = vec![top_left_out, top_left_in, top_right_top, top_right_bot, mid_left_top, mid_left_bot, mid_right_top, mid_right_bot, bot_left_left, bot_left_right];

    let indices = vec![
        0, 1, 9,//
        9, 8, 0,
        0, 1, 3,
        3, 2, 0,
        4, 5, 7,
        7, 6, 4,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_g_render() -> Packet {
    let top_right_top = Vertex::new([1.0, 1.0, 0.0]); //0
    let top_right_bot = Vertex::new([1.0, 0.9, 0.0]); //1
    let top_left_out = Vertex::new([0.0, 1.0, 0.0]); //2
    let top_left_in = Vertex::new([0.1, 0.9, 0.0]); //3
    let bot_left_out = Vertex::new([0.0, 0.0, 0.0]); //4
    let bot_left_in = Vertex::new([0.1, 0.1, 0.0]); //5
    let bot_right_out = Vertex::new([1.0, 0.0, 0.0]); //6
    let bot_right_in = Vertex::new([0.9, 0.1, 0.0]); //7
    let mid_right_out = Vertex::new([1.0, 0.55, 0.0]); //8
    let mid_right_in = Vertex::new([0.9, 0.45, 0.0]); //9
    let mid_mid_top = Vertex::new([0.5, 0.55, 0.0]); //10
    let mid_mid_bot = Vertex::new([0.5, 0.45, 0.0]); //11

    let vertices = vec![top_right_top, top_right_bot, top_left_out, top_left_in, bot_left_out, bot_left_in, bot_right_out, bot_right_in, mid_right_out, mid_right_in, mid_mid_top, mid_mid_bot];

    let indices = vec![
        0, 2, 3,//
        3, 1, 0,
        2, 4, 5,
        5, 3, 2,
        4, 6, 7,
        7, 5, 4,
        6, 8, 9,
        9, 7, 6,
        8, 10, 11,
        11, 9, 8,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_h_render() -> Packet {
    let top_left_out = Vertex::new([0.0, 1.0, 0.0]); //0
    let top_left_in = Vertex::new([0.2, 0.8, 0.0]); //1
    let top_right_top = Vertex::new([1.0, 1.0, 0.0]); //2
    let top_right_bot = Vertex::new([1.0, 0.8, 0.0]); //3
    let mid_left_top = Vertex::new([0.2, 0.6, 0.0]); //4
    let mid_left_bot = Vertex::new([0.2, 0.4, 0.0]); //5
    let mid_right_top = Vertex::new([1.0, 0.6, 0.0]); //6
    let mid_right_bot = Vertex::new([1.0, 0.4, 0.0]); //7
    let bot_left_out = Vertex::new([0.0, 0.0, 0.0]); //8
    let bot_left_in = Vertex::new([0.2, 0.2, 0.0]); //9
    let bot_right_top = Vertex::new([1.0, 0.2, 0.0]); //10
    let bot_right_bot = Vertex::new([1.0, 0.0, 0.0]); //11

    let vertices = vec![top_left_out, top_left_in, top_right_top, top_right_bot, mid_left_top, mid_left_bot, mid_right_top, mid_right_bot, bot_left_out, bot_left_in, bot_right_top, bot_right_bot];

    let indices = vec![
        0, 1, 9,//
        9, 8, 0,
        0, 1, 3,
        3, 2, 0,
        4, 5, 7,
        7, 6, 4,
        8, 11, 10,
        10, 9, 8,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}
