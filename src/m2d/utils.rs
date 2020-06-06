pub mod point {
    use nae_gfx::Matrix4;

    pub fn screen_to_local(x: f32, y: f32, to: &Matrix4) -> (f32, f32) {
        let id = 1.0 / ((to[0] * to[5]) + (to[4] * -to[1]));
        let xx =
            (to[5] * id * x) + (-to[4] * id * y) + (((to[13] * to[4]) - (to[12] * to[5])) * id);
        let yy =
            (to[0] * id * y) + (-to[1] * id * x) + (((-to[13] * to[0]) + (to[12] * to[1])) * id);
        (xx, yy)
    }

    pub fn local_to_screen(x: f32, y: f32, from: &Matrix4) -> (f32, f32) {
        let xx = from[0] * x + from[4] * y + from[12];
        let yy = from[1] * x + from[5] * y + from[13];
        (xx, yy)
    }

    pub fn local_to_local(x: f32, y: f32, from: &Matrix4, to: &Matrix4) -> (f32, f32) {
        let (x, y) = local_to_screen(x, y, from);
        screen_to_local(x, y, to)
    }
}
