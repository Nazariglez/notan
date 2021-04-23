use super::color_batcher::*;
use super::manager::DrawMode;
use crate::geometry::{DrawPath, Path};
use crate::manager::DrawManager;
use glam::{Mat3, Mat4, Vec2, Vec3};
use notan_graphics::prelude::*;
use std::cell::{Ref, RefCell};
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub(crate) enum GraphicCommands {
    Draw(DrawCommands),
    Render(Commands),
}

#[derive(Clone)]
pub(crate) enum DrawCommands {
    Begin(Option<Color>),
    Projection(Mat4),
    Triangle {
        vertices: [f32; 6],
        indices: [u32; 3],
        color: [f32; 4],
    },
    Rect {
        vertices: [f32; 8],
        indices: [u32; 6],
        color: [f32; 4],
    },
    RawColor {
        vertices: Vec<f32>,
        indices: Vec<u32>,
    },
}

#[derive(Clone)]
pub struct VertexColor {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl VertexColor {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self { x, y, color }
    }
}

impl Default for VertexColor {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            color: Color::WHITE,
        }
    }
}

impl From<VertexColor> for [f32; 6] {
    fn from(v: VertexColor) -> [f32; 6] {
        [v.x, v.y, v.color.r, v.color.g, v.color.b, v.color.a]
    }
}

impl From<&VertexColor> for [f32; 6] {
    fn from(v: &VertexColor) -> [f32; 6] {
        [v.x, v.y, v.color.r, v.color.g, v.color.b, v.color.a]
    }
}

#[derive(Clone)]
pub struct Draw {
    size: (i32, i32),
    pub(crate) commands: Vec<GraphicCommands>,

    pub color: Color,
    pub alpha: f32,

    projection: Mat4,
    matrix_identity: Mat3,
    matrix_stack: Vec<Mat3>,
}

// TODO
// - primitives:
//  - draw.line()
//  - draw.triangle()
//  - draw.rect()
//  - draw.circle()
// - Advanced:
//  - draw.geometry(Geometry::builder().whatever())

impl Draw {
    pub fn new(width: i32, height: i32) -> Self {
        let projection = Mat4::orthographic_lh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        Self {
            size: (width, height),
            commands: vec![
                Commands::Size { width, height }.into(),
                DrawCommands::Projection(projection.clone()).into(),
            ],
            color: Color::WHITE,
            alpha: 1.0,
            matrix_identity: Mat3::identity(),
            matrix_stack: vec![],
            projection,
        }
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.projection = projection;
        self.commands
            .push(DrawCommands::Projection(self.projection.clone()).into());
    }

    pub fn projection(&self) -> &Mat4 {
        &self.projection
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
        self.commands.push(Commands::Size { width, height }.into());
    }

    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    pub fn width(&self) -> i32 {
        self.size.0
    }

    pub fn height(&self) -> i32 {
        self.size.1
    }

    pub fn set_pipeline(&mut self, pipeline: &Pipeline) {
        self.commands.push(
            Commands::Pipeline {
                id: pipeline.id(),
                options: pipeline.options.clone(),
            }
            .into(),
        );
    }

    pub fn begin(&mut self, color: Option<&Color>) {
        self.commands
            .push(DrawCommands::Begin(color.map(|c| *c)).into());
    }

    pub fn vertex_color(
        &mut self,
        vertices: impl IntoIterator<Item = impl Into<[f32; 6]>>,
        indices: &[u32],
    ) {
        let mut vertices = vertices
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<[f32; 6]>>()
            .concat();

        compute_vertices(*self.matrix(), &mut vertices, Some(6));

        self.commands.push(
            DrawCommands::RawColor {
                vertices,
                indices: indices.to_vec(),
            }
            .into(),
        );
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        #[rustfmt::skip]
        let mut vertices = [
            x1, y1,
            x2, y2,
            x3, y3
        ];
        compute_vertices(*self.matrix(), &mut vertices, None);

        #[rustfmt::skip]
        let triangle = DrawCommands::Triangle {
            vertices,
            indices: [0, 1, 2],
            color: get_computed_color(self)
        };

        self.commands.push(triangle.into());
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        #[rustfmt::skip]
        let mut vertices = [
            x, y,
            x + width, y,
            x, y + height,
            x + width, y + height
        ];
        compute_vertices(*self.matrix(), &mut vertices, None);

        #[rustfmt::skip]
        let rect = DrawCommands::Rect {
            vertices,
            indices: [0, 1, 2, 2, 1, 3],
            color:get_computed_color(self),
        };

        self.commands.push(rect.into());
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32) {
        self.path_begin(x1, y1)
            .line_to(x2, y2)
            .end(false)
            .stroke(width);
    }

