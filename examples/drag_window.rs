#![allow(clippy::single_match)]

use simple_logger::SimpleLogger;
use winit::{
    event::{ElementState, Event, KeyEvent, MouseButton, StartCause, WindowEvent},
    event_loop::EventLoop,
    keyboard::Key,
    window::{Window, WindowBuilder, WindowId},
};

#[path = "util/fill.rs"]
mod fill;

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new().unwrap();

    let window_1 = WindowBuilder::new().build(&event_loop).unwrap();
    let window_2 = WindowBuilder::new().build(&event_loop).unwrap();

    let mut switched = false;
    let mut entered_id = window_2.id();

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(StartCause::Init) => {
            eprintln!("Switch which window is to be dragged by pressing \"x\".")
        }
        Event::WindowEvent { event, window_id } => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                let window = if (window_id == window_1.id() && switched)
                    || (window_id == window_2.id() && !switched)
                {
                    &window_2
                } else {
                    &window_1
                };

                window.drag_window().unwrap()
            }
            WindowEvent::CursorEntered { .. } => {
                entered_id = window_id;
                name_windows(entered_id, switched, &window_1, &window_2)
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Released,
                        logical_key: Key::Character(c),
                        ..
                    },
                ..
            } if c == "x" => {
                switched = !switched;
                name_windows(entered_id, switched, &window_1, &window_2);
                println!("Switched!")
            }
            WindowEvent::RedrawRequested => {
                if window_id == window_1.id() {
                    fill::fill_window(&window_1);
                } else if window_id == window_2.id() {
                    fill::fill_window(&window_2);
                }
            }
            _ => (),
        },

        _ => (),
    })
}

fn name_windows(window_id: WindowId, switched: bool, window_1: &Window, window_2: &Window) {
    let (drag_target, other) =
        if (window_id == window_1.id() && switched) || (window_id == window_2.id() && !switched) {
            (&window_2, &window_1)
        } else {
            (&window_1, &window_2)
        };
    drag_target.set_title("drag target");
    other.set_title("winit window");
}
