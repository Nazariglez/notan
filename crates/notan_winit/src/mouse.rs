use glutin::dpi::LogicalPosition;

use glutin::event::ElementState;
use glutin::event::{MouseButton as WMouseButton, MouseScrollDelta, WindowEvent};
use notan_core::events::Event;
use notan_core::mouse::MouseButton;

pub fn process_events(
    event: &WindowEvent,
    mx: &mut i32,
    my: &mut i32,
    scale_factor: f64,
) -> Option<Event> {
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
                MouseScrollDelta::PixelDelta(position) => {
                    let LogicalPosition { x, y } = position.to_logical::<f64>(scale_factor);

                    let delta_x = if x > 0.0 {
                        (x / 10.0).max(0.1)
                    } else {
                        (x / 10.0).min(-0.1)
                    } as f32;

                    let delta_y = if y > 0.0 {
                        (y / 10.0).max(0.1)
                    } else {
                        (y / 10.0).min(-0.1)
                    } as f32;
                    Event::MouseWheel { delta_x, delta_y }
                }
            };
            Some(evt)
        }
        WindowEvent::CursorEntered { .. } => Some(Event::MouseEnter { x: *mx, y: *my }),
        WindowEvent::CursorLeft { .. } => Some(Event::MouseLeft { x: *mx, y: *my }),
        WindowEvent::CursorMoved { position, .. } => {
            let position = position.to_logical::<f64>(scale_factor);
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
