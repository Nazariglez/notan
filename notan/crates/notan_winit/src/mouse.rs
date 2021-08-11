use glutin::event::ElementState;
use notan_app::prelude::mouse::MouseButton;
use notan_app::Event;
use winit::event::{MouseButton as WMouseButton, WindowEvent};

pub fn process_events(event: &WindowEvent, mx: &mut i32, my: &mut i32) -> Option<Event> {
    match event {
        WindowEvent::MouseInput { state, button, .. } => {
            let evt = match state {
                ElementState::Pressed => Event::MouseDown {
                    button: mouse_button_to_nae(button),
                    x: *mx,
                    y: *my,
                },
                _ => Event::MouseUp {
                    button: mouse_button_to_nae(button),
                    x: *mx,
                    y: *my,
                },
            };

            Some(evt)
        }
        WindowEvent::CursorEntered { .. } => Some(Event::MouseEnter { x: *mx, y: *my }),
        WindowEvent::CursorLeft { .. } => Some(Event::MouseLeft { x: *mx, y: *my }),
        WindowEvent::CursorMoved { position, .. } => {
            *mx = position.x as _;
            *my = position.y as _;
            Some(Event::MouseMove { x: *mx, y: *my })
        }
        _ => None,
    }
}

fn mouse_button_to_nae(btn: &WMouseButton) -> MouseButton {
    match btn {
        WMouseButton::Left => MouseButton::Left,
        WMouseButton::Right => MouseButton::Right,
        WMouseButton::Middle => MouseButton::Middle,
        WMouseButton::Other(n) => MouseButton::Other(*n as _),
    }
}
