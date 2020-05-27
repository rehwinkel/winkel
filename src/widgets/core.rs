use super::super::{color::Color, ComputedWidget, Event, RenderObject, State, Style, TextStyle};
use super::Widget;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Text<'a> {
    text: &'a str,
    id: usize,
    size: u32,
    font: &'a str,
    color: Color,
}

pub struct TextBuilder<'a> {
    text: &'a str,
    size: u32,
    font: &'a str,
    color: Color,
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str, size: u32, font: &'a str) -> TextBuilder<'a> {
        TextBuilder {
            text,
            font,
            color: [0.0, 0.0, 0.0, 1.0],
            size: size,
        }
    }
}

impl<'a> TextBuilder<'a> {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn build(self) -> Rc<RefCell<Text<'a>>> {
        Rc::new(RefCell::new(Text {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            text: self.text,
            color: self.color,
            font: self.font,
            size: self.size,
        }))
    }

    pub fn build_stateful(self, state: &mut State<Text<'a>>) -> Rc<RefCell<Text<'a>>> {
        let result = Rc::new(RefCell::new(Text {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            text: self.text,
            color: self.color,
            size: self.size,
            font: self.font,
        }));
        state.bind(result.clone());
        result
    }
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

    pub fn build(self) -> Rc<RefCell<Rectangle>> {
        Rc::new(RefCell::new(Rectangle {
            color: self.color,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            border_radius: self.border_radius,
        }))
    }

    pub fn build_stateful(self, state: &mut State<Rectangle>) -> Rc<RefCell<Rectangle>> {
        let result = Rc::new(RefCell::new(Rectangle {
            color: self.color,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            border_radius: self.border_radius,
        }));
        state.bind(result.clone());
        result
    }
}

pub struct Empty {
    pub id: usize,
}

impl Empty {
    pub fn new() -> Rc<Empty> {
        Rc::new(Empty {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        })
    }
}

pub struct MouseGesture<'a> {
    pub background: Rc<RefCell<dyn Widget<'a> + 'a>>,
    pub click_callback: Option<Box<dyn Fn(u8) -> bool + 'a>>,
    pub release_callback: Option<Box<dyn Fn(u8) -> bool + 'a>>,
    pub enter_callback: Option<Box<dyn Fn() -> bool + 'a>>,
    pub leave_callback: Option<Box<dyn Fn() -> bool + 'a>>,
    border_radius: f64,
    pub id: usize,
}

pub struct MouseGestureBuilder<'a> {
    background: Rc<RefCell<dyn Widget<'a> + 'a>>,
    click_callback: Option<Box<dyn Fn(u8) -> bool + 'a>>,
    release_callback: Option<Box<dyn Fn(u8) -> bool + 'a>>,
    enter_callback: Option<Box<dyn Fn() -> bool + 'a>>,
    leave_callback: Option<Box<dyn Fn() -> bool + 'a>>,
    border_radius: f64,
}

impl<'a> MouseGesture<'a> {
    pub fn new(background: Rc<RefCell<dyn Widget<'a> + 'a>>) -> MouseGestureBuilder<'a> {
        MouseGestureBuilder {
            background,
            border_radius: 0.0,
            click_callback: None,
            release_callback: None,
            enter_callback: None,
            leave_callback: None,
        }
    }
}

impl<'a> MouseGestureBuilder<'a> {
    pub fn on_click<F: Fn(u8) -> bool + 'a>(mut self, on_click: F) -> Self {
        self.click_callback = Some(Box::new(on_click));
        self
    }

    pub fn on_release<F: Fn(u8) -> bool + 'a>(mut self, on_release: F) -> Self {
        self.release_callback = Some(Box::new(on_release));
        self
    }
    pub fn on_enter<F: Fn() -> bool + 'a>(mut self, on_enter: F) -> Self {
        self.enter_callback = Some(Box::new(on_enter));
        self
    }

    pub fn on_leave<F: Fn() -> bool + 'a>(mut self, on_leave: F) -> Self {
        self.leave_callback = Some(Box::new(on_leave));
        self
    }

    pub fn border(mut self, border_radius: f64) -> Self {
        self.border_radius = border_radius;
        self
    }

    pub fn build(self) -> Rc<RefCell<MouseGesture<'a>>> {
        Rc::new(RefCell::new(MouseGesture {
            background: self.background,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            border_radius: self.border_radius,
            click_callback: self.click_callback,
            release_callback: self.release_callback,
            enter_callback: self.enter_callback,
            leave_callback: self.leave_callback,
        }))
    }
}

