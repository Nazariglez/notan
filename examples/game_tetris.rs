use notan::draw::*;
use notan::prelude::*;
use std::collections::VecDeque;

const TILE_SIZE: i32 = 30;
const COLS: i32 = 10;
const ROWS: i32 = 18;
const MOVE_DOWN_MS: f32 = 0.5;
const MIN_MOVE_DOWN_MS: f32 = 0.1;
const ACCELERATION_BY_LINE: f32 = 0.02;

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .set_size(500, (TILE_SIZE * ROWS) as _)
        .set_vsync(true);

    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    state.time += app.timer.delta_f32();

    let down = app.keyboard.was_pressed(KeyCode::Down) || app.keyboard.was_pressed(KeyCode::S);
    let up = app.keyboard.was_pressed(KeyCode::Up) || app.keyboard.was_pressed(KeyCode::W);
    let left = app.keyboard.was_pressed(KeyCode::Left) || app.keyboard.was_pressed(KeyCode::A);
    let right = app.keyboard.was_pressed(KeyCode::Right) || app.keyboard.was_pressed(KeyCode::D);

    if down {
        state.move_to(MoveTo::Down);
        state.time = 0.0;
    } else if left {
        state.move_to(MoveTo::Left);
    } else if right {
        state.move_to(MoveTo::Right);
    } else if up {
        state.rotate_to(true);
    }

    if state.can_move() {
        state.time = 0.0;
        state.move_to(MoveTo::Down);
    }

    if !state.drop_lines.is_empty() {
        state.remove_lines_time -= app.timer.delta_f32();
        if state.remove_lines_time <= 0.0 {
            state.remove_lines();
        }
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let tile_size = TILE_SIZE as f32;

    let mut draw = gfx.create_draw();
    draw.clear(Color::new(0.176, 0.176, 0.176, 1.0));

    // draw grid
    for i in 0..COLS * ROWS {
        let (x, y) = xy(i);
        let pos_x = (x * TILE_SIZE) as f32;
        let pos_y = (y * TILE_SIZE) as f32;
        draw.rect((pos_x, pos_y), (tile_size, tile_size))
            .color(Color::WHITE.with_alpha(0.05))
            .stroke(1.0);

        if let Some(tile) = state.grid.get(i as usize) {
            if let Some(s) = tile {
                draw.image(&state.texture)
                    .position(pos_x, pos_y)
                    .color(s.color());
            }
        }
    }

    // draw the current piece
    let total_movement_time = movement_time(state.score_lines);
    let interpolated_y = ((state.time / total_movement_time) * tile_size) - tile_size;
    draw_piece(&mut draw, &state.texture, 0.0, interpolated_y, &state.piece);

    // draw the next piece
    let next_x = (COLS / 2 * TILE_SIZE + TILE_SIZE * 2) as f32;
    let next_y = (TILE_SIZE * 6) as f32;
    draw_piece(&mut draw, &state.texture, next_x, next_y, &state.next);

    let text_x = (COLS * TILE_SIZE + TILE_SIZE) as f32;
    draw.text(&state.font, "NEXT")
        .position(text_x, 10.0)
        .size(30.0);

    draw.text(&state.font, &format!("Score: {}", state.score_lines))
        .position(text_x, next_y + tile_size * 4.0)
        .size(40.0);

    if let Some(lines) = state.last_score {
        draw.text(&state.font, &format!("Last score: {lines}"))
            .position(text_x, next_y * 6.0)
            .size(20.0);
    }

    gfx.render(&draw);
}

