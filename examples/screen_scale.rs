use nae::extras::Transform2d;
use nae::prelude::*;
use nae_core::math::Mat3;

#[derive(Clone, Copy, Debug)]
enum ScreenMode {
    None,
    Fill,
    AspectFit,
    AspectFill,
}

struct ScreenScaler {
    mode: ScreenMode,
    transform: Transform2d,
    scale_dirty: bool,
    container_size: (f32, f32),
    screen_size: (f32, f32),
    scale: (f32, f32),
}

impl ScreenScaler {
    fn new(width: f32, height: f32, mode: ScreenMode) -> Self {
        Self {
            container_size: (1.0, 1.0),
            screen_size: (width, height),
            mode,
            transform: Transform2d::new(width, height),
            scale_dirty: false,
            scale: (1.0, 1.0),
        }
    }

    fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.transform.set_position(x, y);
        self
    }

    fn position(&self) -> (f32, f32) {
        (self.transform.x, self.transform.y)
    }

    fn set_pivot(&mut self, x: f32, y: f32) -> &mut Self {
        self.transform.set_pivot(x, y);
        self
    }

    fn pivot(&self) -> (f32, f32) {
        (self.transform.pivot_x, self.transform.pivot_y)
    }

    fn set_anchor(&mut self, x: f32, y: f32) -> &mut Self {
        self.transform.set_anchor(x, y);
        self
    }

    fn anchor(&self) -> (f32, f32) {
        (self.transform.anchor_x, self.transform.anchor_y)
    }

    fn update(&mut self) {
        if !self.scale_dirty {
            return;
        }
        self.scale_dirty = false;

        let (sw, sh) = self.screen_size;
        self.transform.set_size(sw, sh);

        let (cw, ch) = self.container_size;
        let (scale_x, scale_y) = match self.mode {
            ScreenMode::Fill => (cw / sw, ch / sh),
            ScreenMode::AspectFit => {
                let scale = (cw / sw).min(ch / sh);
                (scale, scale)
            }
            ScreenMode::AspectFill => {
                let scale = (cw / sw).max(ch / sh);
                (scale, scale)
            }
            _ => (1.0, 1.0),
        };

        self.transform
            .set_scale(scale_x * self.scale.0, scale_y * self.scale.1);
    }

    pub fn set_scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.scale = (x, y);
        self.scale_dirty = true;
        self
    }

    pub fn scale(&self) -> (f32, f32) {
        self.scale
    }

    pub fn set_container_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.container_size = (width, height);
        self.scale_dirty = true;
        self
    }

    pub fn set_screen_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.screen_size = (width, height);
        self.scale_dirty = true;
        self
    }

    pub fn set_mode(&mut self, mode: ScreenMode) -> &mut Self {
        self.mode = mode;
        self.scale_dirty = true;
        self
    }

    pub fn mode(&self) -> ScreenMode {
        self.mode
    }

    pub fn matrix(&mut self) -> &Mat3 {
        if self.scale_dirty {
            self.update();
        }
        self.transform.matrix()
    }
}

impl Default for ScreenScaler {
    fn default() -> Self {
        Self::new(SCREEN_WIDTH, SCREEN_HEIGHT, ScreenMode::None)
    }
}

const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[nae::main]
fn main() {
    nae::init_with(create_scaler)
    .update(update)
    .draw(draw)
    .resizable()
    .build()
    .unwrap();
}

fn create_scaler(_ : &mut App) -> ScreenScaler {
    // Create the screen scaler using the Working size SCREEN_WIDTH and SCREEN HEIGHT
    let mut scaler = ScreenScaler::new(SCREEN_WIDTH, SCREEN_HEIGHT, ScreenMode::None);

    // Sets the anchor to the center of the size to set the position on the center of the app
    scaler.set_anchor(0.5, 0.5);

    scaler
}

fn update(app: &mut App, scaler: &mut ScreenScaler) {
    // Set the container size to the window size. This is done here to take in account the size when the user resize the window
    // but can be done listening the resize event too.
    scaler.set_container_size(app.width(), app.height());

    // We set the position of our screen to the center of the window (the anchor is already set)
    scaler.set_position(app.width() * 0.5, app.height() * 0.5);

    // With the keyboard we switch between scale modes
    if app.keyboard.was_pressed(KeyCode::A) {
        scaler.set_mode(ScreenMode::None);
    } else if app.keyboard.was_pressed(KeyCode::S) {
        scaler.set_mode(ScreenMode::Fill);
    } else if app.keyboard.was_pressed(KeyCode::D) {
        scaler.set_mode(ScreenMode::AspectFill);
    } else if app.keyboard.was_pressed(KeyCode::F) {
        scaler.set_mode(ScreenMode::AspectFit);
    }
}

fn draw(app: &mut App, scaler: &mut ScreenScaler) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));

    // First thing is push the matrix calculated by the scaler
    draw.push_matrix(scaler.matrix());

    // Draw a background that covers all the working size
    draw.set_color(rgba(0.5, 0.4, 0.3, 1.0));
    draw.rect(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT);

    // Draw some random shapes to see how the container change with the mode
    draw.set_color(Color::YELLOW);
    draw.circle(200.0, 200.0, 50.0);
    draw.stroke_rounded_rect(400.0, 400.0, 100.0, 100.0, 40.0, 10.0);

    // Just help text
    draw.set_color(Color::WHITE);
    draw.text_ext(
        &format!("Mode enabled: {:?}", scaler.mode()),
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.5,
        40.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );

    draw.set_color(hex(0xc0c0c0ff));
    draw.text("Press A to disable", 10.0, 10.0, 20.0);
    draw.text("Press S to enable Fill", 10.0, 30.0, 20.0);
    draw.text("Press D to enable AspectFill", 10.0, 50.0, 20.0);
    draw.text("Press F to enable AspectFit", 10.0, 70.0, 20.0);

    draw.text_ext(
        "Resize the screen to see how the container changes",
        SCREEN_WIDTH * 0.5,
        SCREEN_HEIGHT * 0.5 + 250.0,
        20.0,
        HorizontalAlign::Center,
        VerticalAlign::Center,
        None,
    );

    // Pop the matrix once we finish working
    draw.pop_matrix();
    draw.end();
}
