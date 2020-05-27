use nae::prelude::*;
use nalgebra_glm as glm;

struct State {
    render_target: RenderTarget,
    clear: ClearOptions,
    clear_target: ClearOptions,
    colored_cube: ColoredCube,
    textured_cube: TexturedCube,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let clear = ClearOptions {
        color: Some(Color::WHITE),
        depth: Some(1.0),
        ..Default::default()
    };

    let clear_target = ClearOptions {
        color: Some(Color::new(0.1, 0.2, 0.1, 1.0)),
        depth: Some(1.0),
        ..Default::default()
    };

    let render_target = RenderTarget::from_size(app, 200, 200, true).unwrap();

    State {
        render_target,
        clear,
        clear_target,
        colored_cube: ColoredCube::new(app).unwrap(),
        textured_cube: TexturedCube::new(app).unwrap(),
    }
}

fn draw(app: &mut App, state: &mut State) {
    let mut gfx = app.gfx();

    // Render the colored cube to a RenderTarget
    gfx.begin_to(Some(&state.render_target), &state.clear_target);
    state.colored_cube.draw(gfx);
    gfx.end();

    // Draw the RenderTarget to the TexturedCube
    gfx.begin(&state.clear);
    state.textured_cube.draw(gfx, &state.render_target.texture);
    gfx.end();
}

fn rotate_matrix(matrix: &mut glm::Mat4, rx: &mut f32, ry: &mut f32) -> glm::Mat4 {
    *rx += 0.01;
    *ry += 0.01;
    let rmx = glm::rotate_x(matrix, *rx);
    let mvp = glm::rotate_y(&rmx, *ry);

    mvp
}

// Represent a colored cube
struct ColoredCube {
    pipeline: Pipeline,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    vertices: [f32; 168],
    indices: [u32; 36],
    rx: f32,
    ry: f32,
    mvp_location: Uniform,
    mvp: glm::Mat4,
    clear: ClearOptions,
}

impl ColoredCube {
    fn new(app: &mut App) -> Result<Self, String> {
        let mut gfx = app.gfx();
        let pipeline = Pipeline::new(
            &gfx,
            include_bytes!("assets/shaders/color_matrix.vert.spv"),
            include_bytes!("assets/shaders/color.frag.spv"),
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float4),
            ],
            PipelineOptions {
                depth_stencil: DepthStencil {
                    write: true,
                    compare: CompareMode::Less,
                },
                ..Default::default()
            },
        )?;

        let mvp_location = pipeline.uniform_location("u_matrix")?;

        let vertex_buffer = VertexBuffer::new(&gfx, DrawUsage::Dynamic)?;
        let index_buffer = IndexBuffer::new(&gfx, DrawUsage::Dynamic)?;

        let clear = ClearOptions {
            color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
            depth: Some(1.0),
            stencil: None,
        };

        #[rustfmt::skip]
            let vertices = [
            -1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
            1.0, -1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
            1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,
            -1.0,  1.0, -1.0,   1.0, 0.0, 0.0, 1.0,

            -1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
            1.0, -1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
            1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,
            -1.0,  1.0,  1.0,   0.0, 1.0, 0.0, 1.0,

            -1.0, -1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
            -1.0,  1.0, -1.0,   0.0, 0.0, 1.0, 1.0,
            -1.0,  1.0,  1.0,   0.0, 0.0, 1.0, 1.0,
            -1.0, -1.0,  1.0,   0.0, 0.0, 1.0, 1.0,

            1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
            1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
            1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,
            1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,

            -1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,
            -1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
            1.0, -1.0,  1.0,   0.0, 0.5, 1.0, 1.0,
            1.0, -1.0, -1.0,   0.0, 0.5, 1.0, 1.0,

            -1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
            -1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
            1.0,  1.0,  1.0,   1.0, 0.0, 0.5, 1.0,
            1.0,  1.0, -1.0,   1.0, 0.0, 0.5, 1.0,
        ];

        #[rustfmt::skip]
            let indices = [
            0, 1, 2,  0, 2, 3,
            6, 5, 4,  7, 6, 4,
            8, 9, 10,  8, 10, 11,
            14, 13, 12,  15, 14, 12,
            16, 17, 18,  16, 18, 19,
            22, 21, 20,  23, 22, 20
        ];

        let projection: glm::Mat4 = glm::perspective(4.0 / 3.0, 45.0, 0.1, 100.0);
        let view = glm::look_at(
            &glm::vec3(4.0, 3.0, 3.0),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let mut mvp: glm::Mat4 = glm::Mat4::identity();
        mvp = mvp * projection;
        mvp = mvp * view;

        Ok(Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            mvp_location,
            vertices,
            indices,
            rx: 0.0,
            ry: 0.0,
            mvp,
            clear,
        })
    }

    fn draw(&mut self, gfx: &mut Graphics) {
        let mvp = rotate_matrix(&mut self.mvp, &mut self.rx, &mut self.ry);

        gfx.set_pipeline(&self.pipeline);
        gfx.bind_uniform(&self.mvp_location, slice_to_matrix4(mvp.as_slice()));
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_index_buffer(&self.index_buffer, &self.indices);
        gfx.draw(0, self.indices.len() as i32);
    }
}

