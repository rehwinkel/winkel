use super::utils::Texture;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Font {
    face: freetype::Face,
    size: u32,
    characters: HashMap<char, Character>,
}

#[derive(Debug)]
pub struct Character {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    advance: i32,
    texture: Texture,
}

impl Font {
    pub fn new(file: &str, size: u32) -> Self {
        let lib = freetype::Library::init().unwrap();
        let face = lib.new_face(file, 0).unwrap();
        face.set_pixel_sizes(0, size).unwrap();

        Font {
            face,
            size,
            characters: HashMap::new(),
        }
    }

    pub fn get_char(&mut self, ch: char) -> &Character {
        if !self.characters.contains_key(&ch) {
            self.face
                .load_char(ch as usize, freetype::face::LoadFlag::RENDER)
                .unwrap();
            let glyph = self.face.glyph();
            let bmp = glyph.bitmap();
            let renderchar = Character {
                left: glyph.bitmap_left(),
                top: glyph.bitmap_top(),
                width: bmp.width(),
                height: bmp.rows(),
                advance: glyph.advance().x as i32,
                texture: Texture::new(bmp.width(), bmp.rows(), bmp.buffer()),
            };
            self.characters.insert(ch, renderchar);
        }
        self.characters.get(&ch).unwrap()
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Character {
    pub fn bind(&self) {
        self.texture.bind();
    }

    pub fn unbind(&self) {
        self.texture.unbind();
    }

    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn advance(&self) -> i32 {
        self.advance
    }

    pub fn top(&self) -> i32 {
        self.top
    }
    pub fn left(&self) -> i32 {
        self.left
    }
}
