use super::Renderer;
use super::{Style, TextStyle};
use std::collections::HashMap;

mod font;
mod utils;

use font::Font;
use utils::{
    shader::{Program, Shader},
    VertexArray,
};

#[derive(Eq, PartialEq, Hash)]
struct FontDescription {
    size: u32,
    name: String,
}

pub struct GlRenderer<'a> {
    quad: VertexArray,
    fonts: HashMap<FontDescription, Font>,
    rect_shader: Program<'a>,
    text_shader: Program<'a>,
}

impl<'a, 'fonts> GlRenderer<'a> {
    pub fn new() -> Self {
        let vertex_data: [f32; 18] = [
            -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0,
            1.0, 0.0,
        ];
        let vert_shader_src = "#version 330 core
        in vec3 position;
        uniform mat4 transform;
        out vec2 pass_pos;
        
        void main()
        {
            pass_pos = position.xy;
            gl_Position = transform * vec4(position, 1.0);
        }";
        let rect_frag_shader_src = "#version 330 core
        in vec2 pass_pos;
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
            out_color = color;
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
        }";
        let text_frag_shader_src = "#version 330 core
        out vec4 out_color;
        uniform vec4 color;
        uniform sampler2D tex;
        in vec2 pass_pos;
        
        void main()
        {
            out_color = mix(vec4(0.0, 0.0, 0.0, 0.0), color, texture2D(tex, pass_pos * 0.5 * vec2(1, -1) + 0.5).x);
        }";
        unsafe {
            gl::Enable(gl::BLEND);
            gl::Enable(gl::MULTISAMPLE);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        GlRenderer {
            quad: VertexArray::new(&vertex_data),
            fonts: HashMap::new(),
            rect_shader: Program::new(
                Shader::new_vertex(vert_shader_src),
                Shader::new_fragment(rect_frag_shader_src),
                vec![
                    "transform",
                    "color",
                    "x",
                    "y",
                    "width",
                    "height",
                    "win_height",
                    "border_radius",
                ],
            ),
            text_shader: Program::new(
                Shader::new_vertex(vert_shader_src),
                Shader::new_fragment(text_frag_shader_src),
                vec!["transform", "color"],
            ),
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

impl<'a> Renderer for GlRenderer<'a> {
    fn render_quad(
        &mut self,
        x: f64,
        y: f64,
        _z: usize,
        width: f64,
        height: f64,
        style: &Style,
        window_width: f64,
        window_height: f64,
    ) {
        let mat = GlRenderer::get_tranform_matrix(
            width / window_width,
            height / window_height,
            (x + width / 2.0 - window_width / 2.0) / window_width * 2.0,
            -(y + height / 2.0 - window_height / 2.0) / window_height * 2.0,
            0.0,
        );
        if let Some(color) = style.color {
            self.rect_shader.start();
            self.rect_shader.load("transform", mat);
            self.rect_shader.load("color", color);
            self.rect_shader.load("x", x as f32);
            self.rect_shader.load("y", y as f32);
            self.rect_shader.load("width", width as f32);
            self.rect_shader.load("height", height as f32);
            self.rect_shader.load("win_height", window_height as f32);
            self.rect_shader
                .load("border_radius", style.border_radius as f32);
            self.quad.draw();
            self.rect_shader.stop();
        }
    }

    fn render_text<'b>(
        &mut self,
        x: f64,
        y: f64,
        _z: usize,
        _width: f64,
        _height: f64,
        text: &'b str,
        style: &TextStyle<'b>,
        window_width: f64,
        window_height: f64,
    ) {
        let font = self
            .fonts
            .entry(FontDescription {
                name: String::from(style.font),
                size: style.size,
            })
            .or_insert_with(|| Font::new(style.font, style.size));
        let fontsize = font.size() as f64;
        let mut offset: f64 = 0.0;
        for ch in text.chars() {
            let renderchar = font.get_char(ch);
            let width = renderchar.width() as f64;
            let height = renderchar.height() as f64;
            let x = x + offset + renderchar.left() as f64;
            let y = y - renderchar.top() as f64 + fontsize;
            offset += renderchar.advance() as f64 / 64.0;
            let mat = GlRenderer::get_tranform_matrix(
                width / window_width,
                height / window_height,
                (x + width / 2.0 - window_width / 2.0) / window_width * 2.0,
                -(y + height / 2.0 - window_height / 2.0) / window_height * 2.0,
                0.0,
            );
            self.text_shader.start();
            renderchar.bind();
            self.text_shader.load("transform", mat);
            self.text_shader.load("color", style.color);
            self.quad.draw();
            renderchar.unbind();
            self.text_shader.stop();
        }
    }
}
