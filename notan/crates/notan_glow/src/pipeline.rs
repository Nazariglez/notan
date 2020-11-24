use glow::*;
use std::rc::Rc;

pub(crate) struct InnerPipeline {
    pub vertex: Shader,
    pub fragment: Shader,
    pub program: Program,
    pub vao: VertexArray,
}

pub(crate) fn clean_pipeline(gl: &Rc<Context>, pip: InnerPipeline) {
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

pub(crate) fn create_pipeline(
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
