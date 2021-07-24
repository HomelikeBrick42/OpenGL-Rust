#![allow(dead_code)]

mod vector3;
mod vector2;
mod vertex;
mod opengl_shader;
mod opengl_vertex_buffer;
mod opengl_vertex_array;
mod opengl_index_buffer;
mod opengl_texture;

use crate::vector2::{ Vector2 };
use crate::vector3::{ Vector3 };
use crate::vertex::{ Vertex };
use crate::opengl_shader::{ OpenGLShader };
use crate::opengl_vertex_buffer::{ OpenGLVertexBuffer };
use crate::opengl_vertex_array::{ OpenGLVertexArray, BufferElement };
use crate::opengl_index_buffer::{ OpenGLIndexBuffer };
use crate::opengl_texture::{ OpenGLTexture };

extern crate glfw;
extern crate gl;
extern crate num;
extern crate image;

use glfw::{ Context, Key, Action };
use gl::types::*;
use image::GenericImageView;

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

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let shader = OpenGLShader::new(include_str!("../texture.vert.glsl"), include_str!("../texture.frag.glsl"));

    let mut vertex_array = OpenGLVertexArray::new();

    let vertices = [
        Vertex::new(Vector3::new(-0.5,  0.5, 0.0), Vector2::new(0.0, 1.0)),
        Vertex::new(Vector3::new( 0.5,  0.5, 0.0), Vector2::new(1.0, 1.0)),
        Vertex::new(Vector3::new( 0.5, -0.5, 0.0), Vector2::new(1.0, 0.0)),
        Vertex::new(Vector3::new(-0.5, -0.5, 0.0), Vector2::new(0.0, 0.0)),
    ];
    let _vertex_buffer = vertex_array.add_vertex_buffer(OpenGLVertexBuffer::new(&vertices), &[BufferElement::Float3, BufferElement::Float2]);

    let indices = [
        0, 1, 2,
        0, 2, 3,
    ];
    let index_buffer = OpenGLIndexBuffer::new(&indices);

    let cat_image = image::load_from_memory(include_bytes!("../cat.jpg"))
        .expect("Failed to read image!");
    let texture = OpenGLTexture::new(&cat_image.flipv().to_rgba8().into_raw(), cat_image.width(), cat_image.height());

    while !window.should_close() {
        process_window_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        shader.bind();

        shader.set_integer("u_Texture", 0);
        texture.bind(0);

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
