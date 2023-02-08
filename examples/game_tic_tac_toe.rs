use notan::draw::*;
use notan::math::{vec2, Mat3, Vec2};
use notan::prelude::*;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 600.0;
const MARGIN: f32 = 50.0;

const WINNER_COMBINATIONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [6, 4, 2],
];

#[derive(Default, Copy, Clone, PartialEq)]
enum Player {
    #[default]
    Empty,
    Circle,
    Cross,
}

#[derive(AppState)]
struct State {
    rng: Random,
    font: Font,
    turn: Player,
    table: [Player; 9],
    winner: Option<Player>,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let font = gfx
            .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
            .unwrap();

        let mut rng = Random::default();
        let turn = if rng.gen_bool(0.5) {
            Player::Cross
        } else {
            Player::Circle
        };

        State {
            rng,
            font,
            turn,
            table: Default::default(),
            winner: None,
        }
    }

    fn reset(&mut self) {
        self.turn = if self.rng.gen_bool(0.5) {
            Player::Cross
        } else {
            Player::Circle
        };

        self.table = Default::default();
        self.winner = None;
    }
}

fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .set_multisampling(8)
        .set_size(WIDTH as _, HEIGHT as _)
        .set_vsync(true);

    notan::init_with(State::new)
        .add_config(win)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    if state.winner.is_some() {
        if app.keyboard.was_pressed(KeyCode::Space) {
            state.reset();
        }
        return;
    }

    let x = MARGIN;
    let y = MARGIN;
    let width = WIDTH - MARGIN * 2.0;
    let height = HEIGHT - MARGIN * 2.0;

    let tile_width = width / 3.0;
    let tile_height = height / 3.0;

    let (mx, my) = app.mouse.position();
    if app.mouse.was_pressed(MouseButton::Left) {
        // check bounds
        if mx < x || mx > x + width {
            return;
        }

        if my < y || my > y + height {
            return;
        }

        // inside the table
        let col = ((mx - x) / tile_width).floor();
        let row = ((my - y) / tile_height).floor();
        let index = index_from_pos(col as _, row as _);

        // set piece
        let is_empty = matches!(state.table[index], Player::Empty);
        if !is_empty {
            return;
        }
        state.table[index] = state.turn;

        // change turn
        let current_turn = state.turn;
        state.turn = match state.turn {
            Player::Empty => unreachable!(),
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        };

        // game over
        if let Some(winner) = check_winner(&state.table, current_turn) {
            state.winner = Some(winner);
        }
    }
}

fn check_winner(table: &[Player; 9], current_turn: Player) -> Option<Player> {
    for combo in WINNER_COMBINATIONS {
        let mut winner = true;
        for index in combo {
            let player = table[index];
            if current_turn != player {
                winner = false;
                break;
            }
        }

        if winner {
            return Some(current_turn);
        }
    }

    let full = !table.iter().any(|player| matches!(player, Player::Empty));
    if full {
        Some(Player::Empty)
    } else {
        None
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    let x = MARGIN;
    let y = MARGIN;
    let width = WIDTH - MARGIN * 2.0;
    let height = HEIGHT - MARGIN * 2.0;

    let tile_width = width / 3.0;
    let tile_height = height / 3.0;
    let padding = 20.0;

    // draw "who is playing"
    let size = vec2(tile_width, tile_height);
    draw_text(
        &mut draw,
        &state.font,
        size,
        state.turn,
        "Playing: ",
        24.0,
        vec2(300.0, MARGIN * 0.5),
        0.2,
    );

    // drawing board
    draw.rect((x, y), (width, height))
        .stroke_color(Color::WHITE)
        .stroke(6.0);

    for index in 1..3 {
        draw.line(
            (x + tile_width * index as f32, y + padding),
            (x + tile_width * index as f32, y + height - padding),
        )
        .width(2.0);
        draw.line(
            (x + padding, y + tile_height * index as f32),
            (x + width - padding, y + tile_height * index as f32),
        )
        .width(2.0);
    }

    // drawing pieces
    state.table.iter().enumerate().for_each(|(i, p)| {
        let pos = pos_from_index(i) * size + vec2(x, y) + size * 0.5;
        match p {
            Player::Circle => draw_circle(&mut draw, size, pos),
            Player::Cross => draw_cross(&mut draw, size, pos),
            Player::Empty => {}
        }
    });

    // draw final menu
    if let Some(winner) = state.winner {
        draw.rect((0.0, 0.0), (WIDTH, HEIGHT))
            .color(Color::BLACK)
            .alpha(0.8);

        let (text, x_offet) = if matches!(winner, Player::Empty) {
            ("Tie", 0.0)
        } else {
            ("Winner: ", size.x * 0.3)
        };

        draw_text(
            &mut draw,
            &state.font,
            size,
            winner,
            text,
            48.0,
            vec2(WIDTH * 0.5 - x_offet, HEIGHT * 0.5),
            0.6,
        );

        draw.text(&state.font, "Press SPACE to reset")
            .position(WIDTH * 0.5, HEIGHT * 0.75)
            .size(32.0)
            .h_align_center()
            .v_align_middle();
    }

    gfx.render(&draw);
}

#[allow(clippy::too_many_arguments)]
fn draw_text(
    draw: &mut Draw,
    font: &Font,
    size: Vec2,
    player: Player,
    text: &str,
    font_size: f32,
    pos: Vec2,
    scale: f32,
) {
    // drawing text
    draw.text(font, text)
        .color(Color::WHITE)
        .size(font_size)
        .v_align_middle()
        .h_align_center()
        .position(pos.x, pos.y);

    let bounds = draw.last_text_bounds();

    let pos = vec2(bounds.max_x() + 30.0, bounds.center_y());
    let mm = Mat3::from_translation(pos)
        * Mat3::from_scale(Vec2::splat(scale))
        * Mat3::from_translation(-pos);
    draw.transform().push(mm);
    match player {
        Player::Circle => draw_circle(draw, size, pos),
        Player::Cross => draw_cross(draw, size, pos),
        Player::Empty => {}
    }
    draw.transform().pop();
}

fn draw_circle(draw: &mut Draw, size: Vec2, pos: Vec2) {
    let radius = size.x * 0.5 - 40.0;
    draw.circle(radius)
        .position(pos.x, pos.y)
        .stroke_color(Color::ORANGE)
        .stroke(10.0);
}

fn draw_cross(draw: &mut Draw, size: Vec2, pos: Vec2) {
    let line_size = size.x - 80.0;
    let half_size = line_size * 0.5;

    let p1a = pos - half_size;
    let p1b = pos + half_size;
    draw.path()
        .move_to(p1a.x, p1a.y)
        .line_to(p1b.x, p1b.y)
        .color(Color::MAGENTA)
        .stroke(10.0);

    let p2a = pos - vec2(half_size, -half_size);
    let p2b = pos + vec2(half_size, -half_size);
    draw.path()
        .move_to(p2a.x, p2a.y)
        .line_to(p2b.x, p2b.y)
        .color(Color::MAGENTA)
        .stroke(10.0);
}

fn pos_from_index(i: usize) -> Vec2 {
    let x = i % 3;
    let y = i / 3;
    vec2(x as _, y as _)
}

fn index_from_pos(x: usize, y: usize) -> usize {
    y * 3 + x
}
