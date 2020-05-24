use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::rc::Rc;

pub trait Renderer {
    fn render_quad(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        style: &Style,
        window_width: f64,
        window_height: f64,
    );

    fn render(
        &self,
        computed: &HashMap<usize, ComputedWidget>,
        window_width: f64,
        window_height: f64,
    ) {
        let mut widgets: Vec<&ComputedWidget> = computed.values().filter(|w| w.display).collect();
        widgets.sort_by_key(|w| w.z);
        for widget in widgets {
            self.render_quad(
                widget.x,
                widget.y,
                widget.z,
                widget.width,
                widget.height,
                &widget.style,
                window_width,
                window_height,
            );
        }
    }
}

mod gl_renderer;
pub use gl_renderer::GlRenderer;

pub enum Event {
    MouseDown { x: f64, y: f64, button: u8 },
    MouseUp { x: f64, y: f64, button: u8 },
}

pub type Color = [f32; 4];

#[derive(Debug)]
pub struct Style {
    color: Option<Color>,
    border_radius: f64,
}

#[derive(Debug)]
pub struct ComputedWidget {
    x: f64,
    y: f64,
    z: usize,
    width: f64,
    height: f64,
    display: bool,
    style: Style,
}

pub trait Widget {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget>,
    );
    fn dispatch(&self, event: Event, map: &HashMap<usize, ComputedWidget>) -> Option<Event>;
    fn get_id(&self) -> usize;
}

pub struct Rectangle {
    pub color: Color,
    pub border_radius: f64,
    pub id: usize,
}

pub struct RectangleBuilder {
    pub color: Color,
    pub border_radius: f64,
}

impl Rectangle {
    pub fn new(color: Color) -> RectangleBuilder {
        RectangleBuilder {
            color,
            border_radius: 0.0,
        }
    }
}

impl RectangleBuilder {
    pub fn border(mut self, border_radius: f64) -> Self {
        self.border_radius = border_radius;
        self
    }

    pub fn build(self) -> Rc<dyn Widget> {
        Rc::new(Rectangle {
            color: self.color,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            border_radius: self.border_radius,
        })
    }
}

pub struct Empty {
    pub id: usize,
}

impl Empty {
    pub fn new() -> Rc<dyn Widget> {
        Rc::new(Empty {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        })
    }
}

pub struct Button {
    pub background: Rc<dyn Widget>,
    pub click_callback: Option<Rc<dyn Fn(u8) -> ()>>,
    pub release_callback: Option<Rc<dyn Fn(u8) -> ()>>,
    pub id: usize,
}

pub struct ButtonBuilder {
    background: Rc<dyn Widget>,
    border_radius: f64,
    click_callback: Option<Rc<dyn Fn(u8) -> ()>>,
    release_callback: Option<Rc<dyn Fn(u8) -> ()>>,
}

impl Button {
    pub fn new(background: Rc<dyn Widget>) -> ButtonBuilder {
        ButtonBuilder {
            background,
            border_radius: 0.0,
            click_callback: None,
            release_callback: None,
        }
    }
}

impl ButtonBuilder {
    pub fn on_click(mut self, on_click: impl Fn(u8) -> () + 'static) -> Self {
        self.click_callback = Some(Rc::new(on_click));
        self
    }

    pub fn on_release(mut self, on_release: Rc<dyn Fn(u8) -> ()>) -> Self {
        self.release_callback = Some(on_release);
        self
    }

    pub fn border(mut self, border_radius: f64) -> Self {
        self.border_radius = border_radius;
        self
    }

    pub fn build(self) -> Rc<dyn Widget> {
        Rc::new(Button {
            background: self.background,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            click_callback: self.click_callback,
            release_callback: self.release_callback,
        })
    }
}

pub struct Padding {
    pub padding: (f64, f64, f64, f64),
    pub child: Rc<dyn Widget>,
    pub id: usize,
}

pub struct PaddingBuilder {
    pub padding: (f64, f64, f64, f64),
    pub child: Rc<dyn Widget>,
}