pub struct Padding<'a> {
    pub padding: (f64, f64, f64, f64),
    pub child: Rc<RefCell<dyn Widget<'a> + 'a>>,
    pub id: usize,
}

pub struct PaddingBuilder<'a> {
    pub padding: (f64, f64, f64, f64),
    pub child: Rc<RefCell<dyn Widget<'a> + 'a>>,
}

impl<'a> Padding<'a> {
    pub fn new(child: Rc<RefCell<dyn Widget<'a> + 'a>>) -> PaddingBuilder<'a> {
        PaddingBuilder {
            child,
            padding: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl<'a> PaddingBuilder<'a> {
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

    pub fn build(self) -> Rc<RefCell<Padding<'a>>> {
        Rc::new(RefCell::new(Padding {
            child: self.child,
            padding: self.padding,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }))
    }
}

pub struct Row<'a> {
    pub children: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
    pub flex: Vec<usize>,
    pub id: usize,
}

pub struct RowBuilder<'a> {
    pub children: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
    pub flex: Vec<usize>,
}

impl<'a> Row<'a> {
    pub fn new() -> RowBuilder<'a> {
        RowBuilder {
            children: Vec::new(),
            flex: Vec::new(),
        }
    }
}

impl<'a> RowBuilder<'a> {
    pub fn add(mut self, child: Rc<RefCell<dyn Widget<'a> + 'a>>) -> Self {
        self.children.push(child);
        self.flex.push(1);
        self
    }

    pub fn add_flex(mut self, child: Rc<RefCell<dyn Widget<'a> + 'a>>, flex: usize) -> Self {
        self.children.push(child);
        self.flex.push(flex);
        self
    }

    pub fn build(self) -> Rc<RefCell<Row<'a>>> {
        Rc::new(RefCell::new(Row {
            children: self.children,
            flex: self.flex,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }))
    }
}

pub struct Column<'a> {
    pub children: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
    pub flex: Vec<usize>,
    pub id: usize,
}

pub struct ColumnBuilder<'a> {
    pub children: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
    pub flex: Vec<usize>,
}

impl<'a> Column<'a> {
    pub fn new() -> ColumnBuilder<'a> {
        ColumnBuilder {
            children: Vec::new(),
            flex: Vec::new(),
        }
    }
}

impl<'a> ColumnBuilder<'a> {
    pub fn add(mut self, child: Rc<RefCell<dyn Widget<'a> + 'a>>) -> Self {
        self.children.push(child);
        self.flex.push(1);
        self
    }

    pub fn add_flex(mut self, child: Rc<RefCell<dyn Widget<'a> + 'a>>, flex: usize) -> Self {
        self.children.push(child);
        self.flex.push(flex);
        self
    }

    pub fn build(self) -> Rc<RefCell<Column<'a>>> {
        Rc::new(RefCell::new(Column {
            children: self.children,
            flex: self.flex,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }))
    }
}

pub struct Stack<'a> {
    pub children: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
    pub id: usize,
}

pub struct StackBuilder<'a> {
    pub children: Vec<Rc<RefCell<dyn Widget<'a> + 'a>>>,
}

impl<'a> Stack<'a> {
    pub fn new() -> StackBuilder<'a> {
        StackBuilder {
            children: Vec::new(),
        }
    }
}

impl<'a> StackBuilder<'a> {
    pub fn add(mut self, child: Rc<RefCell<dyn Widget<'a> + 'a>>) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self) -> Rc<RefCell<Stack<'a>>> {
        Rc::new(RefCell::new(Stack {
            children: self.children,
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }))
    }
}

