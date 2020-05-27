use super::super::color::Color;
use super::super::State;
use super::core::*;
use super::Widget;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Button<'a> {
    child: Option<Rc<RefCell<dyn Widget<'a> + 'a>>>,
    base_color: Color,
    hover_color: Color,
    active_color: Color,
    pressed_callback: Option<Box<dyn Fn(u8) + 'a>>,
    border_radius: f64,
}

impl<'a> Button<'a> {
    pub fn new(base_color: Color) -> Button<'a> {
        Button {
            child: None,
            pressed_callback: None,
            base_color,
            hover_color: base_color,
            active_color: base_color,
            border_radius: 0.0,
        }
    }

    pub fn child(mut self, child: Rc<RefCell<dyn Widget<'a> + 'a>>) -> Self {
        self.child = Some(child);
        self
    }

    pub fn hover(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    pub fn active(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    pub fn border(mut self, border: f64) -> Self {
        self.border_radius = border;
        self
    }

    pub fn on_pressed<F: Fn(u8) + 'a>(mut self, on_pressed: F) -> Self {
        self.pressed_callback = Some(Box::new(on_pressed));
        self
    }

    pub fn build_state(self, rect_state: &'a mut State<Rectangle>) -> Rc<RefCell<dyn Widget + 'a>> {
        let active_color = self.active_color;
        let hover_color = self.hover_color.clone();
        let base_color = self.base_color.clone();
        let rect = Rectangle::new(self.base_color)
            .border(self.border_radius)
            .build_stateful(rect_state);
        let c_rect_state: &'a State<Rectangle> = rect_state;
        let pressed_callback = self.pressed_callback;
        let mut stack_builder = Stack::new().add(rect);
        if let Some(child) = self.child {
            stack_builder = stack_builder.add(child);
        }
        MouseGesture::new(stack_builder.build())
            .border(self.border_radius)
            .on_click(move |_| {
                c_rect_state.borrow_mut().color = active_color;
                true
            })
            .on_release(move |button| {
                c_rect_state.borrow_mut().color = hover_color;
                if let Some(pressed) = &pressed_callback {
                    pressed(button);
                }
                true
            })
            .on_enter(move || {
                c_rect_state.borrow_mut().color = hover_color;
                true
            })
            .on_leave(move || {
                c_rect_state.borrow_mut().color = base_color;
                true
            })
            .build()
    }
}
