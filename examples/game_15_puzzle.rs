use notan::draw::*;
use notan::prelude::*;
use std::ops::Rem;

const COLS: usize = 4;
const NUMBERS: usize = COLS * COLS;
const TILE_SIZE: f32 = 100.0;
const BOARD_SIZE: f32 = COLS as f32 * TILE_SIZE;

const FILL_COLOR: Color = Color::from_rgb(0.9, 0.9, 0.9);
const OUTLINE_COLOR: Color = Color::from_rgb(0.0, 0.8, 0.7);
const TEXT_COLOR: Color = Color::BLACK;

// TODO resolve and reset

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
            }
        }
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
        let mut bag = get_bag_of_numbers();
        let mut grid = [0; NUMBERS].map(|i| *bag.item().unwrap());
        Self { grid }
    }

    fn value(&self, x: usize, y: usize) -> u8 {
        let index = index_from_point(x, y);
        self.grid[index]
    }

    fn set_value(&mut self, x: usize, y: usize, n: u8) {
        let index = index_from_point(x, y);
        self.grid[index] = n;
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

fn get_bag_of_numbers() -> ShuffleBag<u8> {
    let mut bag = ShuffleBag::new(NUMBERS);
    (0..(NUMBERS - 1)).for_each(|n| {
        bag.add(n as u8, 1);
    });
    bag
}

#[inline]
fn index_from_point(x: usize, y: usize) -> usize {
    debug_assert!(x < COLS || y < COLS, "Point index out of bounds.");
    y * COLS + x
}

#[inline]
fn point_from_index(index: usize) -> (usize, usize) {
    debug_assert!(index < NUMBERS - 1, "Index out of bounds");
    (index.rem(COLS), index / COLS)
}
