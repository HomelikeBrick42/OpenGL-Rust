use crate::opengl_vertex_buffer::{ OpenGLVertexBuffer };

use gl::types::*;

#[derive(Clone, Copy)]
pub enum BufferElement {
    Float1,
    Float2,
    Float3,
    Float4,
}

impl BufferElement {
    pub fn get_stride(self) -> usize {
        match self {
            BufferElement::Float1 => 1 * std::mem::size_of::<GLfloat>(),
            BufferElement::Float2 => 2 * std::mem::size_of::<GLfloat>(),
            BufferElement::Float3 => 3 * std::mem::size_of::<GLfloat>(),
            BufferElement::Float4 => 4 * std::mem::size_of::<GLfloat>(),
        }
    }

    pub fn get_count(self) -> usize {
        match self {
            BufferElement::Float1 => 1,
            BufferElement::Float2 => 2,
            BufferElement::Float3 => 3,
            BufferElement::Float4 => 4,
        }
    }

    pub fn get_gl_type(self) -> GLenum {
        match self {
            BufferElement::Float1 => gl::FLOAT,
            BufferElement::Float2 => gl::FLOAT,
            BufferElement::Float3 => gl::FLOAT,
            BufferElement::Float4 => gl::FLOAT,
        }
    }
}

pub struct OpenGLVertexArray {
    id: GLuint,
    vertex_buffers: Vec<(OpenGLVertexBuffer, Vec<BufferElement>)>,
}

impl OpenGLVertexArray {
    pub fn new() -> OpenGLVertexArray {
        unsafe {
            let mut vertex_array = 0;
            gl::GenVertexArrays(1, &mut vertex_array);

            return OpenGLVertexArray {
                id: vertex_array,
                vertex_buffers: Vec::new(),
            };
        }
    }

    pub fn add_vertex_buffer<'a>(&'a mut self, buffer: OpenGLVertexBuffer, layout: &[BufferElement]) -> &'a OpenGLVertexBuffer {
        self.vertex_buffers.push((buffer, layout.to_vec()));

        let mut stride = 0;
        for (_, layout) in &self.vertex_buffers {
            for element in layout {
                stride += element.get_stride();
            }
        }

        self.bind();

        let mut index = 0;
        let mut offset = 0;
        for (vertex_buffer, layout) in &self.vertex_buffers {
            vertex_buffer.bind();
            for element in layout {
                unsafe {
                    gl::EnableVertexAttribArray(index);
                    gl::VertexAttribPointer(
                        index,
                        element.get_count() as GLint,
                        element.get_gl_type(),
                        gl::FALSE,
                        stride as GLsizei,
                        offset as *const GLvoid,
                    );
                }
                index += 1;
                offset += element.get_stride();
            }
            vertex_buffer.un_bind();
        }

        self.un_bind();
        
        return &self.vertex_buffers.last().unwrap().0;
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn un_bind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for OpenGLVertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
