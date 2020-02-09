use crate::res::Texture;

/// Helper that represents an animation
pub struct Animation {
    frames: Vec<Texture>,
    time: f32,
    total_time: f32,
    frame_time: f32,
    index: usize,
}

impl Animation {
    /// Returns a new animation using a list of textures and a time per frame
    pub fn new(frames: Vec<Texture>, frame_time: f32) -> Self {
        let time = 0.0;
        let total_time = frames.len() as f32 * frame_time;
        Self {
            frames,
            time,
            total_time,
            frame_time,
            index: 0,
        }
    }

    /// Reset the frames to the first one
    pub fn reset(&mut self) {
        self.time = 0.0;
        self.index = 0;
    }

    //TODO play, reverse, get index, is finished? callbacks? loop? stop when it's finished?

    /// Add the time between frame to calculate the current frame
    pub fn tick(&mut self, delta: f32) {
        self.time = (self.time + delta) % self.total_time;
        self.index = (self.time / self.frame_time) as usize;
    }

    /// Returns the current frame
    pub fn texture(&self) -> &Texture {
        &self.frames[self.index]
    }

    /// Returns an animation from a single texture that
    /// have all the frames inside as a grid.
    pub fn from_grid(
        texture: &Texture,
        frame_time: f32,
        cols: usize,
        rows: usize,
        total_frames: Option<usize>,
        selected_frames: Option<Vec<usize>>,
    ) -> Self {
        let (xx, yy, width, height) = texture.frame();
        let ww = width / cols as f32;
        let hh = height / rows as f32;

        let frames = if let Some(indices) = selected_frames {
            indices
                .iter()
                .map(|i| texture_from_index(texture, *i, cols, ww, hh))
                .collect()
        } else if let Some(total) = total_frames {
            (0..total)
                .map(|i| texture_from_index(texture, i, cols, ww, hh))
                .collect()
        } else {
            (0..cols * rows)
                .map(|i| texture_from_index(texture, i, cols, ww, hh))
                .collect()
        };

        Self::new(frames, frame_time)
    }
}

fn texture_from_index(
    texture: &Texture,
    index: usize,
    cols: usize,
    width: f32,
    height: f32,
) -> Texture {
    let (xx, yy, _, _) = texture.frame();
    let col = (index % cols as usize) as f32;
    let row = (index / cols as usize) as f32;
    let frame_x = xx + width * col;
    let frame_y = yy + height * row;
    texture.with_frame(frame_x, frame_y, width, height)
}
