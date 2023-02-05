use notan::draw::*;
use notan::prelude::*;

const BUTTON_SIZE: f32 = 25.0;
const MARGIN_X: f32 = 50.0;
const MARGIN_Y: f32 = 50.0;
const WIDTH: f32 = 700.0;
const HEIGHT: f32 = 500.0;

struct ColorButton {
    color: Color,
    x: f32,
    y: f32,
    selected: bool,
}

impl ColorButton {
    fn draw(&self, draw: &mut Draw) {
        draw.rect((self.x, self.y), (BUTTON_SIZE, BUTTON_SIZE))
            .color(self.color)
            .alpha(0.7);

        draw.rect((self.x, self.y), (BUTTON_SIZE, BUTTON_SIZE))
            .color(Color::WHITE)
            .stroke(2.0);

        if self.selected {
            draw.rect(
                (self.x - 2.0, self.y - 2.0),
                (BUTTON_SIZE + 4.0, BUTTON_SIZE + 4.0),
            )
            .color(Color::GREEN.with_alpha(0.7))
            .corner_radius(4.0)
            .stroke(3.0);
        }
    }
}

#[derive(AppState)]
pub struct State {
    rt: RenderTexture,
    last_x: f32,
    last_y: f32,
    colors: Vec<ColorButton>,
    color: Color,
    font: Font,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        // create the render texture
        let rt = gfx
            .create_render_texture(WIDTH as _, HEIGHT as _)
            .with_filter(TextureFilter::Linear, TextureFilter::Linear)
            .build()
            .unwrap();

        let font = gfx
            .create_font(include_bytes!("./assets/kenney_pixel-webfont.ttf"))
            .unwrap();

        // clear the texture with white
        {
            let mut draw = rt.create_draw();
            draw.clear(Color::WHITE);
            draw.text(&font, "Draw here!")
                .size(70.0)
                .color(Color::BLACK)
                .alpha(0.15)
                .v_align_middle()
                .h_align_center()
                .position(350.0, 250.0);

            gfx.render_to(&rt, &draw);
        }

        // colors to use
        let color_base = [
            Color::BLACK,
            Color::GRAY,
            Color::RED,
            Color::GREEN,
            Color::BLUE,
            Color::MAGENTA,
            Color::ORANGE,
            Color::YELLOW,
        ];

        let xx = 400.0 - (color_base.len() as f32 * (BUTTON_SIZE + 10.0)) * 0.5;
        let yy = 10.0;

        let colors: Vec<ColorButton> = color_base
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let n = i as f32;

                ColorButton {
                    x: xx + n * (BUTTON_SIZE + 10.0),
                    y: yy,
                    color: *c,
                    selected: i == 0,
                }
            })
            .collect();

        Self {
            rt,
            last_x: 0.0,
            last_y: 0.0,
            colors,
            color: Color::BLACK,
            font,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::default().vsync(true).lazy_loop(true);

    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if let Some(c) = select_color(&app.mouse, &mut state.colors) {
        state.color = c;
    }

    draw_board(&app.mouse, gfx, state);

    draw_ui(gfx, state);

    if app.keyboard.was_pressed(KeyCode::Space) {
        // Save the render texture to a file
        state.rt.to_file(gfx, "draw.png").unwrap();
        notan::log::info!("Saved file as 'draw.png'");
    }
}

fn draw_board(mouse: &Mouse, gfx: &mut Graphics, state: &mut State) {
    let mut draw = state.rt.create_draw();
    if mouse.x >= MARGIN_X
        && mouse.x <= MARGIN_X + WIDTH
        && mouse.y >= MARGIN_Y
        && mouse.y <= MARGIN_Y + HEIGHT
    {
        let x = mouse.x - MARGIN_X;
        let y = mouse.y - MARGIN_Y;

        if mouse.was_pressed(MouseButton::Left) {
            state.last_x = x;
            state.last_y = y;
        } else if mouse.is_down(MouseButton::Left) {
            draw.path()
                .move_to(state.last_x, state.last_y)
                .line_to(x, y)
                .stroke(10.0)
                .round_join()
                .round_cap()
                .color(state.color);

            state.last_x = x;
            state.last_y = y;
        }
    }

    gfx.render_to(&state.rt, &draw);
}

fn select_color(mouse: &Mouse, colors: &mut [ColorButton]) -> Option<Color> {
    let mut color = None;

    if mouse.was_pressed(MouseButton::Left) {
        let x = mouse.x;
        let y = mouse.y;
        let pos = colors.iter().position(|btn| {
            x >= btn.x && y >= btn.y && x <= btn.x + BUTTON_SIZE && y <= btn.y + BUTTON_SIZE
        });
        if let Some(index) = pos {
            colors.iter_mut().enumerate().for_each(|(i, btn)| {
                btn.selected = index == i;
                if btn.selected {
                    color = Some(btn.color);
                }
            });
        }
    }

    color
}

fn draw_ui(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::from_rgb(0.1, 0.1, 0.1));

    draw.image(&state.rt).translate(MARGIN_X, MARGIN_Y);

    draw.rect((MARGIN_X, MARGIN_Y), (WIDTH, HEIGHT))
        .color(Color::ORANGE)
        .corner_radius(10.0)
        .stroke(10.0);

    state.colors.iter().for_each(|btn| btn.draw(&mut draw));

    draw.text(&state.font, "Press SPACE to save")
        .h_align_center()
        .v_align_top()
        .position(400.0, 560.0)
        .size(30.0)
        .color(Color::WHITE);

    gfx.render(&draw);
}
