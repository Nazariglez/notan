use notan_core::window::{CursorIcon, WindowId};
use winit::window::{CursorIcon as WCursorIcon, WindowId as WWindowId};

pub(crate) fn win_id(window_id: WWindowId) -> WindowId {
    let raw: u64 = window_id.into();
    raw.into()
}

pub(crate) fn cursor_id(cursor: CursorIcon) -> Option<WCursorIcon> {
    Some(match cursor {
        CursorIcon::None => return None,
        CursorIcon::Default => WCursorIcon::Default,
        CursorIcon::ContextMenu => WCursorIcon::ContextMenu,
        CursorIcon::Help => WCursorIcon::Help,
        CursorIcon::PointingHand => WCursorIcon::Pointer,
        CursorIcon::Progress => WCursorIcon::Progress,
        CursorIcon::Wait => WCursorIcon::Wait,
        CursorIcon::Cell => WCursorIcon::Cell,
        CursorIcon::Crosshair => WCursorIcon::Crosshair,
        CursorIcon::Text => WCursorIcon::Text,
        CursorIcon::VerticalText => WCursorIcon::VerticalText,
        CursorIcon::Alias => WCursorIcon::Alias,
        CursorIcon::Copy => WCursorIcon::Copy,
        CursorIcon::Move => WCursorIcon::Move,
        CursorIcon::NoDrop => WCursorIcon::NoDrop,
        CursorIcon::NotAllowed => WCursorIcon::NotAllowed,
        CursorIcon::Grab => WCursorIcon::Grab,
        CursorIcon::Grabbing => WCursorIcon::Grabbing,
        CursorIcon::AllScroll => WCursorIcon::AllScroll,
        CursorIcon::ResizeHorizontal => WCursorIcon::EwResize,
        CursorIcon::ResizeNeSw => WCursorIcon::NeswResize,
        CursorIcon::ResizeNwSe => WCursorIcon::NwseResize,
        CursorIcon::ResizeVertical => WCursorIcon::NsResize,
        CursorIcon::ZoomIn => WCursorIcon::ZoomIn,
        CursorIcon::ZoomOut => WCursorIcon::ZoomOut,
        CursorIcon::ResizeEast => WCursorIcon::EResize,
        CursorIcon::ResizeSouthEast => WCursorIcon::SeResize,
        CursorIcon::ResizeSouth => WCursorIcon::SResize,
        CursorIcon::ResizeSouthWest => WCursorIcon::SwResize,
        CursorIcon::ResizeWest => WCursorIcon::WResize,
        CursorIcon::ResizeNorthWest => WCursorIcon::NwResize,
        CursorIcon::ResizeNorth => WCursorIcon::NResize,
        CursorIcon::ResizeNorthEast => WCursorIcon::NeResize,
        CursorIcon::ResizeColumn => WCursorIcon::ColResize,
        CursorIcon::ResizeRow => WCursorIcon::RowResize,
    })
}