    pub fn path(&mut self, path: &Path) {
        //TODO process this directly without the vertex_color?
        let color: Color = get_computed_color(self).into();
        let vertices = (0..path.vertices.len())
            .step_by(2)
            .map(|i| VertexColor {
                x: path.vertices[i],
                y: path.vertices[i + 1],
                color: color,
            })
            .collect::<Vec<_>>();
        self.vertex_color(&vertices, &path.indices);
    }

    // pub fn test_color<'a>(&mut self, info: &DrawInfo<'a>) {
    //     self.vertex_color(info.vertices, info.indices);
    // }

    pub fn path_begin(&mut self, x: f32, y: f32) -> DrawPath {
        let mut builder = Path::builder();
        builder.begin(x, y);

        DrawPath {
            builder,
            draw: self,
        }
    }

    pub fn end(&mut self) {
        self.commands.push(GraphicCommands::Render(Commands::End));
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn push(&mut self, matrix: Mat3) {
        let last_matrix = *self.matrix();
        let next_matrix = matrix * last_matrix;
        self.matrix_stack.push(next_matrix);
    }

    pub fn pop(&mut self) -> Option<Mat3> {
        self.matrix_stack.pop()
    }

    pub fn push_scale(&mut self, x: f32, y: f32) {
        self.push(Mat3::from_scale(Vec2::new(x, y)));
    }

    pub fn push_translation(&mut self, x: f32, y: f32) {
        self.push(Mat3::from_translation(Vec2::new(x, y)));
    }

    pub fn push_rotation(&mut self, angle: f32) {
        self.push(Mat3::from_angle(angle));
    }

    pub fn push_skew(&mut self, x: f32, y: f32) {
        let xt = x.tan();
        let yt = y.tan();

        self.push(Mat3::from_cols(
            Vec3::new(1.0, xt, 0.0),
            Vec3::new(yt, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ));
    }

    #[inline]
    pub fn matrix(&mut self) -> &Mat3 {
        match self.matrix_stack.last().as_ref() {
            Some(m) => m,
            _ => &self.matrix_identity,
        }
    }
}

fn compute_vertices(matrix: Mat3, vertices: &mut [f32], offset: Option<usize>) {
    debug_assert!(
        vertices.len() % 2 == 0,
        "Vertices len should be a pair number"
    );
    for i in (0..vertices.len()).step_by(offset.unwrap_or(2)) {
        let xyz = matrix * Vec3::new(vertices[i], vertices[i + 1], 1.0);
        vertices[i] = xyz.x;
        vertices[i + 1] = xyz.y;
    }
}

fn get_computed_color(draw: &Draw) -> [f32; 4] {
    [
        draw.color.r,
        draw.color.g,
        draw.color.b,
        draw.color.a * draw.alpha,
    ]
}

// // TODO cargo make

// pub struct DrawInfo<'a> {
//     pub vertices: &'a [VertexColor],
//     pub indices: &'a [u32],
// }

pub trait DrawRenderer {
    fn commands<'a>(
        &self,
        device: &mut Device,
        draw_manager: &'a mut DrawManager,
    ) -> &'a [Commands];
}

impl DrawRenderer for Draw {
    fn commands<'a>(&self, _: &mut Device, draw_manager: &'a mut DrawManager) -> &'a [Commands] {
        draw_manager.process_batch(self)
    }
}

impl From<Commands> for GraphicCommands {
    fn from(cmd: Commands) -> GraphicCommands {
        GraphicCommands::Render(cmd)
    }
}

impl From<DrawCommands> for GraphicCommands {
    fn from(cmd: DrawCommands) -> GraphicCommands {
        GraphicCommands::Draw(cmd)
    }
}
//http://www.independent-software.com/determining-coordinates-on-a-html-canvas-bezier-curve.html
