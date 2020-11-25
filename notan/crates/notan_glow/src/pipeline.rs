use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;
use std::rc::Rc;

pub(crate) struct InnerPipeline {
    pub vertex: Shader,
    pub fragment: Shader,
    pub program: Program,
    pub vao: VertexArray,
}

impl InnerPipeline {
    #[inline(always)]
    pub fn new(
        gl: &Rc<Context>,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<Self, String> {
        create_pipeline(gl, vertex_source, fragment_source)
    }

    #[inline(always)]
    pub fn clean(self, gl: &Rc<Context>) {
        clean_pipeline(gl, self);
    }

    #[inline(always)]
    pub fn bind(&self, gl: &Rc<Context>, options: &PipelineOptions) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.use_program(Some(self.program));

            set_stencil(gl, options);
            set_depth_stencil(gl, options);
            set_color_mask(gl, options);
            set_culling(gl, options);
            set_blend_mode(gl, &options);
        }
    }
}

#[inline(always)]
unsafe fn set_stencil(gl: &Rc<Context>, options: &PipelineOptions) {
    if should_disable_stencil(&options.stencil) {
        gl.disable(glow::STENCIL_TEST);
    } else {
        if let Some(opts) = options.stencil {
            gl.enable(glow::STENCIL_TEST);
            gl.stencil_mask(opts.write_mask);
            gl.stencil_op(
                opts.stencil_fail.to_glow(),
                opts.depth_fail.to_glow(),
                opts.pass.to_glow(),
            );
            gl.stencil_func(
                opts.compare.to_glow().unwrap_or(glow::ALWAYS),
                opts.reference as _,
                opts.read_mask,
            );
        }
    }
}

#[inline(always)]
unsafe fn set_depth_stencil(gl: &Rc<Context>, options: &PipelineOptions) {
    match options.depth_stencil.compare.to_glow() {
        Some(d) => {
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(d);
        }
        _ => gl.disable(glow::DEPTH_TEST),
    }

    gl.depth_mask(options.depth_stencil.write);
}

#[inline(always)]
unsafe fn set_color_mask(gl: &Rc<Context>, options: &PipelineOptions) {
    gl.color_mask(
        options.color_mask.r,
        options.color_mask.g,
        options.color_mask.b,
        options.color_mask.a,
    );
}

#[inline(always)]
unsafe fn set_culling(gl: &Rc<Context>, options: &PipelineOptions) {
    match options.cull_mode.to_glow() {
        Some(mode) => {
            gl.enable(glow::CULL_FACE);
            gl.cull_face(mode);
        }
        _ => gl.disable(glow::CULL_FACE),
    }
}

#[inline(always)]
unsafe fn set_blend_mode(gl: &Rc<Context>, options: &PipelineOptions) {
    match (options.color_blend, options.alpha_blend) {
        (Some(cbm), None) => {
            gl.enable(glow::BLEND);
            gl.blend_func(cbm.src.to_glow(), cbm.dst.to_glow());
            gl.blend_equation(cbm.op.to_glow());
        }
        (Some(cbm), Some(abm)) => {
            gl.enable(glow::BLEND);
            gl.blend_func_separate(
                cbm.src.to_glow(),
                cbm.dst.to_glow(),
                abm.src.to_glow(),
                abm.dst.to_glow(),
            );
            gl.blend_equation_separate(cbm.op.to_glow(), abm.op.to_glow());
        }
        (None, Some(abm)) => {
            let cbm = BlendMode::NORMAL;
            gl.enable(glow::BLEND);
            gl.blend_func_separate(
                cbm.src.to_glow(),
                cbm.dst.to_glow(),
                abm.src.to_glow(),
                abm.dst.to_glow(),
            );
            gl.blend_equation_separate(cbm.op.to_glow(), abm.op.to_glow());
        }
        (None, None) => {
            gl.disable(glow::BLEND);
        }
    }
}

#[inline(always)]
fn clean_pipeline(gl: &Rc<Context>, pip: InnerPipeline) {
    let InnerPipeline {
        vertex,
        fragment,
        program,
        vao,
    } = pip;

    unsafe {
        gl.delete_shader(vertex);
        gl.delete_shader(fragment);
        gl.delete_program(program);
        gl.delete_vertex_array(vao);
    }
}

#[inline(always)]
fn create_pipeline(
    gl: &Rc<Context>,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<InnerPipeline, String> {
    let vertex = create_shader(gl, glow::VERTEX_SHADER, vertex_source)?;
    let fragment = create_shader(gl, glow::FRAGMENT_SHADER, fragment_source)?;
    let program = create_program(gl, vertex, fragment)?;

    let vao = unsafe {
        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(vao));
        vao
    };

    Ok(InnerPipeline {
        vertex,
        fragment,
        program,
        vao,
    })
}

#[inline(always)]
fn create_shader(gl: &Rc<Context>, typ: u32, source: &str) -> Result<Shader, String> {
    unsafe {
        let shader = gl.create_shader(typ)?;
        gl.shader_source(shader, &source);
        gl.compile_shader(shader);

        let success = gl.get_shader_compile_status(shader);
        if success {
            return Ok(shader);
        }

        let err = gl.get_shader_info_log(shader);
        gl.delete_shader(shader);
        Err(err)
    }
}

#[inline(always)]
fn create_program(gl: &Rc<Context>, vertex: Shader, fragment: Shader) -> Result<Program, String> {
    unsafe {
        let program = gl.create_program()?;
        gl.attach_shader(program, vertex);
        gl.attach_shader(program, fragment);
        gl.link_program(program);

        let success = gl.get_program_link_status(program);
        if success {
            return Ok(program);
        }

        let err = gl.get_program_info_log(program);
        gl.delete_program(program);
        Err(err)
    }
}

#[inline(always)]
fn should_disable_stencil(stencil: &Option<StencilOptions>) -> bool {
    match stencil {
        Some(stencil) => {
            stencil.compare == CompareMode::Always
                && stencil.stencil_fail == StencilAction::Keep
                && stencil.depth_fail == StencilAction::Keep
                && stencil.pass == StencilAction::Keep
        }
        None => true,
    }
}
