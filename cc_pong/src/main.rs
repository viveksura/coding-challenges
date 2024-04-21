use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng, Rng};
use nalgebra as na;

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;

const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 300.0;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

fn clamp(val: &mut f32, lower_limit: f32, upper_limit: f32) {
    if *val < lower_limit {
        *val = lower_limit;
    } else if *val > upper_limit {
        *val = upper_limit;
    }
}

fn move_racket(ctx: &mut Context, screen_h: f32, pos: &mut na::Point2<f32>, keycode: KeyCode, direction: f32) {
    if ctx.keyboard.is_key_pressed(keycode) {
        let dt = ctx.time.delta().as_secs_f32();
        pos.y += PLAYER_SPEED * direction * dt;
        clamp(&mut pos.y, RACKET_HEIGHT_HALF, screen_h - RACKET_HEIGHT_HALF);
    }
}

fn randomise_ball_vector(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x
    };

    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y
    };
}


struct MyGame {
    // Your state here...
    screen_h: f32,
    screen_w: f32,
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        // Load/create resources such as images here.
        MyGame {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
            ball_vel: na::Vector2::new(50.0, 16.0),
            screen_h: screen_h,
            screen_w: screen_w,
            player_1_score: 0, 
            player_2_score: 0
        }
    }
}

impl EventHandler for MyGame {
   fn update(&mut self, ctx: &mut Context) -> GameResult {
        move_racket(ctx, self.screen_h, &mut self.player_1_pos, KeyCode::W, -1.0);
        move_racket(ctx, self.screen_h, &mut self.player_1_pos, KeyCode::S, 1.0);
        move_racket(ctx, self.screen_h, &mut self.player_2_pos, KeyCode::Up, -1.0);
        move_racket(ctx, self.screen_h, &mut self.player_2_pos, KeyCode::Down, 1.0);
        let dt = ctx.time.delta().as_secs_f32();
        self.ball_pos += self.ball_vel * dt;

        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = self.screen_w * 0.5;
            self.ball_pos.y = self.screen_h * 0.5;
            randomise_ball_vector(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        }

        if self.ball_pos.x > self.screen_w {
            self.ball_pos.x = self.screen_w * 0.5;
            self.ball_pos.y = self.screen_h * 0.5;
            randomise_ball_vector(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }

        if self.ball_pos.y <= 0.0 || self.ball_pos.y >= self.screen_h {
            self.ball_vel.y *= -0.9;
        }

        if (self.ball_pos.y >= self.player_1_pos.y - RACKET_HEIGHT_HALF && self.ball_pos.y <= self.player_1_pos.y + RACKET_HEIGHT_HALF && self.ball_pos.x <= RACKET_WIDTH_HALF) || 
            (self.ball_pos.y >= self.player_2_pos.y - RACKET_HEIGHT_HALF && self.ball_pos.y <= self.player_2_pos.y + RACKET_HEIGHT_HALF && self.ball_pos.x >= (self.screen_w - RACKET_WIDTH_HALF)) {
            self.ball_vel.x *= -1.0 * rand::thread_rng().gen_range(0.9..1.5);
            self.ball_vel.y *= rand::thread_rng().gen_range(0.9..1.5);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, - RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, Color::WHITE)?;
        canvas.draw(&racket_mesh, graphics::DrawParam::default().dest([self.player_1_pos.x, self.player_1_pos.y]));
        canvas.draw(&racket_mesh, graphics::DrawParam::default().dest([self.player_2_pos.x, self.player_2_pos.y]));

        let ball_rect = graphics::Rect::new(- BALL_SIZE_HALF, - BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, Color::WHITE)?;
        canvas.draw(&ball_mesh, graphics::DrawParam::default().dest([self.ball_pos.x, self.ball_pos.y]));

        let score_text = graphics::Text::new(format!("{}      {}", self.player_1_score, self.player_2_score));
        canvas.draw(&score_text, graphics::DrawParam::default().dest([self.screen_w * 0.5, 20.0]));

        canvas.finish(ctx)
    }
}