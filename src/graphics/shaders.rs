use super::GlContext;
use crate::glm;
use crate::graphics::DrawData;
use crate::res::*;
use glow::*;
use hashbrown::HashMap;

type ShaderKey = glow::WebShaderKey;
type ProgramKey = glow::WebProgramKey;

/*TODO masking: https://stackoverflow.com/questions/46806063/how-stencil-buffer-and-masking-work
    https://jsfiddle.net/z11zhf01/1
    https://jsfiddle.net/gpkdrs93/
*/

pub struct Attribute {
    name: String,
    size: i32,
    data_type: u32,
    normalize: bool,
}

impl Attribute {
    pub fn new(name: &str, size: i32, data_type: u32, normalize: bool) -> Self {
        let name = name.to_string();
        Self {
            name,
            size,
            data_type,
            normalize,
        }
    }
}

struct AttributeData {
    attr: Attribute,
    location: u32,
    buffer: glow::WebBufferKey,
}

fn projection(w: f32, h: f32) -> glm::Mat3 {
    glm::mat3(2.0 / w, 0.0, -1.0, 0.0, -2.0 / h, 1.0, 0.0, 0.0, 1.0)
}

pub trait UniformType {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey);
}

impl UniformType for i32 {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_1_i32(Some(location), *self);
        }
    }
}

impl UniformType for (f32, f32) {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_2_f32(Some(location), self.0, self.1);
        }
    }
}

impl UniformType for glm::Mat3 {
    fn set_uniform_value(&self, gl: &GlContext, location: WebUniformLocationKey) {
        unsafe {
            gl.uniform_matrix_3_f32_slice(Some(location), false, &*m3_to_slice(self));
        }
    }
}

#[inline]
fn m3_to_slice(m: &glm::Mat3) -> *const [f32; 9] {
    m.as_slice().as_ptr() as *const [f32; 9]
}

pub struct Shader {
    vertex: ShaderKey,
    fragment: ShaderKey,
    program: ProgramKey,
    gl: GlContext,
    attributes: HashMap<String, AttributeData>,
    uniforms: HashMap<String, glow::WebUniformLocationKey>,
}

impl Shader {
    pub fn new(
        gl: &GlContext,
        vertex: &str,
        fragment: &str,
        mut attributes: Vec<Attribute>,
        uniforms: Vec<&str>,
    ) -> Result<Self, String> {
        let gl = gl.clone();
        let vertex = create_shader(&gl, glow::VERTEX_SHADER, vertex)?;
        let fragment = create_shader(&gl, glow::FRAGMENT_SHADER, fragment)?;

        let program = create_program(&gl, vertex, fragment)?;

        let mut attrs = HashMap::new();
        let mut uniform_list = HashMap::new();
        unsafe {
            while let Some(attr) = attributes.pop() {
                let location = gl.get_attrib_location(program, &attr.name) as u32;
                let buffer = gl.create_buffer()?;
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
                gl.enable_vertex_attrib_array(location);

                let stride = 0;
                let offset = 0;
                gl.vertex_attrib_pointer_f32(
                    location,
                    attr.size,
                    attr.data_type,
                    attr.normalize,
                    stride,
                    offset,
                );

                attrs.insert(
                    attr.name.clone(),
                    AttributeData {
                        attr,
                        location,
                        buffer,
                    },
                );
            }

            for uniform in uniforms {
                let u = gl
                    .get_uniform_location(program, uniform)
                    .ok_or(format!("Invalid uniform name: {}", uniform))?;

                uniform_list.insert(uniform.to_string(), u);
            }
        }

        Ok(Self {
            vertex: vertex,
            fragment: fragment,
            program: program,
            gl: gl,
            attributes: attrs,
            uniforms: uniform_list,
        })
    }

    pub fn useme(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    pub fn set_uniform<T: UniformType>(&self, name: &str, value: T) {
        if let Some(u) = self.uniforms.get(name) {
            value.set_uniform_value(&self.gl, *u);
        }
    }

    pub fn buffer(&self, name: &str) -> Option<WebBufferKey> {
        if let Some(attr) = self.attributes.get(name) {
            return Some(attr.buffer);
        }

        None
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_shader(self.vertex);
            self.gl.delete_shader(self.fragment);
            self.gl.delete_program(self.program);
        }
    }
}

fn create_shader(gl: &GlContext, typ: u32, source: &str) -> Result<ShaderKey, String> {
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
        Err(err)
    }
}

