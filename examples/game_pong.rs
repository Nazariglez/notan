use notan::draw::*;
use notan::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 580;
const WALL_SIZE: f32 = 20.0;
const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_HEIGHT: f32 = PADDLE_WIDTH * 4.0;
const PADDLE_SPEED: f32 = 200.0;
const BALL_SIZE: f32 = WALL_SIZE * 0.8;
const BALL_SPEED: f32 = 240.0;
const BALL_SPEED_THRESHOLD: f32 = 60.0;
const FIRE_ANGLE_MAX: f32 = 120.0;
const PI: f32 = std::f32::consts::PI;

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new().size(WIDTH, HEIGHT).vsync();

    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    //Init game with space
    if !state.already_started && app.keyboard.was_pressed(KeyCode::Space) {
        state.already_started = true;
        state.ball.fire(&mut state.rng, true);
    }

    //Move paddle1 with W S
    if app.keyboard.is_down(KeyCode::W) {
        state.paddle_1.y = (state.paddle_1.y - PADDLE_SPEED * app.timer.delta_f32()).max(WALL_SIZE);
    } else if app.keyboard.is_down(KeyCode::S) {
        state.paddle_1.y = (state.paddle_1.y + PADDLE_SPEED * app.timer.delta_f32())
            .min(HEIGHT as f32 - WALL_SIZE - PADDLE_HEIGHT);
    }

    //Move paddle2 with arrows UP DOWN
    if app.keyboard.is_down(KeyCode::Up) {
        state.paddle_2.y = (state.paddle_2.y - PADDLE_SPEED * app.timer.delta_f32()).max(WALL_SIZE);
    } else if app.keyboard.is_down(KeyCode::Down) {
        state.paddle_2.y = (state.paddle_2.y + PADDLE_SPEED * app.timer.delta_f32())
            .min(HEIGHT as f32 - WALL_SIZE - PADDLE_HEIGHT);
    }

    //Move ball
    state.ball.x += state.ball.speed_x * app.timer.delta_f32();
    state.ball.y += state.ball.speed_y * app.timer.delta_f32();

    //Manage collision against walls
    let ball_bounds = state.ball.bounds();
    if ball_bounds.min_y <= WALL_SIZE {
        state.ball.y = WALL_SIZE + 1.0;
        state.ball.speed_y *= -1.0
    } else if ball_bounds.max_y >= HEIGHT as f32 - WALL_SIZE {
        state.ball.y = HEIGHT as f32 - WALL_SIZE - 1.0 - BALL_SIZE;
        state.ball.speed_y *= -1.0;
    }

    //Manage collisions against paddles
    if check_intersection(&ball_bounds, state) {
        if state.ball.x < WIDTH as f32 * 0.5 {
            //Left paddle
            let paddle_bounds = state.paddle_1.bounds();
            state.ball.x = state.paddle_1.x + PADDLE_WIDTH + 1.0;
            let angle = ((ball_bounds.center_y - paddle_bounds.center_y) / PADDLE_HEIGHT + 0.5)
                * FIRE_ANGLE_MAX;
            state
                .ball
                .speed_from_angle(random_speed(&mut state.rng), angle, true);
        } else {
            //Right paddle
            let paddle_bounds = state.paddle_2.bounds();
            state.ball.x = state.paddle_2.x - 1.0 - BALL_SIZE;
            let angle = ((ball_bounds.center_y - paddle_bounds.center_y) / PADDLE_HEIGHT + 0.5)
                * FIRE_ANGLE_MAX;
            state
                .ball
                .speed_from_angle(random_speed(&mut state.rng), angle, false);
            state.ball.speed_y *= -1.0;
        }
    }

    //Goals
    let ball_bounds = state.ball.bounds();
    if ball_bounds.max_x <= 0.0 {
        state.scores.1 += 1;
        state.ball.fire(&mut state.rng, false);
    } else if ball_bounds.min_x >= WIDTH as f32 {
        state.scores.0 += 1;
        state.ball.fire(&mut state.rng, true);
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let alpha = if state.already_started { 1.0 } else { 0.6 };
    let width = WIDTH as f32;
    let height = HEIGHT as f32;

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.set_alpha(alpha);

    draw.rect((0.0, 0.0), (width, WALL_SIZE));
    draw.rect((0.0, height - WALL_SIZE), (width, WALL_SIZE));

    let points = HEIGHT / WALL_SIZE as i32;
    for i in (0..points).step_by(2) {
        draw.rect(
            (width * 0.5 - WALL_SIZE * 0.5, WALL_SIZE * i as f32),
            (WALL_SIZE, WALL_SIZE),
        );
    }

    draw.rect(state.paddle_1.position(), (PADDLE_WIDTH, PADDLE_HEIGHT));
    draw.rect(state.paddle_2.position(), (PADDLE_WIDTH, PADDLE_HEIGHT));
    draw.rect(state.ball.position(), (BALL_SIZE, BALL_SIZE));

    draw.text(&state.font, "Use W/S to move")
        .h_align_center()
        .v_align_middle()
        .position(width * 0.25, height - 100.0)
        .size(20.0);

    draw.text(&state.font, "Use Up/Down to move")
        .h_align_center()
        .v_align_middle()
        .position(width - width * 0.25, height - 100.0)
        .size(20.0);

    if !state.already_started {
        draw.set_alpha(1.0);

        draw.text(&state.font, "Press SPACE to start")
            .h_align_center()
            .v_align_middle()
            .position(width * 0.5, height * 0.5)
            .size(80.0);
    }

    gfx.render(&draw);
}

