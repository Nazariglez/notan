use notan_math::{vec2, vec3, vec4, Mat3, Mat4, Vec2, Vec3};
use std::ops::Deref;

/// Helper methods to do matrix transformations
pub trait DrawTransform {
    /// Returns the object's matrix
    fn matrix(&mut self) -> &mut Option<Mat3>;

    /// Set the object's matrix
    fn transform(&mut self, matrix: Mat3) -> &mut Self {
        *self.matrix() = Some(matrix);
        self
    }

    #[inline]
    /// Clone the transformation matrix to the passed matrix
    fn clone_matrix_to(&mut self, matrix: &mut Mat3) -> &mut Self {
        *matrix = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        self
    }

    /// Set the matrix position
    fn translate(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let matrix = old * Mat3::from_translation(Vec2::new(x, y));
        *self.matrix() = Some(matrix);
        self
    }

    /// Set the matrix scale
    fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let matrix = old * Mat3::from_scale(Vec2::new(x, y));
        *self.matrix() = Some(matrix);
        self
    }

    /// Set the matrix rotation using radians
    fn rotate(&mut self, angle: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let matrix = old * Mat3::from_angle(angle);
        *self.matrix() = Some(matrix);
        self
    }

    #[inline]
    /// Set the matrix rotation using degrees
    fn rotate_degrees(&mut self, deg: f32) -> &mut Self {
        self.rotate(deg.to_radians())
    }

    /// Set the matrix skew
    fn skew(&mut self, x: f32, y: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);

        let xt = x.tan();
        let yt = y.tan();

        let new = Mat3::from_cols(
            Vec3::new(1.0, xt, 0.0),
            Vec3::new(yt, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );

        *self.matrix() = Some(old * new);
        self
    }

    /// Set the matrix rotation using radians from the point given
    fn rotate_from(&mut self, point: (f32, f32), angle: f32) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let translate = old * Mat3::from_translation(Vec2::new(point.0, point.1));
        let rotate = translate * Mat3::from_angle(angle);
        let matrix = rotate * Mat3::from_translation(Vec2::new(-point.0, -point.1));
        *self.matrix() = Some(matrix);
        self
    }

    #[inline]
    /// Set the matrix rotation using degrees from the point given
    fn rotate_degrees_from(&mut self, point: (f32, f32), deg: f32) -> &mut Self {
        self.rotate_from(point, deg.to_radians())
    }

    /// Set the matrix scale from the point given
    fn scale_from(&mut self, point: (f32, f32), scale: (f32, f32)) -> &mut Self {
        let old = self.matrix().unwrap_or_else(|| Mat3::IDENTITY);
        let translate = old * Mat3::from_translation(Vec2::new(point.0, point.1));
        let scale = translate * Mat3::from_scale(Vec2::new(scale.0, scale.1));
        let matrix = scale * Mat3::from_translation(Vec2::new(-point.0, -point.1));
        *self.matrix() = Some(matrix);
        self
    }
}

#[derive(Default, Clone, Debug)]
/// This struct represents a stack of matrices
pub struct Transform {
    identity: Mat3,
    stack: Vec<Mat3>,
}

impl Transform {
    /// Create a new instance
    pub fn new() -> Self {
        Self {
            identity: Mat3::IDENTITY,
            stack: vec![],
        }
    }

    /// Returns a read reference of the matrix in use
    pub fn matrix(&self) -> &Mat3 {
        self.stack.last().unwrap_or(&self.identity)
    }

    /// Returns a mutable reference of the matrix in use
    pub fn matrix_mut(&mut self) -> &mut Mat3 {
        self.stack.last_mut().unwrap_or(&mut self.identity)
    }

    /// Set the matrix in use
    pub fn set(&mut self, matrix: Mat3) -> &mut Self {
        *self.matrix_mut() = matrix;
        self
    }

    /// Multiply the last matrix with the new one and adds it to the stack
    pub fn push(&mut self, matrix: Mat3) -> &mut Self {
        self.stack.push(*self.matrix() * matrix);
        self
    }