// Represent a textured cube
struct TexturedCube {
    pipeline: Pipeline,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    vertices: [f32; 120],
    indices: [u32; 36],
    rx: f32,
    ry: f32,
    mvp_location: Uniform,
    tex_location: Uniform,
    mvp: glm::Mat4,
    clear: ClearOptions,
}

impl TexturedCube {
    fn new(app: &mut App) -> Result<Self, String> {
        let mut gfx = app.gfx();
        let pipeline = Pipeline::new(
            &gfx,
            include_bytes!("assets/shaders/image_matrix.vert.spv"),
            include_bytes!("assets/shaders/image.frag.spv"),
            &[
                VertexAttr::new(0, VertexFormat::Float3),
                VertexAttr::new(1, VertexFormat::Float2),
            ],
            PipelineOptions {
                depth_stencil: DepthStencil {
                    write: true,
                    compare: CompareMode::Less,
                },
                ..Default::default()
            },
        )?;

        let mvp_location = pipeline.uniform_location("u_matrix")?;
        let tex_location = pipeline.uniform_location("u_texture")?;

        let vertex_buffer = VertexBuffer::new(&gfx, DrawUsage::Dynamic)?;
        let index_buffer = IndexBuffer::new(&gfx, DrawUsage::Dynamic)?;

        let clear = ClearOptions {
            color: Some(Color::new(0.1, 0.2, 0.3, 1.0)),
            depth: Some(1.0),
            stencil: None,
        };

        #[rustfmt::skip]
            let vertices = [
            -1.0, -1.0, -1.0,   1.0, 1.0,
            1.0, -1.0, -1.0,   1.0, 0.0,
            1.0,  1.0, -1.0,   0.0, 0.0,
            -1.0,  1.0, -1.0,   0.0, 1.0,

            -1.0, -1.0,  1.0,    1.0, 1.0,
            1.0, -1.0,  1.0,    1.0, 0.0,
            1.0,  1.0,  1.0,   0.0, 0.0,
            -1.0,  1.0,  1.0,   0.0, 1.0,

            -1.0, -1.0, -1.0,   1.0, 1.0,
            -1.0,  1.0, -1.0,   1.0, 0.0,
            -1.0,  1.0,  1.0,   0.0, 0.0,
            -1.0, -1.0,  1.0,   0.0, 1.0,

            1.0, -1.0, -1.0,    1.0, 1.0,
            1.0,  1.0, -1.0,    1.0, 0.0,
            1.0,  1.0,  1.0,    0.0, 0.0,
            1.0, -1.0,  1.0,    0.0, 1.0,

            -1.0, -1.0, -1.0,   1.0, 1.0,
            -1.0, -1.0,  1.0,   1.0, 0.0,
            1.0, -1.0,  1.0,   0.0, 0.0,
            1.0, -1.0, -1.0,   0.0, 1.0,

            -1.0,  1.0, -1.0,   1.0, 1.0,
            -1.0,  1.0,  1.0,   1.0, 0.0,
            1.0,  1.0,  1.0,   0.0, 0.0,
            1.0,  1.0, -1.0,   0.0, 1.0,
        ];

        #[rustfmt::skip]
            let indices = [
            0, 1, 2,  0, 2, 3,
            6, 5, 4,  7, 6, 4,
            8, 9, 10,  8, 10, 11,
            14, 13, 12,  15, 14, 12,
            16, 17, 18,  16, 18, 19,
            22, 21, 20,  23, 22, 20
        ];

        let projection: glm::Mat4 = glm::perspective(4.0 / 3.0, 45.0, 0.1, 100.0);
        let view = glm::look_at(
            &glm::vec3(4.0, 3.0, 3.0),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let mut mvp: glm::Mat4 = glm::Mat4::identity();
        mvp = mvp * projection;
        mvp = mvp * view;

        Ok(Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            tex_location,
            mvp_location,
            vertices,
            clear,
            indices,
            rx: 0.0,
            ry: 0.0,
            mvp,
        })
    }

    fn draw(&mut self, gfx: &mut Graphics, texture: &Texture) {
        let mvp = rotate_matrix(&mut self.mvp, &mut self.rx, &mut self.ry);

        gfx.set_pipeline(&self.pipeline);
        gfx.bind_uniform(&self.mvp_location, slice_to_matrix4(mvp.as_slice()));
        gfx.bind_texture(&self.tex_location, texture);
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_index_buffer(&self.index_buffer, &self.indices);
        gfx.draw(0, self.indices.len() as i32);
    }
}
