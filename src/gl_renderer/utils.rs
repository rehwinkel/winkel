pub mod shader {
    use std::collections::HashMap;
    pub struct Shader {
        id: u32,
    }
    impl Shader {
        fn new(source: &str, kind: u32) -> Self {
            let bytes = [source.as_bytes(), &[0]].concat();
            let src = std::ffi::CStr::from_bytes_with_nul(&bytes).unwrap();
            unsafe {
                let shader = gl::CreateShader(kind);
                gl::ShaderSource(shader, 1, &src.as_ptr(), std::ptr::null());
                gl::CompileShader(shader);
                Shader { id: shader }
            }
        }
        pub fn new_vertex(source: &str) -> Self {
            Shader::new(source, gl::VERTEX_SHADER)
        }
        pub fn new_fragment(source: &str) -> Self {
            Shader::new(source, gl::FRAGMENT_SHADER)
        }
    }
    impl std::ops::Drop for Shader {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }
    pub struct Program<'a> {
        id: u32,
        vertex_shader: Shader,
        fragment_shader: Shader,
        uniforms: HashMap<&'a str, i32>,
    }
    pub trait UniformLoadable {
        fn load(&self, id: i32);
    }
    impl UniformLoadable for f32 {
        fn load(&self, id: i32) {
            unsafe {
                gl::Uniform1fv(id, 1, std::mem::transmute(self));
            }
        }
    }
    impl UniformLoadable for [f32; 4] {
        fn load(&self, id: i32) {
            unsafe {
                gl::Uniform4fv(id, 1, std::mem::transmute(self));
            }
        }
    }
    impl UniformLoadable for [f32; 16] {
        fn load(&self, id: i32) {
            unsafe {
                gl::UniformMatrix4fv(id, 1, gl::FALSE, std::mem::transmute(self));
            }
        }
    }
    impl<'a> Program<'a> {
        pub fn new<I: IntoIterator<Item = &'a str>>(
            vertex_shader: Shader,
            fragment_shader: Shader,
            uniforms: I,
        ) -> Self {
            unsafe {
                let program = gl::CreateProgram();
                gl::AttachShader(program, vertex_shader.id);
                gl::AttachShader(program, fragment_shader.id);
                gl::LinkProgram(program);
                gl::ValidateProgram(program);
                let uniforms: HashMap<&'a str, i32> = uniforms
                    .into_iter()
                    .map(|uniform| {
                        let bytes = [uniform.as_bytes(), &[0]].concat();
                        let name = std::ffi::CStr::from_bytes_with_nul(&bytes).unwrap();
                        let uniform_id = gl::GetUniformLocation(program, name.as_ptr());
                        (uniform, uniform_id)
                    })
                    .collect();
                Program {
                    id: program,
                    vertex_shader,
                    fragment_shader,
                    uniforms,
                }
            }
        }
        pub fn start(&self) {
            unsafe {
                gl::UseProgram(self.id);
            }
        }
        pub fn stop(&self) {
            unsafe {
                gl::UseProgram(0);
            }
        }
        pub fn load<T: UniformLoadable>(&self, name: &'a str, value: T) {
            value.load(*self.uniforms.get(name).unwrap());
        }
    }
    impl<'a> std::ops::Drop for Program<'a> {
        fn drop(&mut self) {
            unsafe {
                gl::DetachShader(self.id, self.vertex_shader.id);
                gl::DetachShader(self.id, self.fragment_shader.id);
                gl::DeleteProgram(self.id);
            }
        }
    }
}

pub struct VertexArray {
    id: u32,
    count: usize,
    vertex_buffer: u32,
}

impl VertexArray {
    pub fn new(vertex_data: &[f32]) -> Self {
        unsafe {
            let mut vaoid: u32 = 0;
            gl::GenVertexArrays(1, &mut vaoid);
            gl::BindVertexArray(vaoid);
            let mut vboid: u32 = 0;
            gl::GenBuffers(1, &mut vboid);
            gl::BindBuffer(gl::ARRAY_BUFFER, vboid);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * vertex_data.len()) as isize,
                std::mem::transmute(vertex_data.as_ptr()),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            VertexArray {
                id: vaoid,
                count: vertex_data.len() / 3,
                vertex_buffer: vboid,
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::EnableVertexAttribArray(0);
            gl::DrawArrays(gl::TRIANGLES, 0, self.count as i32);
            gl::DisableVertexAttribArray(0);
            gl::BindVertexArray(0);
        }
    }
}

impl std::ops::Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vertex_buffer);
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new(width: i32, height: i32, data: &[u8]) -> Self {
        assert_eq!(data.len() as i32, width * height);
        unsafe {
            let mut texture: u32 = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                width,
                height,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void,
            );
            Texture { id: texture }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl std::ops::Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