fn create_program(
    gl: &GlContext,
    vertex: ShaderKey,
    fragment: ShaderKey,
) -> Result<ProgramKey, String> {
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

const VERTICES: usize = 3;
const VERTICE_SIZE: usize = 2;
const COLOR_VERTICE_SIZE: usize = 4;

/*TODO check this: drawElements use u16 as indices, 65553 is the max on webgl1
    but drawArrays doesn't have this limit.
    To use drawElements without limit on webgl1 also exists the this extension: OES_element_index_uint https://developer.mozilla.org/en-US/docs/Web/API/OES_element_index_uint
    On webgl2 the limit doesn't exists, but you need to use UNSIGNED_INT as index https://webgl2fundamentals.org/webgl/lessons/webgl2-whats-new.html (i32)
    A way to do this on webgl1 is:
        - try to get the extension
        - If it fails use drawElements by default
        - fallback if indices > 65553 to drawArrays
    On webgl2 we should probably use just drawElements with i32 indices
    --
    # help https://computergraphics.stackexchange.com/questions/3637/how-to-use-32-bit-integers-for-element-indices-in-webgl-1-0
    # var canvas = document.createElement("canvas");
    var gl = canvas.getContext("webgl");
    console.log(gl.getExtension("OES_element_index_uint"));
*/
const MAX_PER_BATCH: usize = 65000 / (VERTICES * COLOR_VERTICE_SIZE);

/* TODO for work with vaos on webgl1:
    https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object
    https://www.khronos.org/registry/webgl/extensions/OES_vertex_array_object/
    https://medium.com/@david.komer/dude-wheres-my-data-vao-s-in-webgl-896631783895
    https://stackoverflow.com/a/46143967
*/

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub struct ColorBatcher {
    shader: Shader,
    vao: glow::WebVertexArrayKey,
    index: i32,
    vertex: Vec<f32>,
    vertex_color: Vec<f32>,
}

impl ColorBatcher {
    pub fn new(gl: &GlContext, data: &DrawData) -> Result<Self, String> {
        let vao = create_vao(gl)?;
        let shader = create_color_shader(gl)?;
        Ok(Self {
            shader,
            vao,
            index: 0,
            vertex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            vertex_color: vec![0.0; MAX_PER_BATCH * VERTICES * COLOR_VERTICE_SIZE],
        })
    }

    pub fn begin(&mut self) {
        self.index = 0;
    }

    pub fn flush(&mut self, gl: &GlContext, data: &DrawData) {
        if self.index == 0 {
            return;
        }

        self.use_shader(data);
        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            //TODO pass the whole slice or just pass what we need to save bandwidth? (is this really that worth it?)
            let v_max = self.index as usize * VERTICES * VERTICE_SIZE;
            let vc_max = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
            self.bind_buffer(gl, "a_position", &self.vertex[0..v_max], 0);
            self.bind_buffer(gl, "a_color", &self.vertex_color[0..vc_max], 0);

            let primitives = glow::TRIANGLES;
            let offset = 0;
            let count = self.index * VERTICES as i32;
            gl.draw_arrays(primitives, offset, count);
        }

        self.index = 0;
    }

    fn use_shader(&self, data: &DrawData) {
        let shader = match &data.shader {
            Some(s) => s,
            _ => &self.shader,
        };
        shader.useme();
        shader.set_uniform("u_matrix", data.projection);
    }

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], _offset: usize) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, self.shader.buffer(name));
            let buff = vf_to_u8(&data);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buff, glow::STATIC_DRAW);
        }
    }

    pub fn draw(&mut self, gl: &GlContext, data: &DrawData, vertex: &[f32], color: Option<&[f32]>) {
        let count = (vertex.len() / 6) as i32; //vertex.len() / (vertices*size)
        let next = self.index + count;

        if next >= (MAX_PER_BATCH as i32) {
            self.flush(gl, data);
        }

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        for (i, _) in vertex.iter().enumerate().step_by(2) {
            if let (Some(v1), Some(v2)) = (vertex.get(i), vertex.get(i + 1)) {
                let v = data.transform.matrix() * glm::vec3(*v1, *v2, 1.0);
                self.vertex[offset] = v.x;
                self.vertex[offset + 1] = v.y;
                offset += 2;
            }
        }

        let color = match color {
            Some(c) => c.to_vec(),
            None => {
                let (r, g, b, a) = data.color.to_rgba();
                let mut color = vec![];
                (0..VERTICES * count as usize).for_each(|_| {
                    color.push(r);
                    color.push(g);
                    color.push(b);
                    color.push(a);
                });
                color
            }
        };

        let mut offset = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
        color.iter().enumerate().for_each(|(i, c)| {
            let is_alpha = (i + 1) % 4 == 0;
            self.vertex_color[offset] = if is_alpha { *c * data.alpha } else { *c };
            offset += 1;
        });

        self.index += count;
    }
}

fn create_vao(gl: &GlContext) -> Result<WebVertexArrayKey, String> {
    unsafe {
        let vao = gl.create_vertex_array()?;
        gl.bind_vertex_array(Some(vao));

        Ok(vao)
    }
}

