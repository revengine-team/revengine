pub mod resources;

use gl;
use std::ffi::CString;
use std::io::Error;
use std::io::ErrorKind;
use std::ptr;
use std::str;

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn new(vertex_source: &str, fragment_source: &str) -> ShaderProgram {
        let vertex_id = ShaderProgram::make_shader(gl::VERTEX_SHADER, vertex_source)
            .expect("Vertex shader compile error");

        let fragment_id = ShaderProgram::make_shader(gl::FRAGMENT_SHADER, fragment_source)
            .expect("Fragment shader compile error");

        let program_id = ShaderProgram::make(vertex_id, fragment_id).expect("Program link error");

        ShaderProgram { id: program_id }
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn make_shader(shader_type: gl::types::GLenum, shader_source: &str) -> Result<u32, Error> {
        let shader_id = unsafe {
            let id = gl::CreateShader(shader_type);
            let shader_source_cstr = CString::new(shader_source.as_bytes()).unwrap();

            gl::ShaderSource(id, 1, &shader_source_cstr.as_ptr(), ptr::null());
            gl::CompileShader(id);

            id
        };

        unsafe {
            let mut success = gl::FALSE as gl::types::GLint;
            let mut info_log = Vec::with_capacity(1024);
            #[allow(clippy::uninit_vec)]
            info_log.set_len(1023);
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(
                    shader_id,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );

                return Err(Error::new(
                    ErrorKind::Other,
                    String::from_utf8_lossy(&info_log),
                ));
            }
        }

        Ok(shader_id)
    }

    fn make(vertex_shader_id: u32, fragment_shader_id: u32) -> Result<u32, Error> {
        let shader_program_id = unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader_id);
            gl::AttachShader(id, fragment_shader_id);
            gl::LinkProgram(id);

            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);

            id
        };

        unsafe {
            let mut success = gl::FALSE as gl::types::GLint;
            let mut info_log = Vec::with_capacity(1024);
            #[allow(clippy::uninit_vec)]
            info_log.set_len(1023);
            gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success);

            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(
                    shader_program_id,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );

                return Err(Error::new(
                    ErrorKind::Other,
                    String::from_utf8_lossy(&info_log),
                ));
            }
        }

        Ok(shader_program_id)
    }
}
