#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
pub(crate) fn create_gl_context(
    win: &web_sys::HtmlCanvasElement,
    antialias: bool,
    transparent: bool,
) -> Result<(glow::Context, String), String> {
    if let Ok(ctx) = create_webgl2_context(win, antialias, transparent) {
        return Ok((ctx, "webgl2".to_string()));
    }

    let ctx = create_webgl_context(win, antialias, transparent)?;
    Ok((ctx, "webgl".to_string()))
}

#[cfg(target_arch = "wasm32")]
fn webgl_options(antialias: bool, transparent: bool) -> web_sys::WebGlContextAttributes {
    let opts = web_sys::WebGlContextAttributes::new();
    opts.set_stencil(true);
    opts.set_premultiplied_alpha(false);
    opts.set_alpha(transparent);
    opts.set_antialias(antialias);
    opts.set_power_preference(web_sys::WebGlPowerPreference::HighPerformance);
    opts.set_fail_if_major_performance_caveat(true);
    opts
}

#[cfg(target_arch = "wasm32")]
fn create_webgl_context(
    win: &web_sys::HtmlCanvasElement,
    antialias: bool,
    transparent: bool,
) -> Result<glow::Context, String> {
    let gl = win
        .get_context_with_context_options("webgl", webgl_options(antialias, transparent).as_ref())
        .map_err(|e| format!("{e:?}"))?
        .ok_or("Cannot acquire the Webgl context. Is the canvas already instantiated?")?
        .dyn_into::<web_sys::WebGlRenderingContext>()
        .map_err(|_| "Cannot acquire WebGL context.")?;

    let ctx = glow::Context::from_webgl1_context(gl);
    Ok(ctx)
}

#[cfg(target_arch = "wasm32")]
fn create_webgl2_context(
    win: &web_sys::HtmlCanvasElement,
    antialias: bool,
    transparent: bool,
) -> Result<glow::Context, String> {
    let gl = win
        .get_context_with_context_options("webgl2", webgl_options(antialias, transparent).as_ref())
        .map_err(|e| format!("{e:?}"))?
        .ok_or("Cannot acquire the Webgl2 context. Is the canvas already instantiated?")?
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .map_err(|_| "Cannot acquire WebGL2 context.")?;

    let ctx = glow::Context::from_webgl2_context(gl);
    Ok(ctx)
}
