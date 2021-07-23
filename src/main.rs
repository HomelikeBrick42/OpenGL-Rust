#![allow(dead_code)]

mod vector3;
use crate::vector3::{ Vector3 };

mod vertex;
use crate::vertex::{ Vertex };

mod opengl_shader;
use crate::opengl_shader::{ OpenGLShader };

extern crate glfw;
use glfw::{ Context, Key, Action };

extern crate gl;
use gl::types::*;

use std::os::raw::{ c_void };
use std::sync::mpsc::{ Receiver };

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize GLFW!");

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 4));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

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
    let vertices = [
        Vertex::new(Vector3::new( 0.0,  0.5, 0.0)),
        Vertex::new(Vector3::new( 0.5, -0.5, 0.0)),
        Vertex::new(Vector3::new(-0.5, -0.5, 0.0)),
    ];

    let indices: [u32; 3] = [
        0, 1, 2,
    ];

    let vertex_array = unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        vao
    };

    unsafe fn as_void_ptr<T>(v: *const T) -> *const c_void {
        v as *const c_void
    }

    let vertex_buffer = unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of_val(&vertices[0])) as GLsizeiptr,
            as_void_ptr(&vertices[0]),
            gl::STATIC_DRAW,
        );
        vbo
    };

    let index_buffer = unsafe {
        let mut ibo = 0;
        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of_val(&indices[0])) as GLsizeiptr,
            as_void_ptr(&indices[0]),
            gl::STATIC_DRAW,
        );
        ibo
    };

    #[allow(deref_nullptr)]
    unsafe {
        // Taken from https://stackoverflow.com/questions/40310483/how-to-get-pointer-offset-in-bytes/40310851#40310851
        macro_rules! offset_of {
            ($ty:ty, $field:ident) => {
                //  Undefined Behavior: dereferences a null pointer.
                //  Undefined Behavior: accesses field outside of valid memory area.
                &(*(0 as *const $ty)).$field as *const _ as usize
            }
        }

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, offset_of!(Vertex, position) as *const c_void);
    }

    while !window.should_close() {
        process_window_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            shader.bind();
            gl::BindVertexArray(vertex_array);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);
            gl::DrawElements(gl::TRIANGLES, indices.len() as GLsizei, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteBuffers(1, &index_buffer);
        gl::DeleteBuffers(1, &vertex_buffer);
        gl::DeleteVertexArrays(1, &vertex_array);
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