    /// Remove the last matrix from the stack
    pub fn pop(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    /// Clear all the matrices on the stack and reset the base to IDENTITY
    pub fn clear(&mut self) -> &mut Self {
        self.identity = Mat3::IDENTITY;
        self.stack.clear();
        self
    }
}

impl Deref for Transform {
    type Target = Mat3;
    fn deref(&self) -> &Self::Target {
        self.matrix()
    }
}

pub(crate) fn screen_to_local_position(
    screen_pos: Vec2,
    screen_size: Vec2,
    inverse_projection: Mat4,
    view: Mat3,
) -> Vec2 {
    // normalized coordinates
    let norm = screen_pos / screen_size;
    let mx = norm.x * 2.0 - 1.0;
    let my = -norm.y * 2.0 + 1.0;

    // projected position
    let pos = inverse_projection.project_point3(vec3(mx, my, 1.0));
    let inverse = view.inverse();

    // local position
    inverse.transform_point2(vec2(pos.x, pos.y))
}

pub(crate) fn local_to_screen_position(
    local_pos: Vec2,
    screen_size: Vec2,
    projection: Mat4,
    view: Mat3,
) -> Vec2 {
    let half = screen_size * 0.5;
    let pos = view * vec3(local_pos.x, local_pos.y, 1.0);
    let pos = projection * vec4(pos.x, pos.y, pos.z, 1.0);
    vec2(half.x + (half.x * pos.x), half.y + (half.y * -pos.y))
}

#[cfg(test)]
mod test {
    use super::local_to_screen_position;
    use super::screen_to_local_position;
    use notan_math::{vec2, Mat3, Mat4};

    #[test]
    fn screen_to_local() {
        let screen_size = vec2(800.0, 600.0);

        // Using regular projection and view
        let proj = Mat4::orthographic_rh_gl(0.0, screen_size.x, screen_size.y, 0.0, -1.0, 1.0);
        let view = Mat3::IDENTITY;

        let pos_expected = [
            (vec2(0.0, 0.0), vec2(0.0, 0.0)),
            (vec2(400.0, 300.0), vec2(400.0, 300.0)),
            (vec2(800.0, 600.0), vec2(800.0, 600.0)),
        ];

        pos_expected.into_iter().for_each(|(pos, expect)| {
            let local = screen_to_local_position(pos, screen_size, proj.inverse(), view);
            assert_eq!(local, expect, "Using regular projection and view");
        });

        // Using regular projection and scaled view
        let proj = Mat4::orthographic_rh_gl(0.0, screen_size.x, screen_size.y, 0.0, -1.0, 1.0);
        let view = Mat3::from_scale(vec2(2.0, 2.0));

        let pos_expected = [
            (vec2(0.0, 0.0), vec2(0.0, 0.0)),
            (vec2(400.0, 300.0), vec2(200.0, 150.0)),
            (vec2(800.0, 600.0), vec2(400.0, 300.0)),
        ];

        pos_expected.into_iter().for_each(|(pos, expect)| {
            let local = screen_to_local_position(pos, screen_size, proj.inverse(), view);
            assert_eq!(local, expect, "Using regular projection and scaled view");
        });

        // Using scaled projection and identity view
        let proj = Mat4::orthographic_rh_gl(
            0.0,
            screen_size.x * 0.5,
            screen_size.y * 0.5,
            0.0,
            -1.0,
            1.0,
        );
        let view = Mat3::IDENTITY;

        let pos_expected = [
            (vec2(0.0, 0.0), vec2(0.0, 0.0)),
            (vec2(400.0, 300.0), vec2(200.0, 150.0)),
            (vec2(800.0, 600.0), vec2(400.0, 300.0)),
        ];

        pos_expected.into_iter().for_each(|(pos, expect)| {
            let local = screen_to_local_position(pos, screen_size, proj.inverse(), view);
            assert_eq!(local, expect, "Using scaled projection and identity view");
        });
    }

