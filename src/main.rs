#![allow(non_upper_case_globals, dead_code)]

extern crate glfw;
use self::glfw::{ Context, Key, Action };

extern crate gl;
use self::gl::types::*;

use std::os::raw::c_void;

use std::ffi::CString;

use std::sync::mpsc::Receiver;

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

    let shader = unsafe {
        const vertex_shader_source: &str = r#"
#version 440 core

layout(location = 0) in vec4 a_Position;

void main() {
    gl_Position = a_Position;
}
"#;

        const fragment_shader_source: &str = r#"
#version 440 core

layout(location = 0) out vec4 o_Color;

void main() {
    o_Color = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;
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
                println!("{} Shader Compilation Failed:\n{}", if shader_type == gl::VERTEX_SHADER { "Vertex" } else { "Fragment" }, std::str::from_utf8(&info_log).unwrap());
            }

            return shader;
        }

        let vertex_shader = create_shader(vertex_shader_source, gl::VERTEX_SHADER);
        let fragment_shader = create_shader(fragment_shader_source, gl::FRAGMENT_SHADER);

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
            println!("Shader Linking Failed:\n{}", std::str::from_utf8(&info_log).unwrap());
        }

        gl::DetachShader(shader_program, vertex_shader);
        gl::DeleteShader(vertex_shader);

        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    };

    let vertices: [f32; 9] = [
         0.0,  0.5, 0.0,
         0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
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

    let vertex_buffer = unsafe {
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
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
            (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            &indices[0] as *const u32 as *const c_void,
            gl::STATIC_DRAW,
        );
        ibo
    };

    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
    }

    while !window.should_close() {
        process_window_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            gl::UseProgram(shader);
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
        gl::DeleteProgram(shader);
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
