use gl::types::*;
use std::os::raw::{ c_void };

pub struct OpenGLVertexBuffer {
    id: GLuint,
}

impl OpenGLVertexBuffer {
    pub fn new<T>(data: &[T]) -> OpenGLVertexBuffer {
        unsafe {
            let mut vertex_buffer = 0;
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of_val(&data[0])) as GLsizeiptr,
                &data[0] as *const T as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            return OpenGLVertexBuffer {
                id: vertex_buffer,
            };
        }
    }

    pub fn set_data<T>(&mut self, data: &[T]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of_val(&data[0])) as GLsizeiptr,
                &data[0] as *const T as *const c_void,
                gl::DYNAMIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn un_bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for OpenGLVertexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}
