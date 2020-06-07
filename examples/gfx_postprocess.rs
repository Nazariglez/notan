use nae::prelude::*;
use nalgebra_glm as glm;

struct State {
    cube: Cube,
    postprocess: PostProcessTexture,
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    State {
        cube: Cube::new(app),
        postprocess: PostProcessTexture::new(app),
    }
}

fn draw(app: &mut App, state: &mut State) {
    // Set the render target to draw the cube
    app.gfx()
        .set_render_target(Some(&state.postprocess.render_target));
    state.cube.draw(app);

    // Disable the render target to draw to screen
    app.gfx().set_render_target(None);

    // Render the texture of the render target with the postprocess shader
    state.postprocess.draw(app);
}

// The data required to draw the cube
struct Cube {
    pipeline: Pipeline,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    clear: ClearOptions,
    vertices: [f32; 168],
    indices: [u32; 36],
    rx: f32,
    ry: f32,
    mvp_location: Uniform,
    mvp: glm::Mat4,
}

impl Cube {
    fn new(app: &mut App) -> Cube {
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
        )
        .unwrap();

        let mvp_location = pipeline.uniform_location("u_matrix").unwrap();

        let vertex_buffer = VertexBuffer::new(&gfx, DrawUsage::Dynamic).unwrap();

        let index_buffer = IndexBuffer::new(&gfx, DrawUsage::Dynamic).unwrap();

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

        Cube {
            pipeline,
            vertex_buffer,
            index_buffer,
            mvp_location,
            vertices,
            clear,
            indices,
            rx: 0.0,
            ry: 0.0,
            mvp,
        }
    }

    fn draw(&mut self, app: &mut App) {
        self.rx += 0.01;
        self.ry += 0.01;
        let rmx = glm::rotate_x(&self.mvp, self.rx);
        let mvp = glm::rotate_y(&rmx, self.ry);

        let mut gfx = app.gfx();
        gfx.begin(&self.clear);
        gfx.set_pipeline(&self.pipeline);
        gfx.bind_uniform(&self.mvp_location, slice_to_matrix4(mvp.as_slice()));
        gfx.bind_vertex_buffer(&self.vertex_buffer, &self.vertices);
        gfx.bind_index_buffer(&self.index_buffer, &self.indices);
        gfx.draw(0, self.indices.len() as i32);
        gfx.end();
    }
}

// Data and render target to use as postprocess effect
struct PostProcessTexture {
    render_target: RenderTarget,
    pipeline: Pipeline,
    tex_size_loc: Uniform,
    value_loc: Uniform,
    value: f32,
}

impl PostProcessTexture {
    fn new(app: &mut App) -> Self {
        let render_target = RenderTarget::from(
            app,
            app.width() as _,
            app.height() as _,
            true,
            Default::default(),
        )
        .unwrap();
        let pipeline = Pipeline::from_image_fragment(
            app.gfx(),
            include_bytes!("assets/shaders/pixel_invert.frag.spv"),
        )
        .unwrap();
        let tex_size_loc = pipeline.uniform_location("u_tex_size").unwrap();
        let value_loc = pipeline.uniform_location("u_value").unwrap();

        Self {
            pipeline,
            tex_size_loc,
            value_loc,
            render_target,
            value: 0.0,
        }
    }

    fn draw(&mut self, app: &mut App) {
        let width = app.width();
        let height = app.height();
        let value = 5.0 + self.value.sin();

        let draw = app.draw();

        // Set the new pipeline using the postprocess shader
        draw.set_pipeline(Some(&self.pipeline));
        draw.set_uniform(&self.tex_size_loc, &[width, height]);
        draw.set_uniform(&self.value_loc, &value);

        draw.begin(Color::new(0.1, 0.2, 0.3, 1.0));
        draw.image(&self.render_target.texture, 0.0, 0.0);
        draw.end();

        draw.set_pipeline(None);

        self.value += 0.005;
    }
}
