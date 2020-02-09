use crate::res::Texture;

/// Helper that represents an animation
pub struct Animation {
    /// List of textures used to display the animation
    pub frames: Vec<Texture>,
    /// Time used to calculated which frame display
    pub elapsed_time: f32,
    /// Time between frames
    pub frame_time: f32,
    /// Index of the current texture to display
    pub index: usize,
    /// Reverse the order of the animation
    pub reverse: bool,
    /// If is set to false the current index will never be update
    pub playing: bool,
    /// Repeat in a loop the animation
    pub repeat: bool,
}

impl Animation {
    /// Returns a new animation using a list of textures and a time per frame
    pub fn new(frames: Vec<Texture>, frame_time: f32) -> Self {
        Self {
            frames,
            elapsed_time: 0.0,
            frame_time,
            index: 0,
            reverse: false,
            playing: true,
            repeat: true,
        }
    }

    /// Reset the frames to the first one
    pub fn reset(&mut self) {
        self.elapsed_time = 0.0;
        self.index = 0;
    }

    //TODO is finished var? callbacks?

    /// Add the time between frame to calculate the current frame
    pub fn tick(&mut self, delta: f32) {
        if !self.playing {
            return;
        }

        let total_time = self.frames.len() as f32 * self.frame_time;
        let last_index = self.index;
        let time = (self.elapsed_time + delta) % total_time;
        let last_frame_num = self.frames.len() - 1;
        let mut current_index = (time / self.frame_time) as usize;

        let need_stop = if self.reverse {
            current_index = last_frame_num - current_index;
            !self.repeat && last_index == 0 && current_index == last_frame_num
        } else {
            !self.repeat && last_index == last_frame_num && current_index == 0
        };

        if need_stop {
            self.playing = false;
        } else {
            self.index = current_index;
        }

        self.elapsed_time = time;
    }

    /// Returns the current frame
    pub fn texture(&self) -> Option<&Texture> {
        self.frames.get(self.index)
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