    #[test]
    fn local_to_screen() {
        let screen_size = vec2(800.0, 600.0);

        // Using regular projection and view
        let proj = Mat4::orthographic_rh_gl(0.0, screen_size.x, screen_size.y, 0.0, -1.0, 1.0);
        let view = Mat3::IDENTITY;

        let pos_expected = [
            (vec2(0.0, 0.0), vec2(0.0, 0.0)),
            (vec2(400.0, 300.0), vec2(400.0, 300.0)),
            (vec2(800.0, 600.0), vec2(800.0, 600.0)),
        ];

        pos_expected.into_iter().for_each(|(pos, expect)| {
            let local = local_to_screen_position(pos, screen_size, proj, view);
            assert_eq!(local, expect, "Using regular projection and view");
        });

        // Using regular projection and scaled view
        let proj = Mat4::orthographic_rh_gl(0.0, screen_size.x, screen_size.y, 0.0, -1.0, 1.0);
        let view = Mat3::from_scale(vec2(2.0, 2.0));

        let pos_expected = [
            (vec2(0.0, 0.0), vec2(0.0, 0.0)),
            (vec2(400.0, 300.0), vec2(800.0, 600.0)),
            (vec2(800.0, 600.0), vec2(1600.0, 1200.0)),
        ];

        pos_expected.into_iter().for_each(|(pos, expect)| {
            let local = local_to_screen_position(pos, screen_size, proj, view);
            assert_eq!(local, expect, "Using regular projection and scaled view");
        });

        // Using scaled projection and identity view
        let proj = Mat4::orthographic_rh_gl(
            0.0,
            screen_size.x * 0.5,
            screen_size.y * 0.5,
            0.0,
            -1.0,
            1.0,
        );
        let view = Mat3::IDENTITY;

        let pos_expected = [
            (vec2(0.0, 0.0), vec2(0.0, 0.0)),
            (vec2(400.0, 300.0), vec2(800.0, 600.0)),
            (vec2(800.0, 600.0), vec2(1600.0, 1200.0)),
        ];

        pos_expected.into_iter().for_each(|(pos, expect)| {
            let local = local_to_screen_position(pos, screen_size, proj, view);
            assert_eq!(local, expect, "Using scaled projection and identity view");
        });
    }

    #[test]
    fn screen_to_local_to_screen() {
        let screen_size = vec2(800.0, 600.0);

        // Using regular projection and view
        let proj = Mat4::orthographic_rh_gl(0.0, screen_size.x, screen_size.y, 0.0, -1.0, 1.0);
        let view = Mat3::IDENTITY;

        let pos_expected = [vec2(0.0, 0.0), vec2(400.0, 300.0), vec2(800.0, 600.0)];

        pos_expected.into_iter().for_each(|pos| {
            let local = screen_to_local_position(pos, screen_size, proj.inverse(), view);
            let screen = local_to_screen_position(local, screen_size, proj, view);
            assert_eq!(screen, pos, "Using regular projection and view");
        });

        // Using regular projection and rotated view
        let proj = Mat4::orthographic_rh_gl(0.0, screen_size.x, screen_size.y, 0.0, -1.0, 1.0);
        let view = Mat3::from_angle(45.0f32.to_radians());

        let pos_expected = [vec2(0.0, 0.0), vec2(400.0, 300.0), vec2(800.0, 600.0)];

        pos_expected.into_iter().for_each(|pos| {
            let local = screen_to_local_position(pos, screen_size, proj.inverse(), view);
            let screen = local_to_screen_position(local, screen_size, proj, view);
            assert_eq!(
                screen.round(),
                pos,
                "Using regular projection and rotated view"
            );
        });

        // Using a scaled projection and rotated nd scaled view
        let proj = Mat4::orthographic_rh_gl(
            0.0,
            screen_size.x * 0.5,
            screen_size.y * 0.5,
            0.0,
            -1.0,
            1.0,
        );
        let view = Mat3::from_scale(vec2(1.5, 2.5)) * Mat3::from_angle(45.0f32.to_radians());

        let pos_expected = [vec2(0.0, 0.0), vec2(400.0, 300.0), vec2(800.0, 600.0)];

        pos_expected.into_iter().for_each(|pos| {
            let local = screen_to_local_position(pos, screen_size, proj.inverse(), view);
            let screen = local_to_screen_position(local, screen_size, proj, view);
            assert_eq!(
                screen.round(),
                pos,
                "Using a scaled projection and rotated nd scaled view"
            );
        });
    }
}
