use notan_graphics::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
pub(crate) fn create_gl_context(
    win: &web_sys::HtmlCanvasElement,
) -> Result<(glow::Context, String), String> {
    if let Ok(ctx) = create_webgl2_context(win) {
        return Ok((ctx, "webgl2".to_string()));
    }

    let ctx = create_webgl_context(win)?;
    Ok((ctx, "webgl".to_string()))
}

#[cfg(target_arch = "wasm32")]
fn webgl_options() -> web_sys::WebGlContextAttributes {
    let mut opts = web_sys::WebGlContextAttributes::new();
    opts.stencil(true);
    opts.premultiplied_alpha(false);
    opts.alpha(false);
    opts.antialias(false);
    opts
}

#[cfg(target_arch = "wasm32")]
fn create_webgl_context(win: &web_sys::HtmlCanvasElement) -> Result<glow::Context, String> {
    //TODO manage errors
    let gl = win
        .get_context_with_context_options("webgl", webgl_options().as_ref())
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGlRenderingContext>()
        .unwrap();

    let ctx = glow::Context::from_webgl1_context(gl);
    Ok(ctx)
}

#[cfg(target_arch = "wasm32")]
fn create_webgl2_context(win: &web_sys::HtmlCanvasElement) -> Result<glow::Context, String> {
    //TODO manage errors
    let gl = win
        .get_context_with_context_options("webgl2", webgl_options().as_ref())
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    let ctx = glow::Context::from_webgl2_context(gl);
    Ok(ctx)
}
