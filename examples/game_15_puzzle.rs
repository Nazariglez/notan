use notan::draw::*;
use notan::prelude::*;
use notan::random::rand::prelude::*;

const COLS: usize = 4;
const NUMBERS: usize = COLS * COLS;
const TILE_SIZE: f32 = 100.0;
const BOARD_SIZE: f32 = COLS as f32 * TILE_SIZE;

const FILL_COLOR: Color = Color::from_rgb(0.9, 0.9, 0.9);
const OUTLINE_COLOR: Color = Color::from_rgb(0.0, 0.8, 0.7);

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default().size(BOARD_SIZE as _, BOARD_SIZE as _);
    notan::init_with(State::new)
        .add_config(win)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    if app.mouse.was_pressed(MouseButton::Left) {
        if let Some((x, y)) = mouse_pos_to_point(app.mouse.x, app.mouse.y) {
            if let Some(dir) = state.board.can_move_to(x, y) {
                let (tx, ty) = match dir {
                    Direction::Left => (x - 1, y),
                    Direction::Right => (x + 1, y),
                    Direction::Up => (x, y - 1),
                    Direction::Down => (x, y + 1),
                };

                state.board.move_tile((x, y), (tx, ty));
                return;
            }
        }
    }

    if state.board.is_solved() && app.mouse.was_pressed(MouseButton::Left) {
        state.reset();
    }
}

fn mouse_pos_to_point(x: f32, y: f32) -> Option<(usize, usize)> {
    let in_x_bounds = x >= 0.0 && x < TILE_SIZE * COLS as f32;
    let in_y_bounds = y >= 0.0 && y < TILE_SIZE * COLS as f32;
    let in_bounds = in_x_bounds && in_y_bounds;
    if !in_bounds {
        return None;
    }

    let xx = (x / TILE_SIZE).floor() as usize;
    let yy = (y / TILE_SIZE).floor() as usize;
    Some((xx, yy))
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    for y in 0..COLS {
        for x in 0..COLS {
            let value = state.board.value(x, y);
            draw_tile(&mut draw, &state.font, x, y, value);
        }
    }

    if state.board.is_solved() {
        let (ww, hh) = gfx.size();
        let (ww, hh) = (ww as f32, hh as f32);

        draw.rect((0.0, 0.0), (ww, hh))
            .color(Color::BLACK)
            .alpha(0.7);

        draw.text(&state.font, "Done!")
            .color(Color::ORANGE)
            .size(74.0)
            .position(ww * 0.5, hh * 0.5)
            .h_align_center()
            .v_align_bottom();

        draw.text(&state.font, "Tap to reset")
            .color(Color::GRAY)
            .size(54.0)
            .v_align_top()
            .h_align_center()
            .position(ww * 0.5, hh * 0.5 + 20.0);
    }

    gfx.render(&draw);
}

fn draw_tile(draw: &mut Draw, font: &Font, x: usize, y: usize, value: u8) {
    if value == 0 {
        return;
    }

    let xx = x as f32 * TILE_SIZE;
    let yy = y as f32 * TILE_SIZE;
    draw.rect((xx, yy), (TILE_SIZE, TILE_SIZE))
        .corner_radius(10.0)
        .color(FILL_COLOR);

    draw.rect((xx, yy), (TILE_SIZE, TILE_SIZE))
        .corner_radius(10.0)
        .color(OUTLINE_COLOR)
        .stroke(5.0);

    draw.text(font, &format!("{}", value))
        .color(Color::BLACK)
        .size(34.0)
        .position(xx + TILE_SIZE * 0.5, yy + TILE_SIZE * 0.5)
        .h_align_center()
        .v_align_middle();
}

#[derive(AppState)]
struct State {
    font: Font,
    board: Board,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let font = gfx
            .create_font(include_bytes!("./assets/Ubuntu-B.ttf"))
            .unwrap();
        let board = Board::new();

        Self { font, board }
    }

    fn reset(&mut self) {
        self.board = Board::new();
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Board {
    grid: [u8; NUMBERS],
}

impl Board {
    /// Create a new board using random numbers
    fn new() -> Self {
        // generate an initial board, and move randomly from it.

        let mut grid = [0; NUMBERS];
        grid.iter_mut().enumerate().for_each(|(i, n)| {
            if i == NUMBERS - 1 {
                return;
            }

            *n = i as u8 + 1;
        });

        // move blank cell randomly.

        let mut x = (COLS - 1) as i32;
        let mut y = (COLS - 1) as i32;

        for _ in 0..1000 {
            const DIRS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
            let (dx, dy) = DIRS.choose(&mut thread_rng()).unwrap();
            let x_nxt = x + dx;
            let y_nxt = y + dy;

            const RANGE: std::ops::Range<i32> = 0..COLS as i32;
            if !RANGE.contains(&x_nxt) || !RANGE.contains(&y_nxt) {
                continue;
            }

            let index = index_from_point(x as usize, y as usize);
            let index_nxt = index_from_point(x_nxt as usize, y_nxt as usize);
            grid.swap(index, index_nxt);

            x = x_nxt;
            y = y_nxt;
        }

        Self { grid }
    }

    fn is_solved(&self) -> bool {
        for i in 0..NUMBERS - 1 {
            if self.grid[i] != i as u8 + 1 {
                return false;
            }
        }
        true
    }

    fn value(&self, x: usize, y: usize) -> u8 {
        let index = index_from_point(x, y);
        self.grid[index]
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.value(x, y) == 0
    }

    fn move_tile(&mut self, pos1: (usize, usize), pos2: (usize, usize)) {
        let index1 = index_from_point(pos1.0, pos1.1);
        let index2 = index_from_point(pos2.0, pos2.1);
        self.grid.swap(index1, index2);
    }

    fn can_move_to(&self, x: usize, y: usize) -> Option<Direction> {
        if x >= 1 && self.is_empty(x - 1, y) {
            return Some(Direction::Left);
        }

        if x <= COLS - 2 && self.is_empty(x + 1, y) {
            return Some(Direction::Right);
        }

        if y >= 1 && self.is_empty(x, y - 1) {
            return Some(Direction::Up);
        }

        if y <= COLS - 2 && self.is_empty(x, y + 1) {
            return Some(Direction::Down);
        }

        None
    }
}

#[inline]
fn index_from_point(x: usize, y: usize) -> usize {
    debug_assert!(x < COLS || y < COLS, "Point index out of bounds.");
    y * COLS + x
}
