use nae::extras::Random;
use nae::prelude::*;
use std::collections::VecDeque;

const TILE_SIZE: i32 = 30;
const COLS: i32 = 10;
const ROWS: i32 = 18;
const MOVE_DOWN_MS: f32 = 0.5;
const MIN_MOVE_DOWN_MS: f32 = 0.1;
const ACCELERATION_BY_LINE: f32 = 0.02;

#[nae::main]
fn main() {
    nae::init_with(init)
        .update(update)
        .draw(draw)
        .build()
        .unwrap();
}

fn init(app: &mut App) -> State {
    State::new(create_texture(app))
}

fn update(app: &mut App, state: &mut State) {
    state.time += app.delta;

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
}

fn draw(app: &mut App, state: &mut State) {
    let tile_size = TILE_SIZE as f32;

    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.176, 0.176, 0.176, 1.0));

    // draw grid
    for i in 0..COLS * ROWS {
        let (x, y) = xy(i);
        let pos_x = (x * TILE_SIZE) as f32;
        let pos_y = (y * TILE_SIZE) as f32;
        draw.set_color(Color::WHITE.with_alpha(0.05));
        draw.stroke_rect(pos_x, pos_y, tile_size, tile_size, 1.0);

        if let Some(tile) = state.grid.get(i as usize) {
            if let Some(s) = tile {
                draw.set_color(s.color());
                draw.image(&state.texture, pos_x, pos_y);
            }
        }
    }

    // draw the current piece
    let total_movement_time = movement_time(state.score_lines);
    let interpolated_y = ((state.time / total_movement_time) * tile_size) - tile_size;
    draw_piece(draw, &state.texture, 0.0, interpolated_y, &state.piece);

    // draw the next piece
    let next_x = (COLS / 2 * TILE_SIZE + TILE_SIZE * 2) as f32;
    let next_y = (TILE_SIZE * 6) as f32;
    draw_piece(draw, &state.texture, next_x, next_y, &state.next);

    let text_x = (COLS * TILE_SIZE + TILE_SIZE) as f32;
    draw.set_color(Color::WHITE);
    draw.text("NEXT", text_x, 10.0, 30.0);

    draw.text(
        &format!("Score: {}", state.score_lines),
        text_x,
        next_y + tile_size * 4.0,
        40.0,
    );

    if let Some(lines) = state.last_score {
        draw.text(
            &format!("Last score: {}", lines),
            text_x,
            next_y + tile_size * 6.0,
            20.0,
        );
    }

    draw.end();
}

fn draw_piece(draw: &mut Context2d, img: &Texture, x: f32, y: f32, piece: &Piece) {
    let color = piece.shape.color().with_alpha(0.7);
    piece.points.iter().for_each(|(px, py)| {
        let pos_x = x + (px * TILE_SIZE) as f32;
        let pos_y = y + (py * TILE_SIZE) as f32;
        draw.set_color(color);
        draw.image(img, pos_x, pos_y);
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
                    0 => [(x + z, y), (x, y), (x, y - 1), (x + z * -1, y - 1)],
                    1 => [(x, y + z), (x, y), (x + 1, y), (x + 1, y + z * -1)],
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
        let mut new_points = self.points.clone();
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

struct State {
    rng: Random,
    piece: Piece,
    grid: VecDeque<Option<Shape>>,
    next: Piece,
    drop_lines: Vec<i32>,
    time: f32,
    score_lines: i32,
    last_score: Option<i32>,
    texture: Texture,
}

impl State {
    fn new(texture: Texture) -> Self {
        let mut rng = Random::default();
        let piece = random_piece(&mut rng);
        let next = random_piece(&mut rng);
        let mut grid = VecDeque::with_capacity((COLS * ROWS) as usize);
        grid.resize(grid.capacity(), None);
        Self {
            rng,
            piece,
            grid,
            next,
            drop_lines: vec![],
            time: 0.0,
            score_lines: 0,
            last_score: None,
            texture,
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
        let next = std::mem::replace(&mut self.next, random_piece(&mut self.rng));
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
        self.remove_lines();
    }

    fn remove_lines(&mut self) {
        if self.drop_lines.len() == 0 {
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

        if !self.set_points(points) {
            if dir == MoveTo::Down {
                if self.is_out() {
                    self.reset();
                    return;
                }

                self.put_on_grid();
            }
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

fn random_piece(rng: &mut Random) -> Piece {
    use Shape::*;
    let shapes = [I, J, L, O, Z, T, S];
    let shape = shapes[rng.gen_range(0, shapes.len())];
    Piece::new(shape)
}

fn xy(index: i32) -> (i32, i32) {
    (index % COLS, index / COLS)
}

fn index(x: i32, y: i32) -> usize {
    (y * COLS + x) as usize
}

fn create_texture(app: &mut App) -> Texture {
    let tile_size = TILE_SIZE as f32;
    let surface = Surface::from_size(app, TILE_SIZE, TILE_SIZE).unwrap();

    let draw = app.draw();
    draw.begin_to_surface(Some(&surface));
    draw.set_color(Color::WHITE);
    draw.rect(0.0, 0.0, tile_size, tile_size);
    draw.set_color(Color::BLACK);
    draw.stroke_rect(2.0, 2.0, tile_size - 4.0, tile_size - 4.0, 4.0);
    draw.set_color(hex(0xc0c0c0ff));
    draw.rect(
        tile_size * 0.3,
        tile_size * 0.3,
        tile_size * 0.4,
        tile_size * 0.4,
    );
    draw.set_color(hex(0x5a5a5ff));
    draw.stroke_rect(
        tile_size * 0.3 + 1.0,
        tile_size * 0.3 + 1.0,
        tile_size * 0.4 - 2.0,
        tile_size * 0.4 - 2.0,
        2.0,
    );
    draw.end();

    surface.texture().clone()
}
