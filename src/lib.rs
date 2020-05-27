use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod color;
pub mod widgets;

use color::Color;
use widgets::Widget;

pub trait Renderer {
    fn render_quad(
        &mut self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        style: &Style,
        window_width: f64,
        window_height: f64,
    );

    fn render_text<'a>(
        &mut self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        text: &'a str,
        style: &TextStyle,
        window_width: f64,
        window_height: f64,
    );

    fn render(
        &mut self,
        computed: &HashMap<usize, ComputedWidget>,
        window_width: f64,
        window_height: f64,
    ) {
        let mut widgets: Vec<&ComputedWidget> =
            computed.values().filter(|w| w.render.is_some()).collect();
        widgets.sort_by_key(|w| w.z);
        for widget in widgets {
            match widget.render.as_ref().unwrap() {
                RenderObject::Rectangle { style } => {
                    self.render_quad(
                        widget.x,
                        widget.y,
                        widget.z,
                        widget.width,
                        widget.height,
                        &style,
                        window_width,
                        window_height,
                    );
                }
                RenderObject::Text { text, style } => {
                    self.render_text(
                        widget.x,
                        widget.y,
                        widget.z,
                        widget.width,
                        widget.height,
                        text,
                        style,
                        window_width,
                        window_height,
                    );
                }
            }
        }
    }
}

mod gl_renderer;
pub use gl_renderer::GlRenderer;

pub struct State<T> {
    reference: Rc<RefCell<T>>,
    bound: bool,
}

impl<T> State<T> {
    pub fn new() -> Self {
        let inner_rc: Rc<RefCell<T>> = unsafe {
            let ptr = std::alloc::alloc(std::alloc::Layout::new::<RefCell<T>>()) as *mut RefCell<T>;
            let b = Box::from_raw(ptr);
            Rc::from(b)
        };
        State {
            reference: inner_rc,
            bound: false,
        }
    }

    pub fn bind(&mut self, reference: Rc<RefCell<T>>) {
        self.reference = reference;
        self.bound = true;
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, T> {
        self.reference.borrow()
    }
    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, T> {
        self.reference.borrow_mut()
    }
}

pub enum Event {
    MouseDown {
        x: f64,
        y: f64,
        button: u8,
    },
    MouseUp {
        x: f64,
        y: f64,
        button: u8,
    },
    MouseMove {
        prev_x: f64,
        prev_y: f64,
        x: f64,
        y: f64,
    },
}

#[derive(Debug)]
pub struct Style {
    color: Option<Color>,
    border_radius: f64,
}

#[derive(Debug)]
pub struct TextStyle<'a> {
    font: &'a str,
    color: Color,
    size: u32,
}

#[derive(Debug)]
pub enum RenderObject<'a> {
    Rectangle { style: Style },
    Text { text: &'a str, style: TextStyle<'a> },
}

#[derive(Debug)]
pub struct ComputedWidget<'a> {
    x: f64,
    y: f64,
    z: usize,
    width: f64,
    height: f64,
    render: Option<RenderObject<'a>>,
}

impl<'a> ComputedWidget<'a> {
    fn in_hitbox(&self, x: f64, y: f64, border_radius: f64) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.width && y < self.y + self.height
        // TODO
    }
}

pub fn compute<'a>(
    tree: &Rc<RefCell<dyn Widget<'a> + 'a>>,
    width: f64,
    height: f64,
) -> HashMap<usize, ComputedWidget<'a>> {
    let mut elem_map = HashMap::new();
    tree.borrow()
        .compute(0.0, 0.0, 0, width, height, &mut elem_map);
    elem_map
}
