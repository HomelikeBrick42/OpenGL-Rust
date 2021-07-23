#![allow(dead_code)]

mod vector3;
mod vertex;
mod opengl_shader;
mod opengl_vertex_buffer;
mod opengl_vertex_array;
mod opengl_index_buffer;

use crate::vector3::{ Vector3 };
use crate::vertex::{ Vertex };
use crate::opengl_shader::{ OpenGLShader };
use crate::opengl_vertex_buffer::{ OpenGLVertexBuffer };
use crate::opengl_vertex_array::{ OpenGLVertexArray, BufferElement };
use crate::opengl_index_buffer::{ OpenGLIndexBuffer };

extern crate glfw;
extern crate gl;
extern crate num;

use glfw::{ Context, Key, Action };
use gl::types::*;

use std::sync::mpsc::{ Receiver };

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize GLFW!");

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 4));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let (mut window, events) = glfw.create_window(1280, 720, "Rust OpenGL Window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window!");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vertex_shader_source: &str = r#"
#version 440 core

layout(location = 0) in vec4 a_Position;

void main() {
    gl_Position = a_Position;
}
"#;

    let fragment_shader_source: &str = r#"
#version 440 core

layout(location = 0) out vec4 o_Color;

void main() {
    o_Color = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

    let shader = OpenGLShader::new(vertex_shader_source, fragment_shader_source);

    let mut vertex_array = OpenGLVertexArray::new();

    let vertices = [
        Vertex::new(Vector3::new( 0.0,  0.5, 0.0)),
        Vertex::new(Vector3::new( 0.5, -0.5, 0.0)),
        Vertex::new(Vector3::new(-0.5, -0.5, 0.0)),
    ];
    let _vertex_buffer = vertex_array.add_vertex_buffer(OpenGLVertexBuffer::new(&vertices), &[BufferElement::Float3]);

    let indices = [
        0, 1, 2,
    ];
    let index_buffer = OpenGLIndexBuffer::new(&indices);

    while !window.should_close() {
        process_window_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        shader.bind();
        vertex_array.bind();
        index_buffer.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, indices.len() as GLsizei, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_window_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
            }

            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }

            _ => {}
        }
    }
}