fn draw_piece(draw: &mut Draw, img: &Texture, x: f32, y: f32, piece: &Piece) {
    let color = piece.shape.color().with_alpha(0.7);
    piece.points.iter().for_each(|(px, py)| {
        let pos_x = x + (px * TILE_SIZE) as f32;
        let pos_y = y + (py * TILE_SIZE) as f32;
        draw.image(img).position(pos_x, pos_y).color(color);
    });
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Shape {
    I,
    J,
    L,
    O,
    Z,
    T,
    S,
}

impl Shape {
    fn color(&self) -> Color {
        use Shape::*;
        match self {
            I => Color::RED,
            J | L => Color::ORANGE,
            O => Color::YELLOW,
            T => Color::PINK,
            Z | S => Color::GREEN,
        }
    }
    fn pos(&self, x: i32, y: i32) -> [(i32, i32); 4] {
        use Shape::*;
        match self {
            I => [(x, y - 2), (x, y - 1), (x, y), (x, y + 1)],
            J => [(x, y - 1), (x, y), (x, y + 1), (x - 1, y + 1)],
            L => [(x, y - 1), (x, y), (x, y + 1), (x + 1, y + 1)],
            O => [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
            Z => [(x + 1, y), (x, y), (x, y - 1), (x - 1, y - 1)],
            T => [(x - 1, y), (x, y), (x + 1, y), (x, y + 1)],
            S => [(x - 1, y), (x, y), (x, y - 1), (x + 1, y - 1)],
        }
    }
    fn rot(&self, rot: i8, points: &[(i32, i32); 4]) -> [(i32, i32); 4] {
        use Shape::*;
        match self {
            L | J => {
                let (x, y) = points[1];
                match rot {
                    0 => {
                        let j = if *self == J { -1 } else { 1 };
                        [(x, y - 1), (x, y), (x, y + 1), (x + j, y + 1)]
                    }
                    1 => {
                        let j = if *self == J { -1 } else { 1 };
                        [(x + 1, y), (x, y), (x - 1, y), (x - 1, y + j)]
                    }
                    2 => {
                        let j = if *self == J { 1 } else { -1 };
                        [(x, y + 1), (x, y), (x, y - 1), (x + j, y - 1)]
                    }
                    3 => {
                        let j = if *self == J { 1 } else { -1 };
                        [(x - 1, y), (x, y), (x + 1, y), (x + 1, y + j)]
                    }
                    _ => *points,
                }
            }
            S | Z => {
                let (x, y) = points[1];
                let z = if *self == Z { 1 } else { -1 };
                match rot {
                    0 => [(x + z, y), (x, y), (x, y - 1), (x + -z, y - 1)],
                    1 => [(x, y + z), (x, y), (x + 1, y), (x + 1, y + -z)],
                    _ => *points,
                }
            }
            I => {
                let (x, y) = points[2];
                match rot {
                    0 => [(x, y - 2), (x, y - 1), (x, y), (x, y + 1)],
                    1 => [(x + 2, y), (x + 1, y), (x, y), (x - 1, y)],
                    _ => *points,
                }
            }
            T => {
                let (x, y) = points[1];
                match rot {
                    0 => [(x - 1, y), (x, y), (x + 1, y), (x, y + 1)],
                    1 => [(x, y - 1), (x, y), (x, y + 1), (x - 1, y)],
                    2 => [(x + 1, y), (x, y), (x - 1, y), (x, y - 1)],
                    3 => [(x, y + 1), (x, y), (x, y - 1), (x + 1, y)],
                    _ => *points,
                }
            }
            _ => *points,
        }
    }
}

struct Piece {
    shape: Shape,
    points: [(i32, i32); 4],
    rotation: i8,
}

impl Piece {
    fn new(shape: Shape) -> Self {
        let points = shape.pos(COLS / 2, -2);
        Self {
            shape,
            points,
            rotation: 0,
        }
    }

    fn move_points(&mut self, x: i32, y: i32) -> [(i32, i32); 4] {
        let mut new_points = self.points;
        new_points.iter_mut().for_each(|(px, py)| {
            *px += x;
            *py += y;
        });
        new_points
    }

    fn left_points(&mut self) -> [(i32, i32); 4] {
        self.move_points(-1, 0)
    }

    fn right_points(&mut self) -> [(i32, i32); 4] {
        self.move_points(1, 0)
    }

    fn down_points(&mut self) -> [(i32, i32); 4] {
        self.move_points(0, 1)
    }

    fn rotate_points(&mut self, clockwise: bool) -> [(i32, i32); 4] {
        use Shape::*;

        let rot_value = if clockwise { 1 } else { -1 };
        self.rotation = match self.shape {
            O => self.rotation,
            L | J | T => (self.rotation + rot_value) % 4,
            Z | S | I => (self.rotation + rot_value) % 2,
        };
        self.shape.rot(self.rotation, &self.points)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum MoveTo {
    Down,
    Left,
    Right,
}

fn movement_time(lines: i32) -> f32 {
    (MOVE_DOWN_MS - lines as f32 * ACCELERATION_BY_LINE).max(MIN_MOVE_DOWN_MS)
}

#[derive(AppState)]
struct State {
    piece: Piece,
    grid: VecDeque<Option<Shape>>,
    next: Piece,
    drop_lines: Vec<i32>,
    time: f32,
    score_lines: i32,
    last_score: Option<i32>,
    texture: Texture,
    remove_lines_time: f32,
    shape_bag: ShuffleBag<Shape>,
    font: Font,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        use Shape::*;
        let shapes = [I, J, L, O, Z, T, S];
        let mut shape_bag = ShuffleBag::new(7);
        shapes.iter().for_each(|s| shape_bag.add(*s, 1));

        let piece = random_piece(&mut shape_bag);
        let next = random_piece(&mut shape_bag);
        let mut grid = VecDeque::with_capacity((COLS * ROWS) as usize);
        grid.resize(grid.capacity(), None);

        let font = gfx
            .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
            .unwrap();
        let texture = create_texture(gfx);

        Self {
            piece,
            grid,
            next,
            drop_lines: vec![],
            time: 0.0,
            score_lines: 0,
            last_score: None,
            texture,
            remove_lines_time: 0.0,
            shape_bag,
            font,
        }
    }

    fn reset(&mut self) {
        let mut grid = VecDeque::with_capacity((COLS * ROWS) as usize);
        grid.resize(grid.capacity(), None);
        self.grid = grid;
        self.add_shape();
        self.drop_lines = vec![];
        self.time = 0.0;
        self.last_score = Some(self.score_lines);
        self.score_lines = 0;
    }

    fn can_move(&self) -> bool {
        self.time >= movement_time(self.score_lines)
    }

    fn add_shape(&mut self) -> bool {
        let next = std::mem::replace(&mut self.next, random_piece(&mut self.shape_bag));
        self.piece = next;
        true
    }

    fn put_on_grid(&mut self) {
        for (x, y) in self.piece.points.iter() {
            if let Some(tile) = self.grid.get_mut(index(*x, *y)) {
                *tile = Some(self.piece.shape);
            }
        }

        self.add_shape();
        self.check_lines();
        if !self.drop_lines.is_empty() {
            self.remove_lines_time = movement_time(self.score_lines) * 0.9;
        }
    }

    fn remove_lines(&mut self) {
        if self.drop_lines.is_empty() {
            return;
        }

        let mut push_rows = 0;
        while let Some(row) = self.drop_lines.pop() {
            let y = row + push_rows;
            let start = index(0, y);
            let end = index(COLS, y);
            let _ = self.grid.drain(start..end);
            (0..COLS).for_each(|_| self.grid.push_front(None));
            push_rows += 1;
        }

        self.score_lines += push_rows;
    }

    fn move_to(&mut self, dir: MoveTo) {
        let points = match dir {
            MoveTo::Down => self.piece.down_points(),
            MoveTo::Left => self.piece.left_points(),
            MoveTo::Right => self.piece.right_points(),
        };

        if !self.set_points(points) && dir == MoveTo::Down {
            if self.is_out() {
                self.reset();
                return;
            }

            self.put_on_grid();
        }
    }

    fn set_points(&mut self, points: [(i32, i32); 4]) -> bool {
        if self.validate_points(&points) {
            self.piece.points = points;
            true
        } else {
            false
        }
    }

    fn rotate_to(&mut self, clockwise: bool) {
        let points = self.piece.rotate_points(clockwise);
        let _ = self.set_points(points);
    }

    fn validate_points(&self, points: &[(i32, i32); 4]) -> bool {
        for (x, y) in points.iter() {
            if *x < 0 || *x >= COLS || *y >= ROWS {
                return false;
            }

            if *y >= 0 {
                if let Some(opt_val) = self.grid.get(index(*x, *y)) {
                    if opt_val.is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_out(&self) -> bool {
        for (_, y) in self.piece.points.iter() {
            if *y >= 0 {
                return false;
            }
        }

        true
    }

    fn check_lines(&mut self) {
        for y in 0..ROWS {
            let mut drop_line = true;
            for x in 0..COLS {
                if let Some(tile) = self.grid.get(index(x, y)) {
                    if tile.is_none() {
                        drop_line = false;
                        break;
                    }
                }
            }

            if drop_line {
                self.drop_lines.push(y);
            }
        }
    }
}

fn random_piece(bag: &mut ShuffleBag<Shape>) -> Piece {
    Piece::new(*bag.item().unwrap())
}

fn xy(index: i32) -> (i32, i32) {
    (index % COLS, index / COLS)
}

fn index(x: i32, y: i32) -> usize {
    (y * COLS + x) as usize
}

fn create_texture(gfx: &mut Graphics) -> Texture {
    let rt = gfx
        .create_render_texture(TILE_SIZE as _, TILE_SIZE as _)
        .build()
        .unwrap();

    let tile_size = TILE_SIZE as f32;

    let mut draw = gfx.create_draw();
    draw.set_size(tile_size, tile_size);
    draw.clear(Color::TRANSPARENT);
    draw.rect((0.0, 0.0), (tile_size, tile_size))
        .color(Color::WHITE);
    draw.rect((2.0, 2.0), (tile_size - 4.0, tile_size - 4.0))
        .color(Color::BLACK)
        .stroke(4.0);

    let tp = tile_size * 0.3;
    let ts = tile_size * 0.4;
    draw.rect((tp, tp), (ts, ts))
        .color(Color::from_hex(0xc0c0c0ff));
    draw.rect((tp + 1.0, tp + 1.0), (ts - 2.0, ts - 2.0))
        .color(Color::from_hex(0x5a5a5aff))
        .stroke(2.0);

    gfx.render_to(&rt, &draw);
    rt.take_inner()
}
