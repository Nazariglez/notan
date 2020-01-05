use nae_core::*;

#[derive(Debug, Default)]
pub struct Transform2d {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub anchor_x: f32,
    pub anchor_y: f32,
    pub pivot_x: f32,
    pub pivot_y: f32,
    pub rotation: f32,
    pub skew_x: f32,
    pub skew_y: f32,
    pub flip_x: bool,
    pub flip_y: bool,

    cached: CachedTransform2d,
}

impl Transform2d {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            scale_x: 1.0,
            scale_y: 1.0,
            ..Default::default()
        }
    }

    fn update(&mut self) {
        self.cached.set_pos(self.x, self.y);
        self.cached.set_size(self.width, self.height);
        self.cached.set_scale(self.scale_x, self.scale_y);
        self.cached.set_anchor(self.anchor_x, self.anchor_y);
        self.cached.set_pivot(self.pivot_x, self.pivot_y);
        self.cached.set_skew(self.skew_x, self.skew_y);
        self.cached.set_flip(self.flip_x, self.flip_y);
        self.cached.set_rotation(self.rotation);
    }

    pub fn matrix(&mut self) -> &math::Mat3 {
        self.update();
        &self.cached.matrix()
    }

    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn set_rotation(&mut self, rad: f32) -> &mut Self {
        self.rotation = rad;
        self
    }

    pub fn set_anchor(&mut self, anchor_x: f32, anchor_y: f32) -> &mut Self {
        self.anchor_x = anchor_x;
        self.anchor_y = anchor_y;
        self
    }

    pub fn set_pivot(&mut self, pivot_x: f32, pivot_y: f32) -> &mut Self {
        self.pivot_x = pivot_x;
        self.pivot_y = pivot_y;
        self
    }

    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) -> &mut Self {
        self.scale_x = scale_x;
        self.scale_y = scale_y;
        self
    }

    pub fn set_skew(&mut self, skew_x: f32, skew_y: f32) -> &mut Self {
        self.skew_x = skew_x;
        self.skew_y = skew_y;
        self
    }

    pub fn set_flip(&mut self, flip_x: bool, flip_y: bool) -> &mut Self {
        self.flip_x = flip_x;
        self.flip_y = flip_y;
        self
    }
}

#[derive(Debug)]
struct CachedTransform2d {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    scale_x: f32,
    scale_y: f32,
    anchor_x: f32,
    anchor_y: f32,
    pivot_x: f32,
    pivot_y: f32,
    rotation: f32,
    skew_x: f32,
    skew_y: f32,
    flip_x: bool,
    flip_y: bool,

    skew_cos_x: f32,
    skew_sin_x: f32,
    skew_cos_y: f32,
    skew_sin_y: f32,

    dirty: bool,
    dirty_angle: bool,

    matrix: math::Mat3,
}

impl Default for CachedTransform2d {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            anchor_x: 0.0,
            anchor_y: 0.0,
            pivot_x: 0.0,
            pivot_y: 0.0,
            rotation: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
            flip_x: false,
            flip_y: false,
            skew_cos_x: 0.0,
            skew_sin_x: 0.0,
            skew_cos_y: 0.0,
            skew_sin_y: 0.0,
            dirty: true,
            dirty_angle: true,
            matrix: math::identity(),
        }
    }
}

impl CachedTransform2d {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    fn set_pos(&mut self, x: f32, y: f32) {
        if x != self.x || y != self.y {
            self.dirty = true;
            self.x = x;
            self.y = y;
        }
    }

    fn set_size(&mut self, width: f32, height: f32) {
        if width != self.width || height != self.height {
            self.dirty = true;
            self.width = width;
            self.height = height;
        }
    }

    fn set_rotation(&mut self, rad: f32) {
        if self.rotation != rad {
            self.dirty = true;
            self.dirty_angle = true;
            self.rotation = rad;
        }
    }

    fn set_anchor(&mut self, anchor_x: f32, anchor_y: f32) {
        if anchor_x != self.anchor_x || anchor_y != self.anchor_y {
            self.dirty = true;
            self.anchor_x = anchor_x;
            self.anchor_y = anchor_y;
        }
    }

    fn set_pivot(&mut self, pivot_x: f32, pivot_y: f32) {
        if pivot_x != self.pivot_x || pivot_y != self.pivot_y {
            self.dirty = true;
            self.pivot_x = pivot_x;
            self.pivot_y = pivot_y;
        }
    }

    fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
        if scale_x != self.scale_x || scale_y != self.scale_y {
            self.dirty = true;
            self.scale_x = scale_x;
            self.scale_y = scale_y;
        }
    }

    fn set_skew(&mut self, skew_x: f32, skew_y: f32) {
        if skew_x != self.skew_x || skew_y != self.skew_y {
            self.dirty = true;
            self.skew_x = skew_x;
            self.skew_y = skew_y;
        }
    }

    fn set_flip(&mut self, flip_x: bool, flip_y: bool) {
        if flip_x != self.flip_x || flip_y != self.flip_y {
            self.dirty = true;
            self.flip_x = flip_x;
            self.flip_y = flip_y;
        }
    }

    fn dirty(&self) -> bool {
        self.dirty || self.dirty_angle
    }

    fn update(&mut self) {
        if !self.dirty() {
            return;
        }

        if self.dirty_angle {
            self.skew_cos_x = (self.rotation + self.skew_x).cos();
            self.skew_sin_x = (self.rotation + self.skew_x).sin();
            self.skew_cos_y = (self.rotation - self.skew_y).cos();
            self.skew_sin_y = -(self.rotation - self.skew_y).sin();
        }

        let scale_x = if self.flip_x { -1.0 } else { 1.0 } * self.scale_x;
        let scale_y = if self.flip_y { -1.0 } else { 1.0 } * self.scale_y;
        let anchor_x = if self.flip_x {
            1.0 - self.anchor_x
        } else {
            self.anchor_x
        };
        let anchor_y = if self.flip_y {
            1.0 - self.anchor_y
        } else {
            self.anchor_y
        };
        let pivot_x = if self.flip_x {
            1.0 - self.pivot_x
        } else {
            self.pivot_x
        };
        let pivot_y = if self.flip_y {
            1.0 - self.pivot_y
        } else {
            self.pivot_y
        };

        self.matrix[0 + (0 * 3)] = self.skew_cos_x * scale_x;
        self.matrix[1 + (0 * 3)] = self.skew_sin_x * scale_x;
        self.matrix[0 + (1 * 3)] = self.skew_sin_y * scale_y;
        self.matrix[1 + (1 * 3)] = self.skew_cos_y * scale_y;

        let anchor_width = anchor_x * self.width;
        let anchor_height = anchor_y * self.height;
        let pivot_width = pivot_x * self.width;
        let pivot_height = pivot_y * self.height;

        self.matrix[0 + (2 * 3)] = self.x - anchor_width * scale_x + pivot_width * scale_x;
        self.matrix[1 + (2 * 3)] = self.y - anchor_height * scale_y + pivot_height * scale_y;

        if pivot_width != 0.0 || pivot_height != 0.0 {
            self.matrix[0 + (2 * 3)] -=
                pivot_width * self.matrix[0 + (0 * 3)] + pivot_height * self.matrix[0 + (1 * 3)];
            self.matrix[1 + (2 * 3)] -=
                pivot_width * self.matrix[1 + (0 * 3)] + pivot_height * self.matrix[1 + (1 * 3)];
        }

        self.dirty = false;
        self.dirty_angle = false;
    }

    fn matrix(&mut self) -> &math::Mat3 {
        self.update();
        &self.matrix
    }
}
