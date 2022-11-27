use notan::math::*;
use notan::prelude::*;
use std::ops::Rem;

const TEXTURE_SIZE: IVec2 = IVec2::new(10, 10);
const TOTAL: usize = (TEXTURE_SIZE.x * TEXTURE_SIZE.y) as usize;

//language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450

    layout(location = 0) in vec4 a_position;
    layout(location = 1) in vec2 a_texcoord;

    layout(location = 0) out vec2 v_texcoord;

    void main() {
        v_texcoord = a_texcoord;
        gl_Position = vec4(a_position.x, a_position.y * -1.0, a_position.z, 1.0);
    }
    "#
};

//language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_texcoord;

    layout(location = 0) out vec4 outColor;

    // texture as unsigned sampler
    layout(binding = 0) uniform usampler2D u_texture;

    void main() {
        // get the color from the texture dividing by 12345
        // because we created the texture miltiplying red color by that number
        float r = texture(u_texture, v_texcoord).r / 12345.0 / 256.0;
        outColor = vec4(r, 0.0, 1.0 - r, 1.0);
    }
    "#
};

#[derive(AppState)]
struct State {
    clear_options: ClearOptions,
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    texture: Texture,
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new().size(400, 400).lazy_loop(true);

    notan::init_with(setup)
        .add_config(win_config)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let clear_options = ClearOptions::color(Color::new(0.1, 0.2, 0.3, 1.0));

    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x3)
        .attr(1, VertexFormat::Float32x2);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .with_vertex_info(&vertex_info)
        .with_color_blend(BlendMode::NORMAL)
        .with_texture_location(0, "u_texture")
        .build()
        .unwrap();

    // Generate the texture data
    let pixels: Vec<u32> = (0..TOTAL).map(|n| (n.rem(256) * 12345) as _).collect();
    // Cast it to a &[u8] to feed the GPU
    let bytes: &[u8] = bytemuck::cast_slice(&pixels);

    let texture = gfx
        .create_texture()
        .from_bytes(bytes, TEXTURE_SIZE.x, TEXTURE_SIZE.y)
        .with_format(TextureFormat::R32Uint)
        .build()
        .unwrap();

    #[rustfmt::skip]
    let vertices = [
        //pos               //coords
        1.0,  1.0, 0.0,     1.0, 1.0,
        1.0, -1.0, 0.0,     1.0, 0.0,
        -1.0, -1.0, 0.0,    0.0, 0.0,
        -1.0,  1.0, 0.0,    0.0, 1.0
    ];

    #[rustfmt::skip]
    let indices = [
        0, 1, 3,
        1, 2, 3,
    ];

    let vertex_buffer = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    let index_buffer = gfx
        .create_index_buffer()
        .with_data(&indices)
        .build()
        .unwrap();

    State {
        clear_options,
        pipeline,
        vertex_buffer,
        index_buffer,
        texture,
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut renderer = gfx.create_renderer();

    renderer.begin(Some(&state.clear_options));
    renderer.set_pipeline(&state.pipeline);
    renderer.bind_texture(0, &state.texture);
    renderer.bind_buffers(&[&state.vertex_buffer, &state.index_buffer]);
    renderer.draw(0, 6);
    renderer.end();

    gfx.render(&renderer);
}
