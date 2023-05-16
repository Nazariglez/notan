use crate::to_glow::*;
use glow::*;
use hashbrown::HashMap;
use notan_graphics::prelude::*;

#[cfg(debug_assertions)]
use hashbrown::HashSet;

pub(crate) struct InnerPipeline {
    pub vertex: Shader,
    pub fragment: Shader,
    pub program: Program,
    pub vao: VertexArray,
    pub uniform_locations: Vec<UniformLocation>,
    pub attrs_bound_to: HashMap<u32, u64>,
    pub texture_locations: HashMap<u32, UniformLocation>,
}

#[inline]
pub(crate) fn get_inner_attrs(attrs: &[VertexAttr]) -> (i32, Vec<InnerAttr>) {
    let mut stride = 0;
    let attrs = attrs
        .iter()
        .map(|attr| {
            let inner_attr = InnerAttr::from(attr, stride);
            stride += attr.format.bytes();
            inner_attr
        })
        .collect::<Vec<_>>();

    (stride, attrs)
}

impl InnerPipeline {
    #[inline(always)]
    pub fn new(
        gl: &Context,
        vertex_source: &str,
        fragment_source: &str,
        attrs: &[VertexAttr],
        texture_locations: &[(u32, String)],
    ) -> Result<Self, String> {
        let (stride, attrs) = get_inner_attrs(attrs);

        create_pipeline(
            gl,
            vertex_source,
            fragment_source,
            stride,
            attrs,
            texture_locations,
        )
    }

    // register the buffer id for each element in case we need to reset the vao attrs when the buffer change
    pub fn use_attrs(&mut self, buffer: u64, attrs: &VertexAttributes) -> bool {
        let mut reset = false;
        attrs.attrs.iter().for_each(|attr| {
            let old = self.attrs_bound_to.insert(attr.location, buffer);
            match old {
                None => reset = true,
                Some(old_id) => {
                    if old_id != buffer {
                        reset = true;
                    }
                }
            }
        });

        reset
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
            set_blend_mode(gl, options);
            #[cfg(not(target_arch = "wasm32"))]
            set_srgb_space(gl, options);
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VertexAttributes {
    pub stride: i32,
    attrs: Vec<InnerAttr>,
    vertex_step_mode: VertexStepMode,
}

impl VertexAttributes {
    pub fn new(stride: i32, attrs: Vec<InnerAttr>, vertex_step_mode: VertexStepMode) -> Self {
        Self {
            stride,
            attrs,
            vertex_step_mode,
        }
    }

    pub unsafe fn enable(&self, gl: &Context) {
        let step_mode = match self.vertex_step_mode {
            VertexStepMode::Vertex => 0,
            VertexStepMode::Instance => 1,
        };

        self.attrs
            .iter()
            .for_each(|attr| attr.enable(gl, self.stride, step_mode));
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
    unsafe fn enable(&self, gl: &Context, stride: i32, vertex_step_mode: u32) {
        gl.enable_vertex_attrib_array(self.location);
        gl.vertex_attrib_pointer_f32(
            self.location,
            self.size,
            self.data_type,
            self.normalized,
            stride,
            self.offset,
        );
        gl.vertex_attrib_divisor(self.location, vertex_step_mode);
    }
}

#[inline(always)]
unsafe fn set_stencil(gl: &Context, options: &PipelineOptions) {
    if should_disable_stencil(&options.stencil) {
        gl.disable(glow::STENCIL_TEST);
    } else if let Some(opts) = options.stencil {
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
#[cfg(not(target_arch = "wasm32"))]
fn set_srgb_space(gl: &Context, opts: &PipelineOptions) {
    unsafe {
        if opts.srgb_space {
            gl.enable(glow::FRAMEBUFFER_SRGB);
        } else {
            gl.disable(glow::FRAMEBUFFER_SRGB);
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
    _stride: i32,
    _attrs: Vec<InnerAttr>,
    texture_locations: &[(u32, String)],
) -> Result<InnerPipeline, String> {
    let vertex = create_shader(gl, glow::VERTEX_SHADER, vertex_source)?;
    let fragment = create_shader(gl, glow::FRAGMENT_SHADER, fragment_source)?;
    let program = create_program(gl, vertex, fragment)?;

    let mut texture_locations_map = HashMap::default();

    #[cfg(debug_assertions)]
    let mut not_used_textures: HashSet<String> = texture_locations
        .iter()
        .map(|(_loc, id)| id.clone())
        .collect();

    let uniform_locations = unsafe {
        let count = gl.get_active_uniforms(program);
        (0..count)
            .filter_map(|index| match gl.get_active_uniform(program, index) {
                Some(u) => {
                    // texture names does not contains . (Locals.property vs u_texture)
                    // this is used to match the name that naga set to the uniform textures
                    let is_tex_name = !u.name.contains(".");

                    match gl.get_uniform_location(program, &u.name) {
                        Some(loc) => {
                            let tex_loc = texture_locations.iter().find_map(|(tloc, id)| {
                                if id == &u.name {
                                    Some(*tloc)
                                } else {
                                    None
                                }
                            });

                            match tex_loc {
                                Some(tloc) => {
                                    #[cfg(debug_assertions)]
                                    {
                                        not_used_textures.remove(&u.name);
                                    }

                                    // register the texture uniform loc under the new loc provided by the user
                                    #[cfg(target_arch = "wasm32")]
                                    texture_locations_map.insert(tloc, loc.clone());

                                    #[cfg(not(target_arch = "wasm32"))]
                                    texture_locations_map.insert(tloc, loc);
                                }
                                None => {
                                    if is_tex_name {
                                        #[cfg(target_arch = "wasm32")]
                                        texture_locations_map.insert(index, loc.clone());

                                        #[cfg(not(target_arch = "wasm32"))]
                                        texture_locations_map.insert(index, loc);

                                        #[cfg(debug_assertions)]
                                        if let Some(tex_name) = texture_locations
                                            .iter()
                                            .find(|(i, _n)| *i == index)
                                            .map(|(_, n)| n)
                                        {
                                            not_used_textures.remove(tex_name);
                                            log::debug!(
                                                "Replacing uniform '{tex_name}' by '{}'",
                                                u.name
                                            );
                                        }
                                    }
                                }
                            }

                            Some(loc)
                        }
                        _ => {
                            // inform about uniforms outside of blocks that are missing
                            if !u.name.contains("") {
                                log::debug!("Cannot get uniform location for: {}", u.name);
                            }
                            None
                        }
                    }
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    };

    #[cfg(debug_assertions)]
    {
        for name in not_used_textures.iter() {
            panic!("Wrong texture location id: {name}");
        }
    }

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
        uniform_locations,
        attrs_bound_to: HashMap::default(),
        texture_locations: texture_locations_map,
    })
}

#[inline(always)]
fn create_shader(gl: &Context, typ: u32, source: &str) -> Result<Shader, String> {
    unsafe {
        let shader = gl.create_shader(typ)?;
        gl.shader_source(shader, source);
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
            _ => format!("unknown type ({typ})"),
        };

        Err(format!(
            "{err} with {typ_name} shader: \n--\n{source}\n--\n"
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
