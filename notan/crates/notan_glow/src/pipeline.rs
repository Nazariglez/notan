use crate::to_glow::*;
use glow::*;
use notan_graphics::prelude::*;
use std::rc::Rc;

pub(crate) struct InnerPipeline {
    pub vertex: Shader,
    pub fragment: Shader,
    pub program: Program,
    pub vao: VertexArray,
    pub attrs: VertexAttributes,

    #[cfg(target_arch = "wasm32")]
    pub uniform_locations: Vec<UniformLocation>,
}

impl InnerPipeline {
    #[inline(always)]
    pub fn new(
        gl: &Context,
        vertex_source: &str,
        fragment_source: &str,
        attrs: &[VertexAttr],
    ) -> Result<Self, String> {
        let mut stride = 0;
        let attrs = attrs
            .iter()
            .map(|attr| {
                let inner_attr = InnerAttr::from(attr, stride);
                stride += attr.format.bytes();
                inner_attr
            })
            .collect::<Vec<_>>();

        create_pipeline(gl, vertex_source, fragment_source, stride, attrs)
    }

    #[inline(always)]
    pub fn clean(self, gl: &Context) {
        clean_pipeline(gl, self);
    }

    #[inline(always)]
    pub fn bind(&self, gl: &Context, options: &PipelineOptions) {
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

#[derive(Debug, Clone)]
pub(crate) struct VertexAttributes {
    pub stride: i32,
    attrs: Vec<InnerAttr>,
}

impl VertexAttributes {
    pub fn new(stride: i32, attrs: Vec<InnerAttr>) -> Self {
        Self { stride, attrs }
    }

    pub unsafe fn enable(&self, gl: &Context) {
        self.attrs
            .iter()
            .for_each(|attr| attr.enable(gl, self.stride));
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InnerAttr {
    pub location: u32,
    pub size: i32,
    pub data_type: u32,
    pub normalized: bool,
    pub offset: i32,
}

impl InnerAttr {
    #[inline(always)]
    fn from(attr: &VertexAttr, offset: i32) -> InnerAttr {
        Self {
            location: attr.location,
            size: attr.format.size(),
            data_type: attr.format.to_glow(),
            normalized: attr.format.normalized(),
            offset,
        }
    }

    #[inline(always)]
    unsafe fn enable(&self, gl: &Context, stride: i32) {
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_f32(
            self.location,
            self.size,
            self.data_type,
            self.normalized,
            stride,
            self.offset,
        );
    }
}

#[inline(always)]
unsafe fn set_stencil(gl: &Context, options: &PipelineOptions) {
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
unsafe fn set_depth_stencil(gl: &Context, options: &PipelineOptions) {
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
unsafe fn set_color_mask(gl: &Context, options: &PipelineOptions) {
    gl.color_mask(
        options.color_mask.r,
        options.color_mask.g,
        options.color_mask.b,
        options.color_mask.a,
    );
}

#[inline(always)]
unsafe fn set_culling(gl: &Context, options: &PipelineOptions) {
    match options.cull_mode.to_glow() {
        Some(mode) => {
            gl.enable(glow::CULL_FACE);
            gl.cull_face(mode);
        }
        _ => gl.disable(glow::CULL_FACE),
    }
}

#[inline(always)]
unsafe fn set_blend_mode(gl: &Context, options: &PipelineOptions) {
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
fn clean_pipeline(gl: &Context, pip: InnerPipeline) {
    let InnerPipeline {
        vertex,
        fragment,
        program,
        vao,
        ..
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
    gl: &Context,
    vertex_source: &str,
    fragment_source: &str,
    stride: i32,
    attrs: Vec<InnerAttr>,
) -> Result<InnerPipeline, String> {
    let vertex = create_shader(gl, glow::VERTEX_SHADER, vertex_source)?;
    let fragment = create_shader(gl, glow::FRAGMENT_SHADER, fragment_source)?;
    let program = create_program(gl, vertex, fragment)?;

    #[cfg(target_arch = "wasm32")]
    let uniform_locations = unsafe {
        let count = gl.get_active_uniforms(program);
        (0..count)
            .into_iter()
            .filter_map(|index| match gl.get_active_uniform(program, index) {
                Some(u) => match gl.get_uniform_location(program, &u.name) {
                    Some(loc) => Some(loc),
                    _ => {
                        // inform about uniforms outside of blocks that are missing
                        if !u.name.contains("") {
                            notan_log::debug!("Cannot get uniform location for: {}", u.name);
                        }
                        None
                    }
                },
                _ => None,
            })
            .collect::<Vec<_>>()
    };

    let vao = unsafe {
        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(vao));
        vao
    };

    let attrs = VertexAttributes::new(stride, attrs);

    Ok(InnerPipeline {
        vertex,
        fragment,
        program,
        vao,
        attrs,

        #[cfg(target_arch = "wasm32")]
        uniform_locations,
    })
}

#[inline(always)]
fn create_shader(gl: &Context, typ: u32, source: &str) -> Result<Shader, String> {
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

        let typ_name = match typ {
            glow::VERTEX_SHADER => "vertex".to_string(),
            glow::FRAGMENT_SHADER => "fragment".to_string(),
            _ => format!("unknown type ({})", typ),
        };

        Err(format!(
            "{} with {} shader: \n--\n{}\n--\n",
            err, typ_name, source
        ))
    }
}

#[inline(always)]
fn create_program(gl: &Context, vertex: Shader, fragment: Shader) -> Result<Program, String> {
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
