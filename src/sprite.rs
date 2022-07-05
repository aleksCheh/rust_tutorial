use bracket_lib::prelude::*;
use std::env::*;
use std::io::{stdin, Read};
use std::vec::*;

use bracket_random::prelude::*;
use bracket_terminal::prelude::*;

const FRAME_FREQ_MS: f32 = 290.0;
struct Character {
    x: i32,
    y: i32,
    animFrames: Vec<Rect>,
}

impl Character {
    fn new() -> Character {
        Character {
            x: 0,
            y: 0,
            animFrames: Vec::<Rect>::new(),
        }
    }
}
struct State {
    x: i32,
    frame_time: f32,
    goblin: Character,
    frame_counter: i64,
}
impl State {
    fn new() -> State {
        State {
            x: 0,
            goblin: Character::new(),
            frame_time: 0.0,
            frame_counter: 0
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
        // if self.frame_time > FRAME_FREQ_MS {
        //     self.frame_time = 0.0;
        //     self.frame_counter += 1;
        // }
        
        ctx.set_active_console(0);        
        let ind: usize = (self.frame_counter % 2).try_into().unwrap();
        
        ctx.add_sprite(
            Rect::with_size(0, 0, 90, 90),
            3,
            RGBA::from_f32(1.0, 1.0, 1.0, 0.5),
            ind + 1 ,
        );

        ctx.set_active_console(1);
        ctx.add_sprite(Rect::with_size(0, 0, 640, 480),
        0,
        RGBA::from_f32(1.0, 1.0, 1.0, 0.5),
        0 ,);

        
        
    }
}


pub fn main_run() -> BError {
    let sp = SpriteSheet::new("resources/goblin.png")
        .add_sprite(Rect::with_size(0, 0, 65, 65))
        .add_sprite(Rect::with_size(65, 0, 65, 65))
        .add_sprite(Rect::with_size(130, 0, 65, 65))
        .add_sprite(Rect::with_size(195, 0, 65, 65));
    let bg = SpriteSheet::new("resources/back.png")
        .add_sprite(Rect::with_size(0, 0, 640, 480));
    let mut bt = BTermBuilder::new()
        .with_sprite_sheet(sp)
        .with_sprite_sheet(bg)
        .with_sprite_console(640, 480, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_sprite_console(640, 480, 1)        
        .with_title("Sprite test")
        .build()?;
    

    main_loop(bt, State::new())
}

pub fn t_out() {
    println!("Function from sprite.rs")
}
