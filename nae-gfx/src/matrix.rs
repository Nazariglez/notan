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
    let x0 = m1[0] * m2[0] + m1[1] * m2[1] + m1[2] * m2[2] + m1[3] * m2[3];
    let y0 = m1[4] * m2[0] + m1[5] * m2[1] + m1[6] * m2[2] + m1[7] * m2[3];
    let z0 = m1[8] * m2[0] + m1[9] * m2[1] + m1[10] * m2[2] + m1[11] * m2[3];
    let w0 = m1[12] * m2[0] + m1[13] * m2[1] + m1[14] * m2[2] + m1[15] * m2[3];

    let x1 = m1[0] * m2[4] + m1[1] * m2[5] + m1[2] * m2[6] + m1[3] * m2[7];
    let y1 = m1[4] * m2[4] + m1[5] * m2[5] + m1[6] * m2[6] + m1[7] * m2[7];
    let z1 = m1[8] * m2[4] + m1[9] * m2[5] + m1[10] * m2[6] + m1[11] * m2[7];
    let w1 = m1[12] * m2[4] + m1[13] * m2[5] + m1[14] * m2[6] + m1[15] * m2[7];

    let x2 = m1[0] * m2[8] + m1[1] * m2[9] + m1[2] * m2[10] + m1[3] * m2[11];
    let y2 = m1[4] * m2[8] + m1[5] * m2[9] + m1[6] * m2[10] + m1[7] * m2[11];
    let z2 = m1[8] * m2[8] + m1[9] * m2[9] + m1[10] * m2[10] + m1[11] * m2[11];
    let w2 = m1[12] * m2[8] + m1[13] * m2[9] + m1[14] * m2[10] + m1[15] * m2[11];

    let x3 = m1[0] * m2[12] + m1[1] * m2[13] + m1[2] * m2[14] + m1[3] * m2[15];
    let y3 = m1[4] * m2[12] + m1[5] * m2[13] + m1[6] * m2[14] + m1[7] * m2[15];
    let z3 = m1[8] * m2[12] + m1[9] * m2[13] + m1[10] * m2[14] + m1[11] * m2[15];
    let w3 = m1[12] * m2[12] + m1[13] * m2[13] + m1[14] * m2[14] + m1[15] * m2[15];

    #[rustfmt::skip]
    let new_matrix = [
        x0, y0, z0, w0,
        x1, y1, z1, w1,
        x2, y2, z2, w2,
        x3, y3, z3, w3,
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
