use notan::draw::*;
use notan::prelude::*;

const TILE_SIZE: usize = 40;
const COLS: usize = 800 / TILE_SIZE;
const ROWS: usize = 600 / TILE_SIZE;
const BACKGROUND_COLOR: Color = Color::new(0.698, 0.792, 0.376, 1.0);
const SNAKE_COLOR: Color = Color::new(0.211, 0.298, 0.074, 1.0);
const LINE_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.1);
const MOVEMENT_MS: f32 = 0.2;
const MIN_MOVEMENT_MS: f32 = 0.02;

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new().set_size(800, 600).set_vsync(true);

    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    state.time += app.timer.delta_f32();

    change_direction(&app.keyboard, state);

    if state.can_move() {
        state.movement();
        state.time = 0.0;
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let tile_size = TILE_SIZE as f32;

    let mut draw = gfx.create_draw();
    draw.clear(BACKGROUND_COLOR);

    // draw grid
    for i in 0..COLS * ROWS {
        let (x, y) = xy(i);
        let pos_x = (x * TILE_SIZE) as f32;
        let pos_y = (y * TILE_SIZE) as f32;
        draw.rect((pos_x, pos_y), (tile_size, tile_size))
            .color(LINE_COLOR)
            .stroke(1.0);
    }

    // how to play
    draw.text(&state.font, "Use WASD to move")
        .position(10.0, 570.0)
        .size(20.0);

    // draw food
    let pos_x = (state.food.0 * TILE_SIZE) as f32;
    let pos_y = (state.food.1 * TILE_SIZE) as f32;
    draw.rect((pos_x, pos_y), (tile_size, tile_size))
        .color(Color::from_hex(0xa0a0a0ff));
    draw.rect((pos_x, pos_y), (tile_size, tile_size))
        .color(Color::BLACK)
        .stroke(2.0);

    // draw snake
    state.snake.iter().enumerate().for_each(|(i, (x, y))| {
        let pos_x = (x * TILE_SIZE) as f32;
        let pos_y = (y * TILE_SIZE) as f32;
        let color = if i == state.snake.len() - 1 {
            SNAKE_COLOR
        } else {
            SNAKE_COLOR.with_alpha(0.8)
        };

        draw.rect((pos_x, pos_y), (tile_size, tile_size))
            .color(color);

        draw.rect((pos_x, pos_y), (tile_size, tile_size))
            .color(Color::BLACK)
            .stroke(1.0);
    });

    draw.text(&state.font, &format!("Score: {}", state.score))
        .position(10.0, 10.0)
        .size(30.0);

    if let Some(last_score) = state.last_score {
        draw.text(&state.font, &format!("Last Score: {last_score}"))
            .position(10.0, 50.0)
            .size(20.0);
    }

    gfx.render(&draw);
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(AppState)]
struct State {
    snake: Vec<(usize, usize)>,
    food: (usize, usize),
    rng: Random,
    dir: Direction,
    time: f32,
    score: i32,
    last_score: Option<i32>,
    accel: f32,
    font: Font,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let mut rng = Random::default();
        let food = random_xy(&mut rng);
        let font = gfx
            .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
            .unwrap();
        Self {
            time: 0.0,
            rng,
            food,
            snake: vec![(8, 7), (9, 7), (10, 7)],
            dir: Direction::Right,
            score: 0,
            last_score: None,
            accel: 0.0,
            font,
        }
    }

    fn hit(&self, x: usize, y: usize, only_body: bool) -> bool {
        let range = if only_body {
            0..self.snake.len() - 1
        } else {
            0..self.snake.len()
        };

        let mut exists_hit = false;
        for i in range {
            let (tile_x, tile_y) = self.snake[i];
            if x == tile_x && y == tile_y {
                exists_hit = true;
                break;
            }
        }
        exists_hit
    }

    fn reset(&mut self) {
        self.time = 0.0;
        self.snake = vec![(8, 7), (9, 7), (10, 7)];
        self.dir = Direction::Right;
        self.food = random_xy(&mut self.rng);
        self.last_score = Some(self.score);
        self.score = 0;
        self.accel = 0.0;
    }

    fn set_food_pos(&mut self) {
        loop {
            let (fx, fy) = random_xy(&mut self.rng);
            if !self.hit(fx, fy, false) {
                self.food = (fx, fy);
                break;
            }
        }
    }

    fn can_move(&self) -> bool {
        self.time >= (MOVEMENT_MS - self.accel).max(MIN_MOVEMENT_MS)
    }

    fn movement(&mut self) {
        if let Some((mut x, mut y)) = self.snake.last() {
            if self.hit(x, y, true) {
                self.reset();
                return;
            }

            match self.dir {
                Direction::Left => {
                    x = (COLS + x - 1) % COLS;
                }
                Direction::Right => {
                    x = (COLS + x + 1) % COLS;
                }
                Direction::Up => {
                    y = (ROWS + y - 1) % ROWS;
                }
                Direction::Down => {
                    y = (ROWS + y + 1) % ROWS;
                }
            }

            self.snake.push((x, y));

            if x == self.food.0 && y == self.food.1 {
                self.score += 1;
                self.accel += 0.01;
                self.set_food_pos();
            } else {
                self.snake.remove(0);
            }
        }
    }
}

fn random_xy(rng: &mut Random) -> (usize, usize) {
    (rng.gen_range(0..COLS), rng.gen_range(0..ROWS))
}

fn xy(index: usize) -> (usize, usize) {
    (index % COLS, index / COLS)
}

fn change_direction(keyboard: &Keyboard, state: &mut State) {
    let up = keyboard.was_pressed(KeyCode::W) || keyboard.was_pressed(KeyCode::Up);
    let down = keyboard.was_pressed(KeyCode::S) || keyboard.was_pressed(KeyCode::Down);
    let left = keyboard.was_pressed(KeyCode::A) || keyboard.was_pressed(KeyCode::Left);
    let right = keyboard.was_pressed(KeyCode::D) || keyboard.was_pressed(KeyCode::Right);

    if up && state.dir != Direction::Down {
        state.dir = Direction::Up;
    } else if down && state.dir != Direction::Up {
        state.dir = Direction::Down;
    } else if left && state.dir != Direction::Right {
        state.dir = Direction::Left;
    } else if right && state.dir != Direction::Left {
        state.dir = Direction::Right;
    }
}
