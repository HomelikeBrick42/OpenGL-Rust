use gl::types::*;
use std::os::raw::{ c_void };

pub struct OpenGLIndexBuffer {
    id: GLuint,
}

impl OpenGLIndexBuffer {
    pub fn new(data: &[u32]) -> OpenGLIndexBuffer {
        unsafe {
            let mut index_buffer = 0;
            gl::GenBuffers(1, &mut index_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                &data[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            return OpenGLIndexBuffer {
                id: index_buffer,
            };
        }
    }

    pub fn set_data(&mut self, data: &[u32]) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                &data[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn un_bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for OpenGLIndexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}
