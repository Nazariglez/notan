use notan::math::{vec3, Mat4, Vec3};
use notan::prelude::*;

// language=glsl
const LIGHT_VERTEX_SHADER: ShaderSource = notan::vertex_shader! {
  r#"
    #version 450
    layout (location = 0) in vec3 a_pos;
    layout (location = 1) in vec3 a_normal;

    layout (location = 0) out vec3 v_pos;
    layout (location = 1) out vec3 v_normal;

    layout(set = 0, binding = 0) uniform Transform {
        mat4 model;
        mat4 view;
        mat4 projection;
    };

    void main()
    {
        v_pos = vec3(model * vec4(a_pos, 1.0));
        v_normal = a_normal;

        gl_Position = projection * view * vec4(v_pos, 1.0);
    }
  "#
};

// language=glsl
const LIGHT_FRAGMENT_SHADER: ShaderSource = notan::fragment_shader! {
  r#"
    #version 450
    layout(location = 0) in vec3 v_normal;
    layout(location = 1) in vec3 v_pos;

    layout(location = 0) out vec4 color;

    layout(set = 0, binding = 1) uniform Light {
        vec3 lightPos;
        vec3 lightColor;
        vec3 objectColor;
    };

    void main()
    {
        // ambient
        float ambientStrength = 0.1;
        vec3 ambient = ambientStrength * lightColor;

        // diffuse
        vec3 norm = normalize(v_normal);
        vec3 lightDir = normalize(lightPos - v_pos);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColor;

        vec3 result = (ambient + diffuse) * objectColor;
        color = vec4(result, 1.0);
    }
  "#
};

/*
   Marking this struct with the `uniform` macro allows
   to use the struct as a buffer data using the right
   layout (std140) automatically, though this struct
   uses 3 Mat4 which is already using the right layout
*/
#[uniform]
#[derive(Copy, Clone)]
struct Transform {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            model: Mat4::IDENTITY,
            view: Mat4::IDENTITY,
            projection: Mat4::perspective_rh_gl(20.0_f32.to_radians(), 800.0 / 600.0, 0.1, 100.0),
        }
    }
}

impl Transform {
    fn with_view(time: f32) -> Transform {
        let radius = 10.0;
        let cam_x = time.sin() * radius;
        let cam_z = time.cos() * radius;

        Transform {
            view: Mat4::look_at_rh(
                vec3(cam_x, cam_z * 0.5, cam_z),
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            ),
            ..Default::default()
        }
    }
}

/*
   This struct doesn't fit the sdt140 layout
   because a Vec3 uses 12 bytes instead of 16,
   using `uniform` macro will adds the necessary
   padding making it works as it is
*/
#[uniform]
#[derive(Copy, Clone)]
struct Light {
    light_pos: Vec3,
    light_color: Vec3,
    object_color: Vec3,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_pos: vec3(1.2, 1.0, 2.0),
            light_color: vec3(1.0, 1.0, 1.0),
            object_color: vec3(1.0, 0.5, 0.31),
        }
    }
}

impl Light {
    fn with_time(time: f32) -> Self {
        Self {
            light_pos: vec3(1.2 + time.sin() * 2.0, 1.0 + time.cos() * 2.0, 2.0),
            ..Default::default()
        }
    }
}

#[derive(AppState)]
struct State {
    pipeline: Pipeline,
    vbo: Buffer,
    transform_ubo: Buffer,
    light_ubo: Buffer,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    // Declare the vertex attributes
    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x3) // positions
        .attr(1, VertexFormat::Float32x3); // normals

    // Enable depth test
    let depth_test = DepthStencil {
        write: true,
        compare: CompareMode::Less,
    };

    // build the pipeline
    let pipeline = gfx
        .create_pipeline()
        .from(&LIGHT_VERTEX_SHADER, &LIGHT_FRAGMENT_SHADER)
        .with_vertex_info(&vertex_info)
        .with_depth_stencil(depth_test)
        .build()
        .unwrap();

    // define vertex data
    #[rustfmt::skip]
    let vertices = [
        // pos              // normals
        -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,
        0.5, -0.5, -0.5,    0.0,  0.0, -1.0,
        0.5,  0.5, -0.5,    0.0,  0.0, -1.0,
        0.5,  0.5, -0.5,    0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,
        0.5, -0.5,  0.5,    0.0,  0.0,  1.0,
        0.5,  0.5,  0.5,    0.0,  0.0,  1.0,
        0.5,  0.5,  0.5,    0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,   0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,

        -0.5,  0.5,  0.5,   -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,   -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,   -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,   -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5,   -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5,   -1.0,  0.0,  0.0,

        0.5,  0.5,  0.5,    1.0,  0.0,  0.0,
        0.5,  0.5, -0.5,    1.0,  0.0,  0.0,
        0.5, -0.5, -0.5,    1.0,  0.0,  0.0,
        0.5, -0.5, -0.5,    1.0,  0.0,  0.0,
        0.5, -0.5,  0.5,    1.0,  0.0,  0.0,
        0.5,  0.5,  0.5,    1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,
        0.5, -0.5, -0.5,    0.0, -1.0,  0.0,
        0.5, -0.5,  0.5,    0.0, -1.0,  0.0,
        0.5, -0.5,  0.5,    0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,   0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,
        0.5,  0.5, -0.5,    0.0,  1.0,  0.0,
        0.5,  0.5,  0.5,    0.0,  1.0,  0.0,
        0.5,  0.5,  0.5,    0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,
    ];

    // create the vertex buffer object
    let vbo = gfx
        .create_vertex_buffer()
        .with_data(&vertices)
        .with_info(&vertex_info)
        .build()
        .unwrap();

    let transform = Transform::with_view(0.0);

    // create the uniform buffer object
    let transform_ubo = gfx
        .create_uniform_buffer(0, "Transform")
        .with_data(&transform) // upload the transform to the gpu directly
        .build()
        .unwrap();

    let light = Light::with_time(0.0);

    let light_ubo = gfx
        .create_uniform_buffer(1, "Light")
        .with_data(&light) // upload the light object to thr gpu directly
        .build()
        .unwrap();

    State {
        pipeline,
        vbo,
        transform_ubo,
        light_ubo,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let time = app.timer.time_since_init();
    gfx.set_buffer_data(&state.transform_ubo, &Transform::with_view(time));
    gfx.set_buffer_data(&state.light_ubo, &Light::with_time(time));

    let mut renderer = gfx.create_renderer();

    let clear = ClearOptions {
        color: Some(Color::from_rgb(0.1, 0.2, 0.3)),
        depth: Some(1.0),
        stencil: None,
    };

    renderer.begin(Some(clear));

    renderer.set_pipeline(&state.pipeline);
    renderer.bind_buffers(&[&state.vbo, &state.transform_ubo, &state.light_ubo]);
    renderer.draw(0, 36);

    renderer.end();

    gfx.render(&renderer);
}
