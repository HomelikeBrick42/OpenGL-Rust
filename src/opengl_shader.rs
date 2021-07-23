use gl::types::*;
use std::ffi::{ CString };

pub struct OpenGLShader {
    id: GLuint,
}

impl OpenGLShader {
    pub fn new(vertex_source: &str, fragment_source: &str) -> OpenGLShader {
        unsafe {
            let vertex_shader = OpenGLShader::create_shader(vertex_source, gl::VERTEX_SHADER);
            let fragment_shader = OpenGLShader::create_shader(fragment_source, gl::FRAGMENT_SHADER);

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut shader_linked = gl::FALSE as GLint;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut shader_linked);
            if shader_linked != gl::TRUE as GLint {
                let mut info_log_length = 0;
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut info_log_length);

                let mut info_log = Vec::with_capacity(info_log_length as usize);
                info_log.set_len(info_log_length as usize - 1);
                gl::GetProgramInfoLog(shader_program, info_log_length, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                panic!("Shader Linking Failed:\n{}", std::str::from_utf8(&info_log).unwrap());
            }

            gl::DetachShader(shader_program, vertex_shader);
            gl::DeleteShader(vertex_shader);

            gl::DetachShader(shader_program, fragment_shader);
            gl::DeleteShader(fragment_shader);

            return OpenGLShader {
                id: shader_program
            };
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn un_bind(&self) {
        unsafe { gl::UseProgram(0); }
    }

    unsafe fn create_shader(shader_source: &str, shader_type: GLenum) -> GLuint {
        let shader = gl::CreateShader(shader_type);
        let c_string_shader_source = CString::new(shader_source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_string_shader_source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut shader_compiled = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut shader_compiled);
        if shader_compiled != gl::TRUE as GLint {
            let mut info_log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);

            let mut info_log = Vec::with_capacity(info_log_length as usize);
            info_log.set_len(info_log_length as usize - 1);
            gl::GetShaderInfoLog(shader, info_log_length, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            panic!("{} Shader Compilation Failed:\n{}", if shader_type == gl::VERTEX_SHADER { "Vertex" } else { "Fragment" }, std::str::from_utf8(&info_log).unwrap());
        }

        return shader;
    }
}

impl Drop for OpenGLShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}
