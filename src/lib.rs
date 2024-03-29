use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::JClass;
use jni::sys::jstring;

use bracket_lib::prelude::*;
//use std::io::{stdin, Read};
mod sprite;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 60.0;

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);

        if self.player.x == self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y >= SCREEN_HEIGHT || self.obstacle.hit_with_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "It is over!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Terminal Test");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
}
struct Obstacle {
    gap_size: i32,
    x: i32,
    gap_y: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_size: i32::max(2, 20 - score),
            gap_y: random.range(10, 40),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.gap_size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    fn hit_with_obstacle(&self, player: &Player) -> bool {
        let half_size = self.gap_size / 2;
        let does_match_x = self.x == player.x + 5;
        let does_match_y =
            player.y < (self.gap_y - half_size) || player.y > (self.gap_y + half_size);
        return does_match_x && does_match_y;
    }
}

struct Player {
    x: i32,
    y: i32,
    vel: f32,
}
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player { x, y, vel: 0.0 }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(5, self.y, BLACK, YELLOW, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.vel < 2.0 {
            self.vel += 0.2;
        }
        self.y += self.vel as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.vel = -2.0;
    }
}
enum GameMode {
    Menu,
    Playing,
    End,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}
#[no_mangle]
#[allow(unused_variables)]
pub extern "system" fn Java_TestInterop_tinTest(env: JNIEnv, o: JClass) -> jstring {
    //sprite::main_run();
    // match  {
    //     Ok(),
    //     Err() => println!("Error"),
    // }

    let context = BTermBuilder::simple80x50()
        .with_title("Term test")
        .build()
        .unwrap();
    main_loop(context, State::new()).unwrap();

    let output = env
        .new_string(format!("Hello,!"))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}
