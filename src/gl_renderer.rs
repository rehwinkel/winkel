use super::Renderer;
use super::Style;

pub struct GlRenderer {
    quad_vao: u32,
    quad_vbo: u32,
    vertex_count: i32,
    shader_program: u32,
    vertex_shader: u32,
    fragment_shader: u32,
    transform_uniform: i32,
    color_uniform: i32,
    x_uniform: i32,
    y_uniform: i32,
    width_uniform: i32,
    height_uniform: i32,
    border_radius_uniform: i32,
    win_height_uniform: i32,
}

impl std::ops::Drop for GlRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.quad_vbo);
            gl::DeleteVertexArrays(1, &self.quad_vao);
            gl::DetachShader(self.shader_program, self.vertex_shader);
            gl::DetachShader(self.shader_program, self.fragment_shader);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
            gl::DeleteProgram(self.shader_program);
        }
    }
}

macro_rules! c_str {
    ($literal:expr) => {
        std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

impl GlRenderer {
    unsafe fn shader_from_src(rust_src: &[u8], kind: u32) -> u32 {
        let src: &std::ffi::CStr = std::ffi::CStr::from_bytes_with_nul(rust_src).unwrap();
        let shader = gl::CreateShader(kind);
        gl::ShaderSource(shader, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        shader
    }

    pub fn new() -> Self {
        let vertex_data: [f32; 18] = [
            -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0,
            1.0, 0.0,
        ];
        let vert_shader_src = b"#version 330 core

        in vec3 position;
        uniform mat4 transform;
        
        void main()
        {
            gl_Position = transform * vec4(position, 1.0);
        }\0";
        let frag_shader_src = b"#version 330 core

        out vec4 out_color;
        uniform vec4 color;

        uniform float border_radius;
        uniform float x;
        uniform float y;
        uniform float width;
        uniform float height;
        uniform float win_height;
        
        void main()
        {
            vec2 corner0 = vec2(x, win_height - y - height);
            vec2 corner1 = vec2(x, win_height - y);
            vec2 corner2 = vec2(x + width, win_height - y - height);
            vec2 corner3 = vec2(x + width, win_height - y);
            out_color = color;//vec4(0,0,0,0.1);
            float border = min(min(width, height) / 2, border_radius);
            
            if (distance(gl_FragCoord.xy, corner0 + vec2(border)) < border) {
                return;
            }
            if (distance(gl_FragCoord.xy, corner1 + vec2(border, -border)) < border) {
                return;
            }
            if (distance(gl_FragCoord.xy, corner2 + vec2(-border, border)) < border) {
                return;
            }
            if (distance(gl_FragCoord.xy, corner3 - vec2(border)) < border) {
                return;
            }
            if (gl_FragCoord.x < width - border + x && gl_FragCoord.x > x + border && gl_FragCoord.y > win_height - y - height && gl_FragCoord.y < win_height - y) {
                return;
            }
            if (gl_FragCoord.x > x && gl_FragCoord.x < x + width && gl_FragCoord.y < win_height - y - border && gl_FragCoord.y > win_height - y - height + border) {
                return;
            }
            discard;
        }\0";
        let mut vaoid: u32 = 0;
        let mut vboid: u32 = 0;
        let program: u32;
        let vertex_shader: u32;
        let fragment_shader: u32;
        let transform_uniform: i32;
        let color_uniform: i32;
        let x_uniform: i32;
        let y_uniform: i32;
        let width_uniform: i32;
        let height_uniform: i32;
        let win_height_uniform: i32;
        let border_radius_uniform: i32;
        unsafe {
            gl::GenVertexArrays(1, &mut vaoid);
            gl::BindVertexArray(vaoid);
            gl::GenBuffers(1, &mut vboid);
            gl::BindBuffer(gl::ARRAY_BUFFER, vboid);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertex_data) as isize,
                std::mem::transmute(&vertex_data),
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            vertex_shader = GlRenderer::shader_from_src(vert_shader_src, gl::VERTEX_SHADER);
            fragment_shader = GlRenderer::shader_from_src(frag_shader_src, gl::FRAGMENT_SHADER);
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::ValidateProgram(program);
            transform_uniform = gl::GetUniformLocation(program, c_str!("transform").as_ptr());
            color_uniform = gl::GetUniformLocation(program, c_str!("color").as_ptr());
            x_uniform = gl::GetUniformLocation(program, c_str!("x").as_ptr());
            y_uniform = gl::GetUniformLocation(program, c_str!("y").as_ptr());
            width_uniform = gl::GetUniformLocation(program, c_str!("width").as_ptr());
            height_uniform = gl::GetUniformLocation(program, c_str!("height").as_ptr());
            win_height_uniform = gl::GetUniformLocation(program, c_str!("win_height").as_ptr());
            border_radius_uniform =
                gl::GetUniformLocation(program, c_str!("border_radius").as_ptr());
        }
        GlRenderer {
            quad_vao: vaoid,
            quad_vbo: vboid,
            vertex_count: 6,
            shader_program: program,
            vertex_shader,
            fragment_shader,
            transform_uniform,
            color_uniform,
            x_uniform,
            y_uniform,
            width_uniform,
            height_uniform,
            border_radius_uniform,
            win_height_uniform,
        }
    }

    fn get_tranform_matrix(
        x_scale: f64,
        y_scale: f64,
        x_off: f64,
        y_off: f64,
        z_off: f64,
    ) -> [f32; 16] {
        [
            x_scale as f32,
            0.0,
            0.0,
            0.0,
            0.0,
            y_scale as f32,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            1.0,
            x_off as f32,
            y_off as f32,
            z_off as f32,
            1.0,
        ]
    }
}

impl Renderer for GlRenderer {
    fn render_quad(
        &self,
        x: f64,
        y: f64,
        _z: usize,
        width: f64,
        height: f64,
        style: &Style,
        window_width: f64,
        window_height: f64,
    ) {
        let width32 = width as f32;
        let height32 = height as f32;
        let x32 = x as f32;
        let y32 = y as f32;
        let window_height32 = window_height as f32;
        let border32 = style.border_radius as f32;
        let mat = GlRenderer::get_tranform_matrix(
            width / window_width,
            height / window_height,
            (x + width / 2.0 - window_width / 2.0) / window_width * 2.0,
            -(y + height / 2.0 - window_height / 2.0) / window_height * 2.0,
            0.0,
        );
        if let Some(color) = style.color {
            unsafe {
                gl::Enable(gl::BLEND);
                gl::Enable(gl::MULTISAMPLE);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::UseProgram(self.shader_program);
                gl::UniformMatrix4fv(
                    self.transform_uniform,
                    1,
                    gl::FALSE,
                    std::mem::transmute(&mat),
                );
                gl::Uniform4fv(self.color_uniform, 1, std::mem::transmute(&color));
                gl::Uniform1fv(self.x_uniform, 1, std::mem::transmute(&x32));
                gl::Uniform1fv(self.y_uniform, 1, std::mem::transmute(&y32));
                gl::Uniform1fv(self.width_uniform, 1, std::mem::transmute(&width32));
                gl::Uniform1fv(self.height_uniform, 1, std::mem::transmute(&height32));
                gl::Uniform1fv(
                    self.win_height_uniform,
                    1,
                    std::mem::transmute(&window_height32),
                );
                gl::Uniform1fv(
                    self.border_radius_uniform,
                    1,
                    std::mem::transmute(&border32),
                );
                gl::BindVertexArray(self.quad_vao);
                gl::EnableVertexAttribArray(0);
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
                gl::DisableVertexAttribArray(0);
                gl::BindVertexArray(0);
                gl::UseProgram(0);
            }
        }
    }
}