impl Padding {
    pub fn new(child: Rc<dyn Widget>) -> PaddingBuilder {
        PaddingBuilder {
            child,
            padding: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl PaddingBuilder {
    pub fn all(mut self, pad: f64) -> Self {
        self.padding = (pad, pad, pad, pad);
        self
    }

    pub fn symmetrical(mut self, horizontal: f64, vertical: f64) -> Self {
        self.padding = (horizontal, vertical, horizontal, vertical);
        self
    }

    pub fn each(mut self, left: f64, top: f64, right: f64, bottom: f64) -> Self {
        self.padding = (left, top, right, bottom);
        self
    }

    pub fn build(self) -> Rc<dyn Widget> {
        Rc::new(Padding {
            child: self.child,
            padding: self.padding,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        })
    }
}

pub struct Row {
    pub children: Vec<Rc<dyn Widget>>,
    pub flex: Vec<usize>,
    pub id: usize,
}

pub struct RowBuilder {
    pub children: Vec<Rc<dyn Widget>>,
    pub flex: Vec<usize>,
}

impl Row {
    pub fn new() -> RowBuilder {
        RowBuilder {
            children: Vec::new(),
            flex: Vec::new(),
        }
    }
}

impl RowBuilder {
    pub fn add(mut self, child: Rc<dyn Widget>) -> Self {
        self.children.push(child);
        self.flex.push(1);
        self
    }

    pub fn add_flex(mut self, child: Rc<dyn Widget>, flex: usize) -> Self {
        self.children.push(child);
        self.flex.push(flex);
        self
    }

    pub fn build(self) -> Rc<dyn Widget> {
        Rc::new(Row {
            children: self.children,
            flex: self.flex,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        })
    }
}

pub struct Column {
    pub children: Vec<Rc<dyn Widget>>,
    pub flex: Vec<usize>,
    pub id: usize,
}

pub struct ColumnBuilder {
    pub children: Vec<Rc<dyn Widget>>,
    pub flex: Vec<usize>,
}

impl Column {
    pub fn new() -> ColumnBuilder {
        ColumnBuilder {
            children: Vec::new(),
            flex: Vec::new(),
        }
    }
}

impl ColumnBuilder {
    pub fn add(mut self, child: Rc<dyn Widget>) -> Self {
        self.children.push(child);
        self.flex.push(1);
        self
    }

    pub fn add_flex(mut self, child: Rc<dyn Widget>, flex: usize) -> Self {
        self.children.push(child);
        self.flex.push(flex);
        self
    }

    pub fn build(self) -> Rc<dyn Widget> {
        Rc::new(Column {
            children: self.children,
            flex: self.flex,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        })
    }
}

pub struct Stack {
    pub children: Vec<Rc<dyn Widget>>,
    pub id: usize,
}

pub mod widgets {
    pub use super::Widget;
    pub use super::{Button, Column, Empty, Padding, Rectangle, Row, Stack};
}

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn compute(tree: &Rc<dyn Widget>, width: f64, height: f64) -> HashMap<usize, ComputedWidget> {
    let mut elem_map = HashMap::new();
    tree.compute(0.0, 0.0, 0, width, height, &mut elem_map);
    elem_map
}

impl Widget for Button {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget>,
    ) {
        self.background.compute(x, y, z, width, height, map);
        map.insert(
            self.get_id(),
            ComputedWidget {
                x,
                y,
                z,
                width,
                height,
                display: false,
                style: Style {
                    color: None,
                    border_radius: 0.0,
                },
            },
        );
    }

    fn dispatch(&self, event: Event, map: &HashMap<usize, ComputedWidget>) -> Option<Event> {
        let computed: &ComputedWidget = map.get(&self.get_id()).unwrap();
        match event {
            Event::MouseDown { x, y, button } => {
                if x > computed.x
                    && y > computed.y
                    && x < computed.x + computed.width
                    && y < computed.y + computed.height
                {
                    if let Some(click) = &self.click_callback {
                        click(button);
                    }
                    None
                } else {
                    Some(event)
                }
            }
            Event::MouseUp { x, y, button } => {
                if x > computed.x
                    && y > computed.y
                    && x < computed.x + computed.width
                    && y < computed.y + computed.height
                {
                    if let Some(release) = &self.release_callback {
                        release(button);
                    }
                    None
                } else {
                    Some(event)
                }
            }
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl Widget for Rectangle {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget>,
    ) {
        map.insert(
            self.get_id(),
            ComputedWidget {
                x,
                y,
                z,
                width,
                height,
                display: true,
                style: Style {
                    color: Some(self.color),
                    border_radius: self.border_radius,
                },
            },
        );
    }

    fn dispatch(&self, event: Event, _map: &HashMap<usize, ComputedWidget>) -> Option<Event> {
        Some(event)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl Widget for Empty {
    fn compute(
        &self,
        _x: f64,
        _y: f64,
        _z: usize,
        _width: f64,
        _height: f64,
        _map: &mut HashMap<usize, ComputedWidget>,
    ) {
    }

    fn dispatch(&self, event: Event, _map: &HashMap<usize, ComputedWidget>) -> Option<Event> {
        Some(event)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl Widget for Padding {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget>,
    ) {
        let mut w = width - self.padding.2 - self.padding.0;
        let mut h = height - self.padding.3 - self.padding.1;
        if w < 0.0 {
            w = 0.0;
        }
        if h < 0.0 {
            h = 0.0;
        }
        self.child
            .compute(x + self.padding.0, y + self.padding.1, z, w, h, map)
    }

    fn dispatch(&self, event: Event, map: &HashMap<usize, ComputedWidget>) -> Option<Event> {
        self.child.dispatch(event, map)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl Widget for Row {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget>,
    ) {
        let total_len = self.flex.iter().sum::<usize>();
        let each_child_width = width / total_len as f64;
        let mut prev_flex = 0;
        self.children.iter().enumerate().for_each(|(i, child)| {
            let flex = self.flex[i];
            let offset = prev_flex as f64 * each_child_width;
            prev_flex += flex;
            child.compute(
                x + offset,
                y,
                z,
                each_child_width * flex as f64,
                height,
                map,
            );
        });
    }

    fn dispatch(&self, event: Event, map: &HashMap<usize, ComputedWidget>) -> Option<Event> {
        let mut e = Some(event);
        for child in &self.children {
            if let Some(ev) = e {
                e = child.dispatch(ev, map);
            } else {
                break;
            }
        }
        e
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl Widget for Stack {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget>,
    ) {
        self.children
            .iter()
            .enumerate()
            .for_each(|(i, c)| c.compute(x, y, z + i, width, height, map));
    }

    fn dispatch(&self, event: Event, map: &HashMap<usize, ComputedWidget>) -> Option<Event> {
        let mut e = Some(event);
        for child in &self.children {
            if let Some(ev) = e {
                e = child.dispatch(ev, map);
            } else {
                break;
            }
        }
        e
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl Widget for Column {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget>,
    ) {
        let total_len = self.flex.iter().sum::<usize>();
        let each_child_height = height / total_len as f64;
        let mut prev_flex = 0;
        self.children.iter().enumerate().for_each(|(i, child)| {
            let flex = self.flex[i];
            let offset = prev_flex as f64 * each_child_height;
            prev_flex += flex;
            child.compute(
                x,
                y + offset,
                z,
                width,
                each_child_height * flex as f64,
                map,
            );
        });
    }

    fn dispatch(&self, event: Event, map: &HashMap<usize, ComputedWidget>) -> Option<Event> {
        let mut e = Some(event);
        for child in &self.children {
            if let Some(ev) = e {
                e = child.dispatch(ev, map);
            } else {
                break;
            }
        }
        e
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

pub mod colors {
    pub const WHITE: [f32; 4] = [1.0; 4];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const MAGENTA: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
    pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    pub const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
}
