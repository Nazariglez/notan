use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 800;

#[derive(AppState)]
struct State {
    clear_color: Color,
    width: f32,
    height: f32,
    rotation: f32,
    skew_x: f32,
    skew_y: f32,
    color: (Color, Color, Color),
}

impl Default for State {
    fn default() -> Self {
        Self {
            clear_color: Color::BLACK,
            width: 400.0,
            height: 300.0,
            rotation: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
            color: (Color::WHITE, Color::WHITE, Color::WHITE),
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .size(WIDTH, HEIGHT)
        .multisampling(8)
        .vsync();

    notan::init_with(State::default)
        .set_config(win_config)
        .set_config(EguiConfig)
        .set_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    // Draw shape
    let mut draw = gfx.create_draw();
    draw.clear(state.clear_color);
    draw_shape(&mut draw, state);
    gfx.render(&draw);

    // Get the EGUI plugin that contains egui::Context
    let mut plugin = plugins.get_mut::<EguiPlugin>().unwrap();

    let output = plugin.run(|ctx| {
        // Draw the EGUI Widget here
        draw_egui_widget(ctx, state);
    });

    // Draw the context to the screen or to a RenderTexture
    gfx.render(&output);
}

// Draw a Triangle using the properties set on the state
fn draw_shape(draw: &mut Draw, state: &mut State) {
    let width = WIDTH as f32;
    let height = HEIGHT as f32;

    let a = (state.width * 0.5, 0.0);
    let b = (0.0, state.height);
    let c = (state.width, state.height);
    let center_x = (a.0 + b.0 + c.0) / 3.0;
    let center_y = (a.1 + b.1 + c.1) / 3.0;

    draw.triangle(a, b, c)
        .translate(width * 0.5 - center_x, height * 0.5 - center_y)
        .rotate_degrees_from((center_x, center_y), state.rotation)
        .skew(state.skew_x, state.skew_y)
        .color_vertex(state.color.0, state.color.1, state.color.2);
}

// Creates a widget to change the properties
fn draw_egui_widget(ctx: &egui::Context, state: &mut State) {
    egui::Window::new("Custom Shape Widget")
        .default_width(400.0)
        .show(ctx, |ui| draw_egui_ui(ui, state));
}

// UI Description
fn draw_egui_ui(ui: &mut egui::Ui, state: &mut State) {
    let mut clear_color = state.clear_color.rgba();
    let mut color_a = state.color.0.rgba();
    let mut color_b = state.color.1.rgba();
    let mut color_c = state.color.2.rgba();

    egui::Grid::new("custom_grid")
        .num_columns(2)
        .spacing([40.0, 6.0])
        // .striped(true)
        .show(ui, |ui| {
            ui.label("Clear color");
            ui.color_edit_button_rgba_premultiplied(&mut clear_color);
            ui.end_row();

            ui.label("Rotation");
            ui.add(egui::Slider::new(&mut state.rotation, 0.0..=360.0).suffix("°"));
            ui.end_row();

            ui.label("Width");
            ui.add(egui::Slider::new(&mut state.width, 0.0..=700.0));
            ui.end_row();

            ui.label("Height");
            ui.add(egui::Slider::new(&mut state.height, 0.0..=500.0));
            ui.end_row();

            ui.label("Vertex Color A");
            ui.color_edit_button_rgba_premultiplied(&mut color_a);
            ui.end_row();

            ui.label("Vertex Color B");
            ui.color_edit_button_rgba_premultiplied(&mut color_b);
            ui.end_row();

            ui.label("Vertex Color C");
            ui.color_edit_button_rgba_premultiplied(&mut color_c);
            ui.end_row();

            ui.label("Skew X");
            ui.add(egui::Slider::new(&mut state.skew_x, -1.0..=1.0));
            ui.end_row();

            ui.label("Skew Y");
            ui.add(egui::Slider::new(&mut state.skew_y, -1.0..=1.0));
            ui.end_row();
        });

    state.clear_color = clear_color.into();
    state.color = (color_a.into(), color_b.into(), color_c.into());
}
