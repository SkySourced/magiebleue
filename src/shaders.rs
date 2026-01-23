use std::{ffi::CString, fs};

use gl::types::{self, GLenum, GLuint};
use ultraviolet::Mat4;

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

    /// Sets MVP matrices in shader uniforms. Uses uniform names `model`, `view`, and `proj`.
    pub fn set_matrix_uniforms(&self, model: &Mat4, view: &Mat4, proj: &Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location("model"),
                1,
                gl::FALSE,
                model.as_array().as_ptr(),
            );

            gl::UniformMatrix4fv(
                self.get_uniform_location("view"),
                1,
                gl::FALSE,
                view.as_array().as_ptr(),
            );

            gl::UniformMatrix4fv(
                self.get_uniform_location("proj"),
                1,
                gl::FALSE,
                proj.as_array().as_ptr(),
            );
        }
    }

    /// Compiles a complete shader program from mandatory vertex & fragment and optional tessellation control/evaluation & geometry shader sources.
    pub fn from_string(
        vert: &str,
        tesc: Option<&str>,
        tese: Option<&str>,
        geom: Option<&str>,
        frag: &str,
    ) -> Result<Self, String> {
        let prog = Self::new().ok_or_else(|| "Couldn't allocate a shader program".to_string())?;
        let vsh = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex compile error: {}", e))?;
        let fsh = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment compile error: {}", e))?;
        let tesc_sh: Option<Shader>;
        if tesc.is_some() {
            tesc_sh = Some(
                Shader::from_source(
                    ShaderType::TessellationControl,
                    tesc.expect("tesc source should exist"),
                )
                .map_err(|e| format!("Tessellation control compile error: {}", e))?,
            );
            prog.attach_shader(tesc_sh.as_ref().expect("tesc shader should exist"));
        } else {
            tesc_sh = None;
        }
        let tese_sh: Option<Shader>;
        if tese.is_some() {
            tese_sh = Some(
                Shader::from_source(
                    ShaderType::TessellationEvaluation,
                    tese.expect("tese source should exist"),
                )
                .map_err(|e| format!("Tessellation evaluation compile error: {}", e))?,
            );
            prog.attach_shader(tese_sh.as_ref().expect("tese shader should exist"));
        } else {
            tese_sh = None;
        }
        let geom_sh: Option<Shader>;
        if geom.is_some() {
            geom_sh = Some(
                Shader::from_source(
                    ShaderType::TessellationControl,
                    geom.expect("geom source should exist"),
                )
                .map_err(|e| format!("Geometry compile error: {}", e))?,
            );
            prog.attach_shader(geom_sh.as_ref().expect("geom shader should exist"));
        } else {
            geom_sh = None;
        }
        prog.attach_shader(&vsh);
        prog.attach_shader(&fsh);
        prog.link_program();
        vsh.delete();
        fsh.delete();
        if let Some(tesc) = tesc_sh {
            tesc.delete();
        }
        if let Some(tese) = tese_sh {
            tese.delete();
        }
        if let Some(geom) = geom_sh {
            geom.delete();
        }

        if prog.link_success() {
            Ok(prog)
        } else {
            let out = format!("Linking error: {}", prog.info_log());
            prog.delete();
            Err(out)
        }
    }

    /// Compiles a complete shader program from a mandatory vertex & fragment and optional tessellation control/evaluation & geometry shader filepaths.
    pub fn from_filepath(
        vert: &str,
        tesc: Option<&str>,
        tese: Option<&str>,
        geom: Option<&str>,
        frag: &str,
    ) -> Result<Self, String> {
        let tesc_src: Result<Option<String>, String> = if let Some(tesc) = tesc {
            Ok(Some(fs::read_to_string(tesc).map_err(|e| {
                format!("Tessellation control compile error: {}", e)
            })?))
        } else {
            Ok(None)
        };
        let tese_src: Result<Option<String>, String> = if let Some(tese) = tese {
            Ok(Some(fs::read_to_string(tese).map_err(|e| {
                format!("Tessellation evaluation compile error: {}", e)
            })?))
        } else {
            Ok(None)
        };
        let geom_src: Result<Option<String>, String> = if let Some(geom) = geom {
            Ok(Some(
                fs::read_to_string(geom).map_err(|e| format!("Geometry compile error: {}", e))?,
            ))
        } else {
            Ok(None)
        };

        ShaderProgram::from_string(
            fs::read_to_string(vert)
                .map_err(|e| format!("Vertex compile error: {}", e))?
                .as_str(),
            tesc_src?.as_deref(),
            tese_src?.as_deref(),
            geom_src?.as_deref(),
            fs::read_to_string(frag)
                .map_err(|e| format!("Fragment compile error: {}", e))?
                .as_str(),
        )
    }
}

pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as _,
    Fragment = gl::FRAGMENT_SHADER as _,
    Geometry = gl::GEOMETRY_SHADER as _,
    TessellationControl = gl::TESS_CONTROL_SHADER as _,
    TessellationEvaluation = gl::TESS_EVALUATION_SHADER as _,
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
