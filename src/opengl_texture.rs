use gl::types::*;

pub struct OpenGLTexture {
    id: GLuint,
}

impl OpenGLTexture {
    pub fn new(rgba_pixels: &[u8], width: u32, height: u32) -> OpenGLTexture {
        assert_eq!(rgba_pixels.len(), width as usize * height as usize * 4);
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &rgba_pixels[0] as *const u8 as *const GLvoid
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);

            return OpenGLTexture {
                id: texture,
            };
        }
    }

    pub fn bind(&self, index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn un_bind(&self, index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for OpenGLTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
