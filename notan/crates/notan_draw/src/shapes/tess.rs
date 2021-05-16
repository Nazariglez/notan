use lyon::path::Path;
use lyon::tessellation::*;
use notan_graphics::color::Color;
use std::cell::RefCell;

thread_local! {
    static STROKE_TESSELLATOR:RefCell<StrokeTessellator> = RefCell::new(StrokeTessellator::new());
    static FILL_TESSELLATOR:RefCell<FillTessellator> = RefCell::new(FillTessellator::new());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TessMode {
    Fill,
    Stroke,
}

pub(super) fn fill_lyon_path(
    path: &Path,
    color: Color,
    options: &FillOptions,
) -> (Vec<f32>, Vec<u32>) {
    let mut geometry: VertexBuffers<[f32; 6], u32> = VertexBuffers::new();
    {
        FILL_TESSELLATOR.with(|tessellator| {
            tessellator
                .borrow_mut()
                .tessellate_path(
                    path,
                    options,
                    &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                        let [x, y] = vertex.position().to_array();
                        [x, y, color.r, color.g, color.b, color.a]
                    }),
                )
                .unwrap()
        });
    }

    (geometry.vertices.concat(), geometry.indices)
}

pub(super) fn stroke_lyon_path(
    path: &Path,
    color: Color,
    options: &StrokeOptions,
) -> (Vec<f32>, Vec<u32>) {
    let mut geometry: VertexBuffers<[f32; 6], u32> = VertexBuffers::new();
    {
        STROKE_TESSELLATOR.with(|tessellator| {
            tessellator
                .borrow_mut()
                .tessellate_path(
                    path,
                    options,
                    &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
                        let [x, y] = vertex.position().to_array();
                        [x, y, color.r, color.g, color.b, color.a]
                    }),
                )
                .unwrap()
        });
    }

    (geometry.vertices.concat(), geometry.indices)
}