const COLOR_VERTEX: &str = r#"#version 300 es
in vec2 a_position;
in vec4 a_color;
out vec4 v_color;

uniform mat3 u_matrix;

void main() {
  v_color = a_color;
  gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}
"#;

const COLOR_FRAGMENT: &str = r#"#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

void main() {
    outColor = v_color;
}
"#;

fn create_color_shader(gl: &GlContext) -> Result<Shader, String> {
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
    ];

    let uniforms = vec!["u_matrix"];
    Ok(Shader::new(
        gl,
        COLOR_VERTEX,
        COLOR_FRAGMENT,
        attrs,
        uniforms,
    )?)
}

pub struct SpriteBatcher {
    shader: Shader,
    vao: glow::WebVertexArrayKey,
    index: i32,
    vertex: Vec<f32>,
    vertex_color: Vec<f32>,
    vertex_tex: Vec<f32>,
    current_tex: Option<glow::WebTextureKey>,
}

impl SpriteBatcher {
    pub fn new(gl: &GlContext, data: &DrawData) -> Result<Self, String> {
        let vao = create_vao(gl)?;
        let shader = create_pattern_shader(gl)?;
        Ok(Self {
            shader,
            vao,
            index: 0,
            vertex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            vertex_color: vec![0.0; MAX_PER_BATCH * VERTICES * COLOR_VERTICE_SIZE],
            vertex_tex: vec![0.0; MAX_PER_BATCH * VERTICES * VERTICE_SIZE],
            current_tex: None,
        })
    }

    fn use_shader(&self, data: &DrawData) {
        let shader = match &data.shader {
            Some(s) => s,
            _ => &self.shader,
        };
        shader.useme();
        shader.set_uniform("u_matrix", data.projection);
        shader.set_uniform("u_texture", 0);
    }

    fn bind_buffer(&self, gl: &GlContext, name: &str, data: &[f32], _offset: usize) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, self.shader.buffer(name));
            let buff = vf_to_u8(&data);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buff, glow::STATIC_DRAW);
        }
    }

    pub fn flush(&mut self, gl: &GlContext, data: &DrawData) {
        if self.index == 0 {
            return;
        }

        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            self.use_shader(data);

            gl.active_texture(glow::TEXTURE0);
            //            log(&format!("{:?}", tex_data));
            gl.bind_texture(glow::TEXTURE_2D, self.current_tex);

            self.bind_buffer(gl, "a_position", &self.vertex, 0);
            self.bind_buffer(gl, "a_texcoord", &self.vertex_tex, 0);
            self.bind_buffer(gl, "a_color", &self.vertex_color, 0);
            let count = self.index * VERTICES as i32;
            gl.draw_arrays(glow::TRIANGLES, 0, count);
        }

        self.index = 0;
    }

    pub fn draw(
        &mut self,
        gl: &GlContext,
        data: &DrawData,
        x: f32,
        y: f32,
        img: &mut Texture,
        source_x: f32,
        source_y: f32,
        source_width: f32,
        source_height: f32,
        color: Option<&[f32]>,
    ) {
        if !img.is_loaded() {
            return;
        }

        let tex = match img.tex() {
            Some(t) => t,
            _ => init_graphic_texture(gl, img).unwrap(),
        };

        let ww = img.width();
        let hh = img.height();

        let sw = if source_width == 0.0 {
            ww
        } else {
            source_width
        };
        let sh = if source_height == 0.0 {
            hh
        } else {
            source_height
        };

        let vertex = [
            x,
            y,
            x,
            y + sh,
            x + sw,
            y,
            x + sw,
            y,
            x,
            y + sh,
            x + sw,
            y + sh,
        ];

        if self.current_tex.is_none() {
            self.current_tex = Some(tex);
        }

        if let Some(t) = self.current_tex {
            if t != tex {
                self.flush(gl, data);
            } else {
                self.current_tex = Some(tex);
            }
        }

        let count = (vertex.len() / 6) as i32;
        let next = self.index + count;

        if next >= (MAX_PER_BATCH as i32) {
            self.flush(gl, data);
        }

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        for (i, _) in vertex.iter().enumerate().step_by(2) {
            if let (Some(v1), Some(v2)) = (vertex.get(i), vertex.get(i + 1)) {
                let v = data.transform.matrix() * glm::vec3(*v1, *v2, 1.0);
                self.vertex[offset] = v.x;
                self.vertex[offset + 1] = v.y;
                offset += 2;
            }
        }

        let x1 = source_x / ww;
        let y1 = source_y / hh;
        let x2 = (source_x + sw) / ww;
        let y2 = (source_y + sh) / hh;

        let mut offset = self.index as usize * VERTICES * VERTICE_SIZE;
        let vertex_tex = [x1, y1, x1, y2, x2, y1, x2, y1, x1, y2, x2, y2];
        vertex_tex.iter().for_each(|v| {
            self.vertex_tex[offset] = *v;
            offset += 1;
        });

        let color = match color {
            Some(c) => c.to_vec(),
            None => {
                let (r, g, b, a) = data.color.to_rgba();
                let mut color = vec![];
                (0..VERTICES * count as usize).for_each(|_| {
                    color.push(r);
                    color.push(g);
                    color.push(b);
                    color.push(a);
                });
                color
            }
        };

        let mut offset = self.index as usize * VERTICES * COLOR_VERTICE_SIZE;
        color.iter().enumerate().for_each(|(i, c)| {
            let is_alpha = (i + 1) % 4 == 0;
            self.vertex_color[offset] = if is_alpha { *c * data.alpha } else { *c };
            offset += 1;
        });

        self.index += count;
    }
}

