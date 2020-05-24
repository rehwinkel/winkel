use gl;
use glutin::event::{ElementState, MouseButton};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::NotCurrent;
use glutin::WindowedContext;

pub fn run<F: FnOnce() -> Box<dyn Renderer>>(
    tree: Box<dyn Widget>,
    get_renderer: F,
    start_width: f64,
    start_height: f64,
    title: &str,
) {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title(title)
        .with_inner_size(glutin::dpi::LogicalSize::new(start_width, start_height));
    let windowed_context: WindowedContext<NotCurrent> = glutin::ContextBuilder::new()
        .with_double_buffer(Some(true))
        .with_gl(glutin::GlRequest::Latest)
        .with_multisampling(16)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current() }.unwrap();
    gl::load_with(|s| windowed_context.get_proc_address(s));

    let renderer: Box<dyn Renderer> = get_renderer();

    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;

    let mut computed = compute(&tree, start_width, start_height);

    el.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event,
        } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(sz) => {
                windowed_context.resize(sz);
                unsafe { gl::Viewport(0, 0, sz.width as i32, sz.height as i32) };
                computed = compute(&tree, sz.width as f64, sz.height as f64);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
                modifiers: _,
            } => {
                let button = match button {
                    MouseButton::Left => 0,
                    MouseButton::Middle => 1,
                    MouseButton::Right => 2,
                    MouseButton::Other(v) => v,
                };
                match state {
                    ElementState::Pressed => {
                        tree.dispatch(
                            WinkelEvent::MouseDown {
                                x: mouse_x,
                                y: mouse_y,
                                button,
                            },
                            &computed,
                        );
                    }
                    ElementState::Released => {
                        tree.dispatch(
                            WinkelEvent::MouseUp {
                                x: mouse_x,
                                y: mouse_y,
                                button,
                            },
                            &computed,
                        );
                    }
                }
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
                modifiers: _,
            } => {
                mouse_x = position.x;
                mouse_y = position.y;
            }
            _ => {}
        },
        Event::RedrawRequested(_window_id) => {
            unsafe {
                gl::ClearColor(1.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            let sz = windowed_context.window().inner_size();
            renderer.render(&computed, sz.width as f64, sz.height as f64);
            windowed_context.swap_buffers().unwrap();
        }
        _ => {}
    });
}

use winkel::colors;
use winkel::compute;
use winkel::widgets::*;
use winkel::Event as WinkelEvent;
use winkel::GlRenderer;
use winkel::Renderer;

fn main() {
    let tree: Box<dyn Widget> = Padding::new(
        Button::new(Rectangle::new(colors::RED).border(50.0).build())
            .on_click(|_| {
                println!("Clicked!");
            })
            .build(),
    )
    .symmetrical(20.0, 30.0)
    .build();
    run(tree, || Box::new(GlRenderer::new()), 1024.0, 768.0, "Test");
}
