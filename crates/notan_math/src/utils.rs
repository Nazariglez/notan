/// Returns a local position from screen
#[inline]
#[deprecated]
#[doc(hidden)]
pub fn mat3_screen_to_local(x: f32, y: f32, m: glam::Mat3) -> glam::Vec2 {
    let inverse = m.inverse();
    let v = inverse * glam::vec3(x, y, 1.0);
    glam::vec2(v.x, v.y)
}

/// Returns a screen position from local
#[inline]
#[deprecated]
#[doc(hidden)]
pub fn mat3_local_to_screen(x: f32, y: f32, m: glam::Mat3) -> glam::Vec2 {
    let v = m * glam::vec3(x, y, 1.0);
    glam::vec2(v.x, v.y)
}

/// Returns a local position from another local
#[inline]
#[deprecated]
#[doc(hidden)]
pub fn mat3_local_to_local(x: f32, y: f32, from: glam::Mat3, to: glam::Mat3) -> glam::Vec2 {
    let from_point = mat3_local_to_screen(x, y, from);
    mat3_screen_to_local(from_point.x, from_point.y, to)
}
