use bracket_lib::prelude::*;
use std::env::*;
use std::io::{stdin, Read};
use std::vec::*;

use bracket_random::prelude::*;
use bracket_terminal::prelude::*;

const FRAME_FREQ_MS: f32 = 290.0;

#[derive(Copy, Clone)]
struct CharSprite {
    pos: Rect,
    global_index: usize,
}

struct Character {
    x: i32,
    y: i32,
    current_frame: usize,
    anim_freq: f32,
    ms_elapsed: f32,
    sprites: Vec<CharSprite>,
}

impl Character {
    fn new(xn: i32, yn: i32, v: Vec<CharSprite>) -> Character {
        Character {
            x: xn,
            y: yn,
            current_frame: 0,
            anim_freq: 500.0,
            ms_elapsed: 0.0,
            sprites: v, // Vec::<CharSprite>::new(),
        }
    }

    fn new_blank() -> Character {
        Character {
            x: 0,
            y: 0,
            current_frame: 0,
            anim_freq: 500.0,
            ms_elapsed: 0.0,
            sprites: Vec::<CharSprite>::new(),
        }
    }

    fn render(&mut self, context: &mut bracket_lib::prelude::BTerm) {
        context.cls();
        self.ms_elapsed += context.frame_time_ms;
        if self.ms_elapsed >= self.anim_freq {
            self.current_frame += 1;
            self.ms_elapsed = 0.0;
        }
        if self.current_frame >= self.sprites.len() {
            self.current_frame = 0;
        }

        context.add_sprite(
            Rect::with_size(
                self.x,
                self.y,
                self.sprites[self.current_frame].pos.width(),
                self.sprites[self.current_frame].pos.height(),
            ),
            1,
            RGBA {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            self.sprites[self.current_frame].global_index,
        )
    }
}
struct State {
    x: i32,
    frame_time: f32,
    goblin: Character,
    frame_counter: i64,
    bg_chars: Vec<Character>,
}
impl State {
    fn new() -> State {
        State {
            x: 0,
            goblin: Character::new_blank(),
            frame_time: 0.0,
            frame_counter: 0,
            bg_chars: Vec::<Character>::new(),
        }
    }
    fn setCharacterPos(&mut self, x: i32, y: i32) {
        self.goblin.x = x;
        self.goblin.y = y;
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut ind = 0;
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > 1000.0 {
            println!("Frame time ms: {}", ctx.frame_time_ms);
            self.frame_time = 0.0;
        }
        for bc in self.bg_chars.iter_mut() {
            bc.render(ctx);
        }
    }
}

pub fn main_run() -> BError {
    let sp = SpriteSheet::new("resources/goblin.png")
        .add_sprite(Rect::with_size(0, 0, 65, 65))
        .add_sprite(Rect::with_size(65, 0, 65, 65))
        .add_sprite(Rect::with_size(130, 0, 65, 65))
        .add_sprite(Rect::with_size(195, 0, 65, 65));
    let bg = SpriteSheet::new("resources/back.png").add_sprite(Rect::with_size(0, 0, 640, 480));
    let mut bt = BTermBuilder::new()
        .with_sprite_sheet(sp)
        .with_sprite_sheet(bg)
        .with_sprite_console(640, 480, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_sprite_console(640, 480, 1)
        .with_title("Sprite test")
        .build()?;
    let v = vec![
        CharSprite {
            pos: Rect::with_size(0, 0, 64, 64),
            global_index: 1,
        },
        CharSprite {
            pos: Rect::with_size(0, 0, 64, 64),
            global_index: 2,
        },
    ];
    let mut curState = State::new();
    curState.bg_chars.push(Character::new(10, 10, v.clone()));
    curState.bg_chars.push(Character::new(70, 10, v));

    main_loop(bt, curState)
}

pub fn t_out() {
    println!("Function from sprite.rs")
}
