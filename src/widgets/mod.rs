use super::{ComputedWidget, Event};
use std::collections::HashMap;

mod core;
mod extra;

pub trait Widget<'a> {
    fn compute(
        &self,
        x: f64,
        y: f64,
        z: usize,
        width: f64,
        height: f64,
        map: &mut HashMap<usize, ComputedWidget<'a>>,
    );
    fn dispatch(
        &self,
        event: Event,
        prev_state_change: bool,
        map: &HashMap<usize, ComputedWidget>,
    ) -> (Option<Event>, bool);
    fn get_id(&self) -> usize;
}

pub use self::core::*;
pub use self::extra::*;