#[derive(AppState)]

struct State {
    paddle_1: Paddle,
    paddle_2: Paddle,
    ball: Ball,
    scores: (i32, i32),
    rng: Random,
    already_started: bool,
    font: Font,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let font = gfx
            .create_font(include_bytes!("assets/kenney_pixel-webfont.ttf"))
            .unwrap();
        Self {
            paddle_1: Paddle::new(10.0, HEIGHT as f32 * 0.5 - PADDLE_HEIGHT * 0.5),
            paddle_2: Paddle::new(
                WIDTH as f32 - 10.0 - PADDLE_WIDTH,
                HEIGHT as f32 * 0.5 - PADDLE_HEIGHT * 0.5,
            ),
            ball: Ball::new(
                WIDTH as f32 * 0.5 - BALL_SIZE * 0.5,
                HEIGHT as f32 * 0.5 - BALL_SIZE * 0.5,
            ),
            scores: (0, 0),
            rng: Random::default(),
            already_started: false,
            font,
        }
    }
}

struct Paddle {
    x: f32,
    y: f32,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Self {
        Paddle { x, y }
    }
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl BoundCalc for Paddle {
    fn bounds(&self) -> Bounds {
        Bounds {
            min_x: self.x,
            min_y: self.y,
            max_x: self.x + PADDLE_WIDTH,
            max_y: self.y + PADDLE_HEIGHT,
            center_y: self.y + PADDLE_HEIGHT * 0.5,
        }
    }
}

fn random_speed(rng: &mut Random) -> f32 {
    rng.gen_range((BALL_SPEED - BALL_SPEED_THRESHOLD)..(BALL_SPEED + BALL_SPEED_THRESHOLD))
}

struct Ball {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

impl Ball {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed_x: 0.0,
            speed_y: 0.0,
        }
    }

    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn fire(&mut self, rng: &mut Random, left: bool) {
        self.x = if left { 50.0 } else { WIDTH as f32 - 50.0 };
        self.y = rng.gen_range(WALL_SIZE..(HEIGHT as f32 - WALL_SIZE));

        let angle_to_fire: f32 = rng.gen_range(0.0..FIRE_ANGLE_MAX);
        self.speed_from_angle(random_speed(rng), angle_to_fire, left);
    }

    fn speed_from_angle(&mut self, speed: f32, angle_to_fire: f32, left: bool) {
        let angle = if left {
            angle_to_fire - FIRE_ANGLE_MAX * 0.5
        } else {
            180.0 - FIRE_ANGLE_MAX * 0.5 + angle_to_fire
        };
        let rad = angle * PI / 180.0;
        self.speed_x = rad.cos() * speed;
        self.speed_y = rad.sin() * speed;
    }
}

impl BoundCalc for Ball {
    fn bounds(&self) -> Bounds {
        Bounds {
            min_x: self.x,
            min_y: self.y,
            max_x: self.x + BALL_SIZE,
            max_y: self.y + BALL_SIZE,
            center_y: self.y + BALL_SIZE * 0.5,
        }
    }
}

struct Bounds {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    center_y: f32,
}

trait BoundCalc {
    fn bounds(&self) -> Bounds;
}

#[inline]
fn check_intersection(ball_bounds: &Bounds, state: &mut State) -> bool {
    intersect_between(&ball_bounds, &state.paddle_1.bounds())
        || intersect_between(&ball_bounds, &state.paddle_2.bounds())
}

#[inline]
fn intersect_between(a: &Bounds, b: &Bounds) -> bool {
    a.min_x < b.max_x && a.max_x > b.min_x && a.min_y < b.max_y && a.max_y > b.min_y
}
