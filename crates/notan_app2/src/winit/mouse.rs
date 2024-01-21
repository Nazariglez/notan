use notan_core::mouse::{MouseAction, MouseButton, MouseEvent};
use notan_core::window::WindowId;
use winit::event::{ElementState, MouseButton as WMouseButton, MouseScrollDelta};

pub(crate) fn process_input(
    window_id: WindowId,
    state: ElementState,
    btn: WMouseButton,
    pos: Option<(f32, f32)>,
) -> MouseEvent {
    let button = button_id(btn);
    let (x, y) = pos.unwrap_or((0.0, 0.0));
    match state {
        ElementState::Pressed => MouseEvent {
            window_id,
            action: MouseAction::ButtonPressed { button },
            x,
            y,
        },
        ElementState::Released => MouseEvent {
            window_id,
            action: MouseAction::ButtonReleased { button },
            x,
            y,
        },
    }
}

pub(crate) fn process_motion(
    window_id: WindowId,
    pos: (f32, f32),
    old: Option<(f32, f32)>,
) -> MouseEvent {
    let (x, y) = pos;
    let (relative_x, relative_y) = match old {
        None => (0.0, 0.0),
        Some(old_pos) => (x - old_pos.0, y - old_pos.1),
    };
    MouseEvent {
        window_id,
        x,
        y,
        action: MouseAction::Move {
            relative_x,
            relative_y,
        },
    }
}

pub(crate) fn process_enter(window_id: WindowId, pos: Option<(f32, f32)>) -> MouseEvent {
    let (x, y) = pos.unwrap_or((0.0, 0.0));
    MouseEvent {
        window_id,
        action: MouseAction::Enter,
        x,
        y,
    }
}

pub(crate) fn process_leave(window_id: WindowId, pos: Option<(f32, f32)>) -> MouseEvent {
    let (x, y) = pos.unwrap_or((0.0, 0.0));
    MouseEvent {
        window_id,
        action: MouseAction::Left,
        x,
        y,
    }
}

pub(crate) fn process_wheel(
    window_id: WindowId,
    delta: MouseScrollDelta,
    scale_factor: f64,
    mouse_pos: Option<(f32, f32)>,
) -> MouseEvent {
    let (mx, my) = mouse_pos.unwrap_or((0.0, 0.0));
    match delta {
        MouseScrollDelta::LineDelta(x, y) => MouseEvent {
            window_id,
            action: MouseAction::Wheel {
                delta_x: x * 50.0,
                delta_y: y * 50.0,
            },
            x: mx,
            y: my,
        },
        MouseScrollDelta::PixelDelta(position) => {
            let pos = position.to_logical::<f64>(scale_factor);

            let delta_x = if pos.x > 0.0 {
                (pos.x / 10.0).max(0.1)
            } else {
                (pos.x / 10.0).min(-0.1)
            } as f32;

            let delta_y = if pos.y > 0.0 {
                (pos.y / 10.0).max(0.1)
            } else {
                (pos.y / 10.0).min(-0.1)
            } as f32;
            MouseEvent {
                window_id,
                action: MouseAction::Wheel { delta_x, delta_y },
                x: mx,
                y: my,
            }
        }
    }
}

fn button_id(btn: WMouseButton) -> MouseButton {
    match btn {
        WMouseButton::Left => MouseButton::Left,
        WMouseButton::Right => MouseButton::Right,
        WMouseButton::Middle => MouseButton::Middle,
        WMouseButton::Other(n) => MouseButton::Other(n as _),
    }
}
