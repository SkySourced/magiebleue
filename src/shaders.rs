use std::{ffi::CString, fs};

use gl::types::{self, GLenum, GLuint};

use crate::functions::get_error;

pub struct ShaderProgram(pub GLuint);
impl ShaderProgram {
    pub fn new() -> Option<Self> {
        let prog = unsafe { gl::CreateProgram() };
        if prog != 0 { Some(Self(prog)) } else { None }
    }

    /// Attaches a shader to this program
    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.0, shader.0) }
    }

    /// Links all attached shaders to this program
    pub fn link_program(&self) {
        unsafe { gl::LinkProgram(self.0) };
    }

    /// Checks if linking was successful
    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe {
            gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success);
        }
        success == i32::from(gl::TRUE)
    }

    /// Reads the log from this shader program
    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    /// Sets this program as the active shader program
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }

    /// Marks the program for deletion. Will delete as soon as it becomes inactive
    pub fn delete(self) {
        unsafe {
            gl::DeleteProgram(self.0);
        }
    }

    /// Gets the location of a uniform in this shader.
    pub fn get_uniform_location(&self, uniform: &str) -> types::GLint {
        let loc = unsafe {
            gl::GetUniformLocation(
                self.0,
                CString::new(uniform)
                    .expect("should be a valid cstring")
                    .as_ptr(),
            )
        };
        get_error(Some("ShaderProgram::get_uniform_location"));
        loc
    }

    /// Compiles a complete shader program from a vsh source and a fsh source.
    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let prog = Self::new().ok_or_else(|| "Couldn't allocate a shader program".to_string())?;
        let vsh = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex compile error: {}", e))?;
        let fsh = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment compile error: {}", e))?;
        prog.attach_shader(&vsh);
        prog.attach_shader(&fsh);
        prog.link_program();
        vsh.delete();
        fsh.delete();
        if prog.link_success() {
            Ok(prog)
        } else {
            let out = format!("Linking error: {}", prog.info_log());
            prog.delete();
            Err(out)
        }
    }

    /// Compiles a complete shader program from a vsh filepath and fsh filepath
    pub fn from_vert_frag_file(vert: &str, frag: &str) -> Result<Self, String> {
        ShaderProgram::from_vert_frag(
            fs::read_to_string(vert)
                .map_err(|e| format!("Vertex read error: {}", e))?
                .as_str(),
            fs::read_to_string(frag)
                .map_err(|e| format!("Fragment read error: {}", e))?
                .as_str(),
        )
    }
}

pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as _,
    Fragment = gl::FRAGMENT_SHADER as _,
}

pub struct Shader(pub GLuint);
impl Shader {
    /// Creates a new shader
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { gl::CreateShader(ty as GLenum) };
        if shader != 0 {
            Some(Self(shader))
        } else {
            None
        }
    }

    /// Sets new source to this shader, replacing any existing source
    pub fn set_source(&self, src: &str) {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }

    /// Compiles the shader using the given source
    pub fn compile(&self) {
        unsafe { gl::CompileShader(self.0) };
    }

    /// Checks if the last compilation was successful
    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut compiled) };
        compiled == i32::from(gl::TRUE)
    }

    /// Gets the shader's info log
    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    /// Marks a shader for deletion. Does not delete immediately
    pub fn delete(self) {
        unsafe { gl::DeleteShader(self.0) };
    }

    /// Compiles a full shader from source & type, providing the compiled shader ID or an error message
    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty).ok_or_else(|| "Couldn't allocate new shader".to_string())?;
        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.delete();
            Err(out)
        }
    }
}
