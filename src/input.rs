use nae_core::Event;

pub struct Mouse {
    pub x: f32,
    pub y: f32,
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    #[inline]
    pub(crate) fn process(&mut self, evt: &Event) {
        match evt {
            Event::MouseMove { x, y } => {
                self.x = *x as f32;
                self.y = *y as f32;
            }

            Event::MouseUp { x, y, .. } => {
                self.x = *x as f32;
                self.y = *y as f32;
            }

            Event::MouseDown { x, y, .. } => {
                self.x = *x as f32;
                self.y = *y as f32;
            }
            _ => {}
        }
    }
}