fn init_graphic_texture(gl: &GlContext, img: &mut Texture) -> Result<glow::WebTextureKey, String> {
    let gt = create_gl_texture(gl, &img.data().borrow().as_ref().unwrap())?;

    let tex = gt.tex;

    img.data().borrow_mut().as_mut().unwrap().init_graphics(gt);

    Ok(tex)
}

const SPRITE_VERTEX: &str = r#"#version 300 es
in vec2 a_position;
in vec4 a_color;
out vec4 v_color;

in vec2 a_texcoord;
out vec2 v_texcoord;

uniform mat3 u_matrix;

void main() {
  v_color = a_color;
  v_texcoord = a_texcoord;
  gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}
"#;

//TODO https://thebookofshaders.com/09/

const SPRITE_FRAGMENT: &str = r#"#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

in vec2 v_texcoord;
uniform sampler2D u_texture;

void main() {
    outColor = texture(u_texture, v_texcoord) * v_color;
}
"#;

fn create_sprite_shader(gl: &GlContext) -> Result<Shader, String> {
    //https://webgl2fundamentals.org/webgl/lessons/webgl-2d-drawimage.html
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
        Attribute::new("a_texcoord", 2, glow::FLOAT, true),
    ];

    let uniforms = vec!["u_matrix", "u_texture"];
    Ok(Shader::new(
        gl,
        SPRITE_VERTEX,
        SPRITE_FRAGMENT,
        attrs,
        uniforms,
    )?)
}

#[derive(Debug, Clone)]
pub(crate) struct GraphicTexture {
    pub gl: GlContext,
    pub tex: glow::WebTextureKey,
}

fn create_gl_texture(gl: &GlContext, data: &TextureData) -> Result<GraphicTexture, String> {
    unsafe {
        let tex = gl.create_texture()?;
        gl.bind_texture(glow::TEXTURE_2D, Some(tex));

        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::NEAREST as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::NEAREST as i32,
        );

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            data.width,
            data.height,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(&data.raw),
        );

        //TODO mipmaps? gl.generate_mipmap(glow::TEXTURE_2D);
        gl.bind_texture(glow::TEXTURE_2D, None);
        Ok(GraphicTexture {
            gl: gl.clone(),
            tex: tex,
        })
    }
}

const PATTERN_VERTEX: &str = r#"#version 300 es
in vec2 a_position;
in vec4 a_color;
out vec4 v_color;

in vec2 a_texcoord;
out vec2 v_texcoord;
out float v_scale;

uniform mat3 u_matrix;

void main() {
  v_scale = 10.0;
  vec2 coord = a_texcoord;
  vec2 size = vec2(64.0, 64.0) * v_scale;
  vec2 offset = fract((size - vec2(32.0, 32.0)) / size);

//  coord = fract(coord*1.1);
  coord += offset;
  v_color = a_color;
  v_texcoord = coord;
  gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}
"#;

//TODO https://thebookofshaders.com/09/

const PATTERN_FRAGMENT: &str = r#"#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

in vec2 v_texcoord;
uniform sampler2D u_texture;

in float v_scale;

void main() {
    vec2 coord = v_texcoord;
    coord = fract(coord*v_scale);
//    coord.x += 0.0;
    outColor = texture(u_texture, coord) * v_color;
}
"#;

fn create_pattern_shader(gl: &GlContext) -> Result<Shader, String> {
    //https://webgl2fundamentals.org/webgl/lessons/webgl-2d-drawimage.html
    let attrs = vec![
        Attribute::new("a_position", 2, glow::FLOAT, false),
        Attribute::new("a_color", 4, glow::FLOAT, false),
        Attribute::new("a_texcoord", 2, glow::FLOAT, true),
    ];

    let uniforms = vec!["u_matrix", "u_texture"];
    Ok(Shader::new(
        gl,
        PATTERN_VERTEX,
        PATTERN_FRAGMENT,
        attrs,
        uniforms,
    )?)
}

