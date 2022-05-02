use egui::PointerButton;
use notan_core::keyboard::KeyCode;
use notan_core::mouse::MouseButton;

pub(crate) fn to_egui_pointer(btn: &MouseButton) -> Option<egui::PointerButton> {
    Some(match btn {
        MouseButton::Left => PointerButton::Primary,
        MouseButton::Right => PointerButton::Secondary,
        MouseButton::Middle => PointerButton::Middle,
        MouseButton::Other(_) => return None,
    })
}

pub(crate) fn to_egui_key(key: &KeyCode) -> Option<egui::Key> {
    Some(match key {
        KeyCode::Down => egui::Key::ArrowDown,
        KeyCode::Left => egui::Key::ArrowLeft,
        KeyCode::Right => egui::Key::ArrowRight,
        KeyCode::Up => egui::Key::ArrowUp,

        KeyCode::Escape => egui::Key::Escape,
        KeyCode::Tab => egui::Key::Tab,
        KeyCode::Back => egui::Key::Backspace,
        KeyCode::Return => egui::Key::Enter,
        KeyCode::Space => egui::Key::Space,

        KeyCode::Insert => egui::Key::Insert,
        KeyCode::Delete => egui::Key::Delete,
        KeyCode::Home => egui::Key::Home,
        KeyCode::End => egui::Key::End,
        KeyCode::PageUp => egui::Key::PageUp,
        KeyCode::PageDown => egui::Key::PageDown,

        KeyCode::Key0 => egui::Key::Num0,
        KeyCode::Key1 => egui::Key::Num1,
        KeyCode::Key2 => egui::Key::Num2,
        KeyCode::Key3 => egui::Key::Num3,
        KeyCode::Key4 => egui::Key::Num4,
        KeyCode::Key5 => egui::Key::Num5,
        KeyCode::Key6 => egui::Key::Num6,
        KeyCode::Key7 => egui::Key::Num7,
        KeyCode::Key8 => egui::Key::Num8,
        KeyCode::Key9 => egui::Key::Num9,

        KeyCode::A => egui::Key::A,
        KeyCode::B => egui::Key::B,
        KeyCode::C => egui::Key::C,
        KeyCode::D => egui::Key::D,
        KeyCode::E => egui::Key::E,
        KeyCode::F => egui::Key::F,
        KeyCode::G => egui::Key::G,
        KeyCode::H => egui::Key::H,
        KeyCode::I => egui::Key::I,
        KeyCode::J => egui::Key::J,
        KeyCode::K => egui::Key::K,
        KeyCode::L => egui::Key::L,
        KeyCode::M => egui::Key::M,
        KeyCode::N => egui::Key::N,
        KeyCode::O => egui::Key::O,
        KeyCode::P => egui::Key::P,
        KeyCode::Q => egui::Key::Q,
        KeyCode::R => egui::Key::R,
        KeyCode::S => egui::Key::S,
        KeyCode::T => egui::Key::T,
        KeyCode::U => egui::Key::U,
        KeyCode::V => egui::Key::V,
        KeyCode::W => egui::Key::W,
        KeyCode::X => egui::Key::X,
        KeyCode::Y => egui::Key::Y,
        KeyCode::Z => egui::Key::Z,

        _ => return None,
    })
}
