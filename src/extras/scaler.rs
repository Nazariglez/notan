use super::Transform2d;
use nae_gfx::Matrix4;

/// Modes or rules used to scale the matrix transformation
#[derive(Clone, Copy, Debug)]
pub enum ScalerMode {
    /// Do nothing, keeps the matrix scale
    None,
    /// Scale the content to fill the container's size without keep the aspect ratio
    Fill,
    /// Scale the content to fit the container's size keeping the aspect ratio
    AspectFit,
    /// Scale the content to fill the container's size keeping the aspect ratio (leaving outside some content if needed)
    AspectFill,
}

/// Helper that scale a matrix following the rules of the ScalerMode
/// Useful to work with a fixed width and height while the space can be adapted
/// to any screen or container size.
pub struct Scaler {
    mode: ScalerMode,
    transform: Transform2d,
    scale_dirty: bool,
    container_size: (f32, f32),
    working_size: (f32, f32),
    scale: (f32, f32),
    working_scale: (f32, f32),
}

impl Scaler {
    /// Create a new object with a custom size and a custom mode
    pub fn new(width: f32, height: f32, mode: ScalerMode) -> Self {
        Self {
            container_size: (1.0, 1.0),
            working_size: (width, height),
            mode,
            transform: Transform2d::new(width, height),
            scale_dirty: false,
            scale: (1.0, 1.0),
            working_scale: (1.0, 1.0),
        }
    }

    /// Sets the origin position
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.transform.set_position(x, y);
        self
    }

    /// Origin position
    pub fn position(&self) -> (f32, f32) {
        (self.transform.x, self.transform.y)
    }

    /// Sets the anchor position of the matrix (using normalized values)
    pub fn set_anchor(&mut self, x: f32, y: f32) -> &mut Self {
        self.transform.set_anchor(x, y);
        self
    }

    /// Anchor position
    pub fn anchor(&self) -> (f32, f32) {
        (self.transform.anchor_x, self.transform.anchor_y)
    }

    fn update(&mut self) {
        if !self.scale_dirty {
            return;
        }

        self.scale_dirty = false;

        let (sw, sh) = self.working_size;
        self.transform.set_size(sw, sh);

        let (cw, ch) = self.container_size;
        let (scale_x, scale_y) = match self.mode {
            ScalerMode::Fill => (cw / sw, ch / sh),
            ScalerMode::AspectFit => {
                let scale = (cw / sw).min(ch / sh);
                (scale, scale)
            }
            ScalerMode::AspectFill => {
                let scale = (cw / sw).max(ch / sh);
                (scale, scale)
            }
            _ => (1.0, 1.0),
        };

        self.working_scale = (scale_x * self.scale.0, scale_y * self.scale.1);
        self.transform
            .set_scale(self.working_scale.0, self.working_scale.1);
    }

    /// Returns the current working scale
    pub fn working_scale(&mut self) -> (f32, f32) {
        self.update();
        self.working_scale
    }

    /// Sets the scale of the content
    pub fn set_scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale = (x, y);
        self.scale_dirty = true;
        self
    }

    /// Content scale
    pub fn scale(&self) -> (f32, f32) {
        self.scale
    }

    /// Sets the size of the container to fill or fit
    pub fn set_container_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.container_size = (width, height);
        self.scale_dirty = true;
        self
    }

    /// Returns the fixed size of the content
    pub fn working_size(&self) -> (f32, f32) {
        self.working_size
    }

    /// Returns the size of the container used to scale the content
    pub fn container_size(&self) -> (f32, f32) {
        self.container_size
    }

    /// Sets the fixed sizes that we want to use to work it
    pub fn set_working_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.working_size = (width, height);
        self.scale_dirty = true;
        self
    }

    /// Sets the mode/rule used to scale the content
    pub fn set_mode(&mut self, mode: ScalerMode) -> &mut Self {
        self.mode = mode;
        self.scale_dirty = true;
        self
    }

    /// Gets the current mode/rule used to scale the content
    pub fn mode(&self) -> ScalerMode {
        self.mode
    }

    /// Returns the calculated matrix ready to use
    pub fn matrix(&mut self) -> &Matrix4 {
        self.update();
        self.transform.matrix()
    }
}
