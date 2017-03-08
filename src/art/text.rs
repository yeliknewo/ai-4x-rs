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
        'i' => make_i_render(),
        'j' => make_j_render(),
        'k' => make_k_render(),
        'l' => make_l_render(),
        'm' => make_m_render(),
        'n' => make_n_render(),
        'o' => make_o_render(),
        'p' => make_p_render(),
        'q' => make_q_render(),
        'r' => make_r_render(),
        's' => make_s_render(),
        't' => make_t_render(),
        'u' => make_u_render(),
        'v' => make_v_render(),
        'w' => make_w_render(),
        'x' => make_x_render(),
        'y' => make_y_render(),
        'z' => make_z_render(),
        _ => make_space_render(),
    }

}

fn make_a_render() -> Packet {
    let bot_left_left = Vertex::new([0.0, 0.0]); //0
    let bot_left_right = Vertex::new([0.2, 0.0]); //1
    let top_mid_left = Vertex::new([0.4, 1.0]); //2
    let top_mid_right = Vertex::new([0.6, 1.0]); //3
    let mid_left_top = Vertex::new([0.23, 0.4]); //4
    let mid_left_bot = Vertex::new([0.18, 0.2]); //5
    let mid_right_top = Vertex::new([0.77, 0.4]); //6
    let mid_right_bot = Vertex::new([0.82, 0.2]); //7
    let bot_right_left = Vertex::new([0.8, 0.0]); //8
    let bot_right_right = Vertex::new([1.0, 0.0]); //9

    let vertices = vec![bot_left_left, bot_left_right, top_mid_left, top_mid_right, mid_left_top, mid_left_bot, mid_right_top, mid_right_bot, bot_right_left, bot_right_right];

    let indices = vec![
        0, 1, 3,//
        3, 2, 0,
        4, 5, 7,
        7, 6, 4,
        8, 9, 3,
        3, 2, 8
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_b_render() -> Packet {
    let bot_left_out = Vertex::new([0.0, 0.0]); //0
    let bot_left_in = Vertex::new([0.2, 0.2]); //1
    let top_left_out = Vertex::new([0.0, 1.0]); //2
    let top_left_in = Vertex::new([0.2, 0.8]); //3
    let mid_left_top = Vertex::new([0.2, 0.6]); //4
    let mid_left_bot = Vertex::new([0.2, 0.4]); //5
    let top_right_top_out = Vertex::new([0.75, 1.0]); //6
    let top_right_top_in = Vertex::new([0.6, 0.8]); //7
    let top_right_right_out = Vertex::new([1.0, 0.75]); //8
    let top_right_right_in = Vertex::new([0.75, 0.75]); //9
    let mid_right_top = Vertex::new([0.6, 0.4]); //10
    let mid_right_bot = Vertex::new([0.6, 0.6]); //11
    let bot_right_right_out = Vertex::new([1.0, 0.25]); //12
    let bot_right_right_in = Vertex::new([0.75, 0.25]); //13
    let bot_right_bot_out = Vertex::new([0.75, 0.0]); //14
    let bot_right_bot_in = Vertex::new([0.6, 0.2]); //15

    let indices = vec![
        0, 1, 3,//
        3, 2, 0,
        14, 12, 13,
        13, 15, 14,
        4, 5, 10,
        10, 11, 4,
        11, 10, 8,
        8, 9, 11,
        8, 6, 7,
        7, 9, 8,
        7, 6, 2,
        2, 3, 7,
        10, 11, 12,
        12, 13, 10,
        1, 0, 14,
        14, 15, 1,
    ];

    let vertices = vec![bot_left_out,
                        bot_left_in,
                        top_left_out,
                        top_left_in,
                        mid_left_top,
                        mid_left_bot,
                        top_right_top_out,
                        top_right_top_in,
                        top_right_right_out,
                        top_right_right_in,
                        mid_right_top,
                        mid_right_bot,
                        bot_right_right_out,
                        bot_right_right_in,
                        bot_right_bot_out,
                        bot_right_bot_in];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_c_render() -> Packet {
    let top_right_out = Vertex::new([1.0, 1.0]); //0
    let top_right_in = Vertex::new([1.0, 0.8]); //1
    let top_left_out = Vertex::new([0.0, 1.0]); //2
    let top_left_in = Vertex::new([0.2, 0.8]); //3
    let bot_left_out = Vertex::new([0.0, 0.0]); //4
    let bot_left_in = Vertex::new([0.2, 0.2]); //5
    let bot_right_out = Vertex::new([1.0, 0.0]); //6
    let bot_right_in = Vertex::new([1.0, 0.2]); //7

    let vertices = vec![top_right_out, top_right_in, top_left_out, top_left_in, bot_left_out, bot_left_in, bot_right_out, bot_right_in];

    let indices = vec![
        1, 0, 2, //
        2, 3, 1,
        3, 2, 4,
        4, 5, 3,
        5, 4, 6,
        6, 7, 5,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_d_render() -> Packet {
    let top_left_out = Vertex::new([0.0, 1.0]); //0
    let top_left_in = Vertex::new([0.2, 0.8]); //1
    let bot_left_out = Vertex::new([0.0, 0.0]); //2
    let bot_left_in = Vertex::new([0.2, 0.2]); //3
    let bot_right_out = Vertex::new([0.8, 0.0]); //4
    let bot_right_in = Vertex::new([0.6, 0.2]); //5
    let right_mid_out = Vertex::new([1.0, 0.5]); //6
    let right_mid_in = Vertex::new([0.8, 0.5]); //7
    let top_right_out = Vertex::new([0.8, 1.0]); //8
    let top_right_in = Vertex::new([0.6, 0.8]); //9

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
    let top_left_out = Vertex::new([0.0, 1.0]); //0
    let top_left_in = Vertex::new([0.2, 0.8]); //1
    let top_right_top = Vertex::new([1.0, 1.0]); //2
    let top_right_bot = Vertex::new([1.0, 0.8]); //3
    let mid_left_top = Vertex::new([0.2, 0.6]); //4
    let mid_left_bot = Vertex::new([0.2, 0.4]); //5
    let mid_right_top = Vertex::new([1.0, 0.6]); //6
    let mid_right_bot = Vertex::new([1.0, 0.4]); //7
    let bot_left_out = Vertex::new([0.0, 0.0]); //8
    let bot_left_in = Vertex::new([0.2, 0.2]); //9
    let bot_right_top = Vertex::new([1.0, 0.2]); //10
    let bot_right_bot = Vertex::new([1.0, 0.0]); //11

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
    let top_left_out = Vertex::new([0.0, 1.0]); //0
    let top_left_in = Vertex::new([0.3, 0.8]); //1
    let top_right_top = Vertex::new([1.0, 1.0]); //2
    let top_right_bot = Vertex::new([1.0, 0.8]); //3
    let mid_left_top = Vertex::new([0.3, 0.6]); //4
    let mid_left_bot = Vertex::new([0.3, 0.4]); //5
    let mid_right_top = Vertex::new([1.0, 0.6]); //6
    let mid_right_bot = Vertex::new([1.0, 0.4]); //7
    let bot_left_left = Vertex::new([0.0, 0.0]); //8
    let bot_left_right = Vertex::new([0.3, 0.0]); //9

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
    let top_right_top = Vertex::new([1.0, 1.0]); //0
    let top_right_bot = Vertex::new([1.0, 0.8]); //1
    let top_left_out = Vertex::new([0.0, 1.0]); //2
    let top_left_in = Vertex::new([0.2, 0.8]); //3
    let bot_left_out = Vertex::new([0.0, 0.0]); //4
    let bot_left_in = Vertex::new([0.2, 0.2]); //5
    let bot_right_out = Vertex::new([1.0, 0.0]); //6
    let bot_right_in = Vertex::new([0.8, 0.2]); //7
    let mid_right_out = Vertex::new([1.0, 0.6]); //8
    let mid_right_in = Vertex::new([0.8, 0.4]); //9
    let mid_mid_top = Vertex::new([0.5, 0.6]); //10
    let mid_mid_bot = Vertex::new([0.5, 0.4]); //11

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
    let top_left_left = Vertex::new([0.0, 1.0]); //0
    let top_left_right = Vertex::new([0.2, 1.0]); //1
    let top_right_right = Vertex::new([1.0, 1.0]); //2
    let top_right_left = Vertex::new([0.8, 1.0]); //3
    let mid_left_top = Vertex::new([0.2, 0.6]); //4
    let mid_left_bot = Vertex::new([0.2, 0.4]); //5
    let mid_right_top = Vertex::new([0.8, 0.6]); //6
    let mid_right_bot = Vertex::new([0.8, 0.4]); //7
    let bot_left_left = Vertex::new([0.0, 0.0]); //8
    let bot_left_right = Vertex::new([0.2, 0.0]); //9
    let bot_right_right = Vertex::new([1.0, 0.0]); //10
    let bot_right_left = Vertex::new([0.8, 0.0]); //11

    let vertices = vec![top_left_left, top_left_right, top_right_right, top_right_left, mid_left_top, mid_left_bot, mid_right_top, mid_right_bot, bot_left_left, bot_left_right, bot_right_right, bot_right_left];

    let indices = vec![
        1, 0, 8,//
        8, 9, 1,
        2, 3, 11,
        11, 10, 2,
        4, 5, 7,
        7, 6, 4,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_i_render() -> Packet {
    let top_left = Vertex::new([0.4, 1.0]); //0
    let top_right = Vertex::new([0.6, 1.0]); //1
    let bot_left = Vertex::new([0.4, 0.0]); //2
    let bot_right = Vertex::new([0.6, 0.0]); //3

    let vertices = vec![top_left, top_right, bot_left, bot_right];

    let indices = vec![
        0, 2, 3,//
        3, 1, 0,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_j_render() -> Packet {
    let top_left = Vertex::new([0.4, 1.0]); //0
    let top_right = Vertex::new([0.6, 1.0]); //1
    let bot_right_out = Vertex::new([0.6, 0.0]); //2
    let bot_right_in = Vertex::new([0.4, 0.2]); //3
    let bot_left_out = Vertex::new([0.0, 0.0]); //4
    let bot_left_in = Vertex::new([0.2, 0.2]); //5
    let mid_left_left = Vertex::new([0.0, 0.4]); //6
    let mid_left_right = Vertex::new([0.2, 0.4]); //7

    let vertices = vec![top_left, top_right, bot_right_out, bot_right_in, bot_left_out, bot_left_in, mid_left_left, mid_left_right];

    let indices = vec![
        1, 0, 3,//
        3, 2, 1,
        2, 4, 5,
        5, 3, 2,
        4, 6, 7,
        7, 5, 4,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_k_render() -> Packet {
    let top_left_left = Vertex::new([0.0, 1.0]); //0
    let top_left_right = Vertex::new([0.2, 1.0]); //1
    let bot_left_left = Vertex::new([0.0, 0.0]); //2
    let bot_left_right = Vertex::new([0.2, 0.0]); //3
    let mid_mid_out = Vertex::new([0.4, 0.5]); //4
    let mid_mid_in = Vertex::new([0.1, 0.5]); //5
    let top_right_left = Vertex::new([0.7, 1.0]); //6
    let top_right_right = Vertex::new([1.0, 1.0]); //7
    let bot_right_left = Vertex::new([0.7, 0.0]); //8
    let bot_right_right = Vertex::new([1.0, 0.0]); //9

    let vertices = vec![top_left_left, top_left_right, bot_left_left, bot_left_right, mid_mid_out, mid_mid_in, top_right_left, top_right_right, bot_right_left, bot_right_right];

    let indices = vec![
        1, 0, 2,//
        2, 3, 1,
        5, 4, 7,
        7, 6, 5,
        8, 9, 4,
        4, 5, 8,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_l_render() -> Packet {
    let top_left_left = Vertex::new([0.0, 1.0]); //0
    let top_left_right = Vertex::new([0.2, 1.0]); //1
    let bot_left_out = Vertex::new([0.0, 0.0]); //2
    let bot_left_in = Vertex::new([0.2, 0.2]); //3
    let bot_right_top = Vertex::new([1.0, 0.2]); //4
    let bot_right_bot = Vertex::new([1.0, 0.0]); //5

    let vertices = vec![top_left_left, top_left_right, bot_left_out, bot_left_in, bot_right_top, bot_right_bot];

    let indices = vec![
        1, 0, 2,//
        2, 3, 1,
        3, 2, 5,
        5, 4, 3,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_m_render() -> Packet {
    let bot_left_left = Vertex::new([0.0, 0.0]); //0
    let bot_left_right = Vertex::new([0.2, 0.0]); //1
    let top_left_left = Vertex::new([0.2, 1.0]); //2
    let top_left_right = Vertex::new([0.4, 1.0]); //3
    let bot_mid_left = Vertex::new([0.4, 0.0]); //4
    let bot_mid_right = Vertex::new([0.6, 0.0]); //5
    let top_right_left = Vertex::new([0.6, 1.0]); //6
    let top_right_right = Vertex::new([0.8, 1.0]); //7
    let bot_right_left = Vertex::new([0.8, 0.0]); //8
    let bot_right_right = Vertex::new([1.0, 0.0]); //9

    let vertices = vec![bot_left_left, bot_left_right, top_left_left, top_left_right, bot_mid_left, bot_mid_right, top_right_left, top_right_right, bot_right_left, bot_right_right];

    let indices = vec![
        0, 1, 3, //
        3, 2, 0,
        2, 4, 5,
        5, 3, 2,
        4, 5, 7,
        7, 6, 4,
        7, 6, 8,
        8, 9, 7,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_n_render() -> Packet {
    let bot_left_left = Vertex::new([0.0, 0.0]); //0
    let bot_left_right = Vertex::new([0.2, 0.0]); //1
    let top_left_left = Vertex::new([0.0, 1.0]); //2
    let top_left_right = Vertex::new([0.2, 1.0]); //3
    let bot_right_left = Vertex::new([0.8, 0.0]); //4
    let bot_right_right = Vertex::new([1.0, 0.0]); //5
    let top_right_left = Vertex::new([0.8, 1.0]); //6
    let top_right_right = Vertex::new([1.0, 1.0]); //7

    let vertices = vec![bot_left_left, bot_left_right, top_left_left, top_left_right, bot_right_left, bot_right_right, top_right_left, top_right_right];

    let indices = vec![
        0, 1, 3,//
        3, 2, 0,
        3, 2, 4,
        4, 5, 3,
        4, 5, 7,
        7, 6, 4,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_o_render() -> Packet {
    let top_left_out = Vertex::new([0.0, 1.0]); //0
    let top_left_in = Vertex::new([0.2, 0.8]); //1
    let bot_left_out = Vertex::new([0.0, 0.0]); //2
    let bot_left_in = Vertex::new([0.2, 0.2]); //3
    let bot_right_out = Vertex::new([1.0, 0.0]); //4
    let bot_right_in = Vertex::new([0.8, 0.2]); //5
    let top_right_out = Vertex::new([1.0, 1.0]); //6
    let top_right_in = Vertex::new([0.8, 0.8]); //7

    let vertices = vec![top_left_out, top_left_in, bot_left_out, bot_left_in, bot_right_out, bot_right_in, top_right_out, top_right_in];

    let indices = vec![
        1, 0, 2,//
        2, 3, 1,
        2, 4, 5,
        5, 3, 2,
        4, 6, 7,
        7, 5, 4,
        6, 0, 1,
        1, 7, 6,
    ];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_p_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_q_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_r_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_s_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_t_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_u_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_v_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_w_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_x_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_y_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_z_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}

fn make_space_render() -> Packet {
    let vertices = vec![];

    let indices = vec![];

    let rasterizer = Rasterizer::new_fill();

    Packet::new(vertices, indices, rasterizer)
}
