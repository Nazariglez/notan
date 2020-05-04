use std::convert::TryInto;

pub type Matrix4 = [f32; 16];

#[inline]
pub fn matrix4_orthogonal(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Matrix4 {
    let rml = right - left;
    let rpl = right + left;
    let tmb = top - bottom;
    let tpb = top + bottom;
    let fmn = far - near;
    let fpn = far + near;

    #[rustfmt::skip]
    let matrix = [
        2.0 / rml, 0.0, 0.0, 0.0,
        0.0, 2.0 / tmb, 0.0, 0.0,
        0.0, 0.0, 2.0 / fmn, 0.0,
        -(rpl / rml), -(tpb / tmb), -(fpn / fmn), 1.0,
    ];

    matrix
}

#[inline]
pub fn matrix4_perspective(fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Matrix4 {
    let uh = 1.0 / (fovy / 2.0).tan();
    let uw = uh / aspect;
    let nmf = z_near - z_far;
    let fpn = z_far + z_near;

    #[rustfmt::skip]
    let matrix = [
        uw, 0.0, 0.0, 0.0,
        0.0, uh, 0.0, 0.0,
        0.0, 0.0, fpn / nmf, 2.0 * z_far * z_near / nmf,
        0.0, 0.0, -1.0, 0.0
    ];

    matrix
}

#[inline]
pub fn matrix4_identity() -> Matrix4 {
    #[rustfmt::skip]
    let matrix = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    matrix
}

pub type Vector4 = [f32; 4];

#[inline]
pub fn matrix4_mul_vector4(mt: &Matrix4, vt: &Vector4) -> Vector4 {
    #[rustfmt::skip]
    let new_vector = [
        mt[0] * vt[0] + mt[1] * vt[1] + mt[2] * vt[2] + mt[3] * vt[3],
        mt[4] * vt[0] + mt[5] * vt[1] + mt[6] * vt[2] + mt[7] * vt[3],
        mt[8] * vt[0] + mt[9] * vt[1] + mt[10] * vt[2] + mt[11] * vt[3],
        mt[12] * vt[0] + mt[13] * vt[1] + mt[14] * vt[2] + mt[15] * vt[3],
    ];

    new_vector
}

#[inline]
pub fn matrix4_mul_matrix4(m1: &Matrix4, m2: &Matrix4) -> Matrix4 {
    let x0 = m1[0] * m2[0] + m1[1] * m2[4] + m1[2] * m2[8] + m1[3] * m2[12];
    let x1 = m1[0] * m2[1] + m1[1] * m2[5] + m1[2] * m2[9] + m1[3] * m2[13];
    let x2 = m1[0] * m2[2] + m1[1] * m2[6] + m1[2] * m2[10] + m1[3] * m2[14];
    let x3 = m1[0] * m2[3] + m1[1] * m2[7] + m1[2] * m2[11] + m1[3] * m2[15];

    let y0 = m1[4] * m2[0] + m1[5] * m2[4] + m1[6] * m2[8] + m1[7] * m2[12];
    let y1 = m1[4] * m2[1] + m1[5] * m2[5] + m1[6] * m2[9] + m1[7] * m2[13];
    let y2 = m1[4] * m2[2] + m1[5] * m2[6] + m1[6] * m2[10] + m1[7] * m2[14];
    let y3 = m1[4] * m2[3] + m1[5] * m2[7] + m1[6] * m2[11] + m1[7] * m2[15];

    let z0 = m1[8] * m2[0] + m1[9] * m2[4] + m1[10] * m2[8] + m1[11] * m2[12];
    let z1 = m1[8] * m2[1] + m1[9] * m2[5] + m1[10] * m2[9] + m1[11] * m2[13];
    let z2 = m1[8] * m2[2] + m1[9] * m2[6] + m1[10] * m2[10] + m1[11] * m2[14];
    let z3 = m1[8] * m2[3] + m1[9] * m2[7] + m1[10] * m2[11] + m1[11] * m2[15];

    let w0 = m1[12] * m2[0] + m1[13] * m2[4] + m1[14] * m2[8] + m1[15] * m2[12];
    let w1 = m1[12] * m2[1] + m1[13] * m2[5] + m1[14] * m2[9] + m1[15] * m2[13];
    let w2 = m1[12] * m2[2] + m1[13] * m2[6] + m1[14] * m2[10] + m1[15] * m2[14];
    let w3 = m1[12] * m2[3] + m1[13] * m2[7] + m1[14] * m2[11] + m1[15] * m2[15];

    #[rustfmt::skip]
    let new_matrix = [
        x0, x1, x2, x3,
        y0, y1, y2, y3,
        z0, z1, z2, z3,
        w0, w1, w2, w3,
    ];

    new_matrix
}

#[inline]
pub fn matrix4_scale(x: f32, y: f32, z: f32) -> Matrix4 {
    #[rustfmt::skip]
    let matrix = [
        x, 0.0, 0.0, 0.0,
        0.0, y, 0.0, 0.0,
        0.0, 0.0, z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];

    matrix
}

#[inline]
pub fn matrix4_translate(x: f32, y: f32, z: f32) -> Matrix4 {
    #[rustfmt::skip]
    let matrix = [
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0
    ];

    matrix
}

#[inline]
pub fn matrix4_rotation_x(angle: f32) -> Matrix4 {
    let ca = angle.cos();
    let sa = angle.sin();
    #[rustfmt::skip]
    let matrix = [
        1.0, 0.0, 0.0, 0.0,
        0.0, ca, -sa, 0.0,
        0.0, sa, ca, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    matrix
}

#[inline]
pub fn matrix4_rotation_y(angle: f32) -> Matrix4 {
    let ca = angle.cos();
    let sa = angle.sin();
    #[rustfmt::skip]
        let matrix = [
        ca, 0.0, sa, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sa, 0.0, ca, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    matrix
}

#[inline]
pub fn matrix4_rotation_z(angle: f32) -> Matrix4 {
    let ca = angle.cos();
    let sa = angle.sin();
    #[rustfmt::skip]
        let matrix = [
        ca, -sa, 0.0, 0.0,
        sa, ca, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];

    matrix
}

#[inline]
pub fn slice_to_matrix4(slice: &[f32]) -> &Matrix4 {
    slice.try_into().unwrap()
}
