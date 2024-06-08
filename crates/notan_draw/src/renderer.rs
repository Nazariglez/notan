use notan_math::Mat3;

struct BatchInfo {
    start: usize,
    end: usize,
}

pub struct DrawRenderer {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    batches: Vec<BatchInfo>,
}

impl DrawRenderer {
    pub fn push<T: DrawItem>(&mut self, item: &T, matrix: impl Into<Mat3>) {
        // TODO
    }
}

pub trait DrawItem {
    fn geometry(&self) -> (&[f32], &[u32]);
}