impl<'a> Widget<'a> for MouseGesture<'a> {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget<'a>>,
    ) {
        self.background
            .borrow()
            .compute(x, y, z, width, height, map);
        map.insert(
            self.get_id(),
            ComputedWidget {
                x,
                y,
                z,
                width,
                height,
                render: None,
            },
        );
    }

    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool) {
        let computed: &ComputedWidget = map.get(&self.get_id()).unwrap();
        match event {
            Event::MouseDown { x, y, button } => {
                if computed.in_hitbox(x, y, self.border_radius) {
                    let state_change = if let Some(click) = &self.click_callback {
                        click(button)
                    } else {
                        false
                    };
                    (None, prev_state_change | state_change)
                } else {
                    (Some(event), prev_state_change)
                }
            }
            Event::MouseUp { x, y, button } => {
                if computed.in_hitbox(x, y, self.border_radius) {
                    let state_change = if let Some(release) = &self.release_callback {
                        release(button)
                    } else {
                        false
                    };
                    (None, prev_state_change | state_change)
                } else {
                    (Some(event), prev_state_change)
                }
            }
            Event::MouseMove {
                prev_x,
                prev_y,
                x,
                y,
            } => {
                if computed.in_hitbox(x, y, self.border_radius)
                    && !computed.in_hitbox(prev_x, prev_y, self.border_radius)
                {
                    let state_change = if let Some(enter) = &self.enter_callback {
                        enter()
                    } else {
                        false
                    };
                    (None, prev_state_change | state_change)
                } else if !computed.in_hitbox(x, y, self.border_radius)
                    && computed.in_hitbox(prev_x, prev_y, self.border_radius)
                {
                    let state_change = if let Some(leave) = &self.leave_callback {
                        leave()
                    } else {
                        false
                    };
                    (None, prev_state_change | state_change)
                } else {
                    (Some(event), prev_state_change)
                }
            }
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl<'a> Widget<'a> for Text<'a> {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget<'a>>,
    ) {
        map.insert(
            self.get_id(),
            ComputedWidget {
                x,
                y,
                z,
                width,
                height,
                render: Some(RenderObject::Text {
                    text: self.text,
                    style: TextStyle {
                        color: self.color,
                        size: self.size,
                        font: self.font,
                    },
                }),
            },
        );
    }

    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        _map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool) {
        (Some(event), prev_state_change)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl<'a> Widget<'a> for Rectangle {
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
                render: Some(RenderObject::Rectangle {
                    style: Style {
                        color: Some(self.color),
                        border_radius: self.border_radius,
                    },
                }),
            },
        );
    }

    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        _map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool) {
        (Some(event), prev_state_change)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl<'a> Widget<'a> for Empty {
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

    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        _map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool) {
        (Some(event), prev_state_change)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl<'a> Widget<'a> for Padding<'a> {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget<'a>>,
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
            .borrow()
            .compute(x + self.padding.0, y + self.padding.1, z, w, h, map)
    }

    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool) {
        self.child.borrow().dispatch(event, prev_state_change, map)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl<'a> Widget<'a> for Row<'a> {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget<'a>>,
    ) {
        let total_len = self.flex.iter().sum::<usize>();
        let each_child_width = width / total_len as f64;
        let mut prev_flex = 0;
        self.children.iter().enumerate().for_each(|(i, child)| {
            let flex = self.flex[i];
            let offset = prev_flex as f64 * each_child_width;
            prev_flex += flex;
            child.borrow().compute(
                x + offset,
                y,
                z,
                each_child_width * flex as f64,
                height,
                map,
            );
        });
    }

    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool) {
        let mut e = Some(event);
        let mut state_change = prev_state_change;
        for child in &self.children {
            if let Some(ev) = e {
                let r = child.borrow().dispatch(ev, prev_state_change, map);
                e = r.0;
                state_change = prev_state_change | r.1;
            } else {
                break;
            }
        }
        (e, state_change)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl<'a> Widget<'a> for Stack<'a> {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget<'a>>,
    ) {
        self.children
            .iter()
            .enumerate()
            .for_each(|(i, c)| c.borrow().compute(x, y, z + i, width, height, map));
    }

    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool) {
        let mut e = Some(event);
        let mut state_change = prev_state_change;
        for child in &self.children {
            if let Some(ev) = e {
                let r = child.borrow().dispatch(ev, prev_state_change, map);
                e = r.0;
                state_change = prev_state_change | r.1;
            } else {
                break;
            }
        }
        (e, state_change)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl<'a> Widget<'a> for Column<'a> {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget<'a>>,
    ) {
        let total_len = self.flex.iter().sum::<usize>();
        let each_child_height = height / total_len as f64;
        let mut prev_flex = 0;
        self.children.iter().enumerate().for_each(|(i, child)| {
            let flex = self.flex[i];
            let offset = prev_flex as f64 * each_child_height;
            prev_flex += flex;
            child.borrow().compute(
                x,
                y + offset,
                z,
                width,
                each_child_height * flex as f64,
                map,
            );
        });
    }

    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool) {
        let mut e = Some(event);
        let mut state_change = prev_state_change;
        for child in &self.children {
            if let Some(ev) = e {
                let r = child.borrow().dispatch(ev, prev_state_change, map);
                e = r.0;
                state_change = prev_state_change | r.1;
            } else {
                break;
            }
        }
        (e, state_change)
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
