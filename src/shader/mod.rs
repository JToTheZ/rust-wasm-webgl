use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlProgram, WebGlShader, WebGlUniformLocation};

#[derive(Clone, Debug)]
pub enum GLSLSource {
    Vertex(String),
    Fragment(String),
}

impl GLSLSource {
    pub fn compile(&self, context: &GL) -> Result<WebGlShader, String> {
        let shader = match self {
           GLSLSource::Vertex(source) => {
                let shader = context.create_shader(GL::VERTEX_SHADER).ok_or_else(|| String::from("Unable to create shader object"))?;
                context.shader_source(&shader, source);
                context.compile_shader(&shader);
                shader
            },
            GLSLSource::Fragment(source) => {
                let shader = context.create_shader(GL::FRAGMENT_SHADER).ok_or_else(|| String::from("Unable to create shader object"))?;
                context.shader_source(&shader, source);
                context.compile_shader(&shader);
                shader
            }
        };

        if context.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
            Ok(shader)
        } else {
            Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown shader compilation error")))
        }
    }
}

#[derive(Clone, Debug)]
pub enum GLSLPrimitive {
    Bool,
    Double,
    Float,
    Int,
    UInt,
}

#[derive(Clone, Debug)]
pub enum GLSLUniform {
    Scalar(WebGlUniformLocation, GLSLPrimitive),
    Vector(WebGlUniformLocation, GLSLPrimitive, usize),
    MatrixN(WebGlUniformLocation, GLSLPrimitive, usize),
    MatrixNM(WebGlUniformLocation, GLSLPrimitive, usize, usize),
}
#[derive(Clone, Debug)]
pub struct GLSLAttribute {

}

#[derive(Clone, Debug)]
pub struct Program {
    webgl_handle: Option<WebGlProgram>,
    shaders: Vec<GLSLSource>,
    uniforms: Vec<GLSLUniform>,
    attributes: Vec<GLSLAttribute>,
}

impl Program {
    pub fn new(source: Vec<GLSLSource>) -> Self {
        Program {
            webgl_handle: None,
            shaders: source,
            uniforms: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn use_program(&self, context: &GL) -> Result<(), String> {
        context.use_program(Some(self.webgl_handle.as_ref().ok_or_else(|| String::from("Unable to attach program"))?));
        Ok(())
    }

    pub fn link(&mut self, context: &GL) -> Result<(), String> {
        let program = context.create_program().ok_or_else(|| String::from("Unable to create shader program object."))?;
        for shader in &self.shaders {
            let gl_handle = shader.compile(context)?;
            context.attach_shader(&program, &gl_handle);
        }
        context.link_program(&program);

        if context.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
            self.webgl_handle = Some(program);
            Ok(())
        } else {
            Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    pub fn render(&self, context: &GL) -> Result<(), String> {
        
        Ok(())
    }
}

pub fn default_shader(context: &GL) -> Result<Program, String> {
    let mut program = Program::new(vec![GLSLSource::Vertex(include_str!("default/default_vertex_shader.glsl").into()), GLSLSource::Fragment(include_str!("default/default_fragment_shader.glsl").into())]);
    program.link(context)?;
    Ok(program)
}