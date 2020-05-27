use gl;
use glfw::{Action, Context};

pub fn run<'a, F: FnOnce() -> Box<dyn Renderer>>(
    tree: Rc<RefCell<dyn Widget<'a> + 'a>>,
    get_renderer: F,
    start_width: u32,
    start_height: u32,
    title: &str,
) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(start_width, start_height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));

    let mut renderer: Box<dyn Renderer> = get_renderer();

    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;
    let mut win_width: f64 = start_width as f64;
    let mut win_height: f64 = start_height as f64;

    let mut computed = compute(&tree, start_width as f64, start_height as f64);

    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_size_polling(true);
    window.set_cursor_enter_polling(true);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        renderer.render(&computed, win_width, win_height);
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::CursorEnter(entered) => {
                    if !entered {
                        if tree
                            .borrow()
                            .dispatch(
                                WinkelEvent::MouseMove {
                                    prev_x: mouse_x,
                                    prev_y: mouse_y,
                                    x: -1.0,
                                    y: -1.0,
                                },
                                false,
                                &computed,
                            )
                            .1
                        {
                            computed = compute(&tree, win_width, win_height);
                        }
                        mouse_x = -1.0;
                        mouse_y = -1.0;
                    }
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    if tree
                        .borrow()
                        .dispatch(
                            WinkelEvent::MouseMove {
                                prev_x: mouse_x,
                                prev_y: mouse_y,
                                x,
                                y,
                            },
                            false,
                            &computed,
                        )
                        .1
                    {
                        computed = compute(&tree, win_width, win_height);
                    }
                    mouse_x = x;
                    mouse_y = y;
                }
                glfw::WindowEvent::MouseButton(button, Action::Press, _) => {
                    if tree
                        .borrow()
                        .dispatch(
                            WinkelEvent::MouseDown {
                                x: mouse_x,
                                y: mouse_y,
                                button: button as i32 as u8,
                            },
                            false,
                            &computed,
                        )
                        .1
                    {
                        computed = compute(&tree, win_width, win_height);
                    }
                }
                glfw::WindowEvent::MouseButton(button, Action::Release, _) => {
                    if tree
                        .borrow()
                        .dispatch(
                            WinkelEvent::MouseUp {
                                x: mouse_x,
                                y: mouse_y,
                                button: button as i32 as u8,
                            },
                            false,
                            &computed,
                        )
                        .1
                    {
                        computed = compute(&tree, win_width, win_height);
                    }
                }
                glfw::WindowEvent::Size(width, height) => {
                    win_width = width as f64;
                    win_height = height as f64;
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    };
                    computed = compute(&tree, width as f64, height as f64);
                }
                _ => {}
            }
        }
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        renderer.render(&computed, win_width, win_height);
        window.swap_buffers();
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use winkel::color;
use winkel::compute;
use winkel::widgets::*;
use winkel::Event as WinkelEvent;
use winkel::GlRenderer;
use winkel::Renderer;
use winkel::State;

fn main() {
    let mut button1: State<Rectangle> = State::new();
    let tree: Rc<RefCell<dyn Widget>> = Padding::new(
        Button::new(color::RED)
            .border(20.0)
            .child(
                Column::new()
                    .add(
                        Text::new("Hello World", 20, "Raleway-Regular.ttf")
                            .color(color::BLUE)
                            .build(),
                    )
                    .add(
                        Text::new("Hello World 2", 54, "Raleway-Regular.ttf")
                            .color(color::MAGENTA)
                            .build(),
                    )
                    .build(),
            )
            .on_pressed(|_| {
                println!("Clicked!");
            })
            .hover(color::YELLOW)
            .active(color::GREEN)
            .build_state(&mut button1),
    )
    .all(30.0)
    .build();
    run(tree, || Box::new(GlRenderer::new()), 1024, 768, "Test");
}
