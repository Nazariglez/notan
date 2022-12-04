use winit::event::{Touch, TouchPhase, WindowEvent};
use notan_core::events::Event;

pub fn process_events(event: &WindowEvent, scale_factor: f64) -> Option<Event> {
    match event {
        WindowEvent::Touch(Touch {
            phase,
            location,
            id,
            ..
        }) => {
            let pos = location.to_logical(scale_factor);
            let id = *id;
            let x = pos.x;
            let y = pos.y;
            Some(match phase {
                TouchPhase::Started => Event::TouchStart { id, x, y },
                TouchPhase::Moved => Event::TouchMove { id, x, y },
                TouchPhase::Ended => Event::TouchEnd { id, x, y },
                TouchPhase::Cancelled => Event::TouchCancel { id, x, y },
            })
        }
        _ => None,
    }
}
