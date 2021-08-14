use notan_app::prelude::mouse::MouseButton;
use notan_app::Event;
use winit::dpi::PhysicalPosition;
use winit::event::ElementState;
use winit::event::{MouseButton as WMouseButton, MouseScrollDelta, WindowEvent};

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
        WindowEvent::MouseWheel { delta, .. } => {
            let evt = match delta {
                MouseScrollDelta::LineDelta(x, y) => Event::MouseWheel {
                    delta_x: *x,
                    delta_y: *y,
                },
                MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => {
                    let delta_x = if *x > 0.0 {
                        (*x / 10.0).max(0.1)
                    } else {
                        (*x / 10.0).min(-0.1)
                    } as f32;

                    let delta_y = if *y > 0.0 {
                        (*y / 10.0).max(0.1)
                    } else {
                        (*y / 10.0).min(-0.1)
                    } as f32;
                    Event::MouseWheel { delta_x, delta_y }
                }
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
