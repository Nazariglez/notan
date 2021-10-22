use glam::{Mat3, Vec2};
use notan::draw::*;
use notan::prelude::*;

// Hierarchy
//  - A
//    - B
//      - C
//    - D
//    - E

#[derive(AppState)]
struct State {
    count: f32,
    container_a: Container,
    container_b: Container,
    container_c: Container,
    container_d: Container,
    container_e: Container,
}

impl State {
    fn new() -> Self {
        let container_a = Container::new(500.0, 500.0, Color::WHITE);
        let container_b = Container::new(230.0, 350.0, Color::from_rgb(1.0, 1.0, 0.5));
        let container_c = Container::new(130.0, 130.0, Color::from_rgb(0.5, 1.0, 0.5));
        let container_d = Container::new(130.0, 130.0, Color::from_rgb(1.0, 0.5, 1.0));
        let container_e = Container::new(130.0, 130.0, Color::from_rgb(0.5, 1.0, 1.0));

        Self {
            count: 0.0,
            container_a,
            container_b,
            container_c,
            container_d,
            container_e,
        }
    }
}

fn draw_container<F: FnMut(&mut Draw, &Container)>(
    draw: &mut Draw,
    container: &Container,
    mut f: F,
) {
    // Push the matrix to the stack
    draw.transform().push(container.matrix());

    draw.rect((0.0, 0.0), (container.size.x, container.size.y))
        .color(container.color)
        .stroke(5.0);

    // Anything draw after the push will behave as a "children" of this container
    f(draw, container);

    // After we finish drawing this container we pop the matrix from the stack
    draw.transform().pop();
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .set_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    // Draw Container A as root
    draw_container(&mut draw, &state.container_a, |draw, _c| {
        // B is a child of A
        draw_container(draw, &state.container_b, |draw, _c| {
            // C is a child of B
            draw_container(draw, &state.container_c, |draw, c| {
                draw_triangle(draw, c.size, state.count.cos());
            });
        });

        // D is a child of A
        draw_container(draw, &state.container_d, |draw, c| {
            draw_triangle(draw, c.size, state.count.cos());
        });

        // E is a child of A
        draw_container(draw, &state.container_e, |draw, c| {
            draw_triangle(draw, c.size, state.count.cos());
        });
    });

    // Render the frame
    gfx.render(&draw);
}

// Animate the containers to show how the matrix of the parents affects the children
fn update(app: &mut App, state: &mut State) {
    let offset = state.count.sin();
    let center_x = app.window().width() as f32 * 0.5;
    let center_y = app.window().height() as f32 * 0.5;

    // Container A, relative to the screen
    state.container_a.pos.x = center_x - state.container_a.size.x * 0.5 + offset * 120.0;
    state.container_a.pos.y = center_y - state.container_a.size.y * 0.5;

    // Container B, relative to Container A
    state.container_b.pos.x = 20.0;
    state.container_b.pos.y =
        state.container_a.size.y * 0.5 - state.container_b.size.y * 0.5 + offset * 50.0;

    // Container C, relative to Container B
    state.container_c.pos.x =
        state.container_b.size.x * 0.5 - state.container_c.size.x * 0.5 - offset * 32.0;
    state.container_c.pos.y =
        state.container_b.size.y * 0.5 - state.container_c.size.y * 0.5 - offset * 90.0;

    // Container D, relative to Container A
    state.container_d.pos.x = state.container_a.size.x * 0.75 - state.container_d.size.x * 0.5;
    state.container_d.pos.y = state.container_a.size.y * 0.25 - state.container_d.size.y * 0.5;
    state.container_d.rotation = offset * 1.0;

    // Container E, relative to Container A
    state.container_e.pos.x = state.container_a.size.x * 0.75 - state.container_e.size.x * 0.5;
    state.container_e.pos.y = state.container_a.size.y * 0.75 - state.container_e.size.y * 0.5;
    state.container_e.scale.x = 1.0 - offset * 0.3;
    state.container_e.scale.y = 1.0 + offset * 0.3;

    state.count += app.timer.delta_f32();
}

struct Container {
    pos: Vec2,
    rotation: f32,
    scale: Vec2,
    size: Vec2,
    color: Color,
}

impl Container {
    fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
            size: Vec2::new(width, height),
            color,
        }
    }

    // Returns a matrix calculated from the container's attributes
    fn matrix(&self) -> Mat3 {
        let translation = Mat3::from_translation(self.pos);
        let rotation = Mat3::from_angle(self.rotation);
        let scale = Mat3::from_scale(self.scale);
        return translation * rotation * scale;
    }
}

// Draw a triangle with a local matrix
fn draw_triangle(draw: &mut Draw, size: Vec2, offset: f32) {
    let a = (size.x * 0.5, size.y * 0.2);
    let b = (size.x * 0.2, size.y * 0.8);
    let c = (size.x * 0.8, size.y * 0.8);
    let rotation = offset * 360.0 * 0.5;

    draw.triangle(a, b, c)
        .alpha(0.7)
        .rotate_degrees_from((size.x * 0.5, size.y * 0.5), rotation)
        .color_vertex(Color::RED, Color::GREEN, Color::BLUE);
}
