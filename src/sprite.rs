use bracket_lib::prelude::*;
use std::env::*;
use std::io::{stdin, Read};
use std::vec::*;


use bracket_random::prelude::*;
use bracket_terminal::prelude::*;

const FRAME_FREQ_MS: f32 = 60.0;
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
    
}
impl State {
    fn new() -> State {
        State {
            x: 0,
            goblin: Character::new(),
            frame_time: 0.0,
            }
    }
    fn setCharacterPos(&mut self,x:i32, y:i32){
        self.goblin.x = x;
        self.goblin.y = y;
    }
}
impl GameState for State {  

    

    fn tick(&mut self, ctx: &mut BTerm) {

        self.frame_time = ctx.frame_time_ms;
        if self.frame_time > FRAME_FREQ_MS {
            self.frame_time = 0.0;
        }

        ctx.set_active_console(0);
        ctx.cls();
        ctx.add_sprite(
            Rect::with_size(0, 0, 65, 65),
            1,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            0,
        );
        ctx.add_sprite(
            Rect::with_size(65, 65, 65, 65),
            1,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            1,
        );
        ctx.add_sprite(
            Rect::with_size(130, 0, 65, 65),
            1,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            2,
        );
        ctx.add_sprite(
            Rect::with_size(195, 65, 65, 65),
            1,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            3,
        );
    }
}

bracket_terminal::embedded_resource!(NYAN_CAT, "../resources/goblin.png");
pub fn main_run() -> BError {
    bracket_terminal::link_resource!(NYAN_CAT, "resources/goblin.png");

    //std::env::set_var("RUST_BACKTRACE", "1");
    let sp = SpriteSheet::new("resources/goblin.png")
    .add_sprite(Rect::with_size(0, 0, 65, 65))
    .add_sprite(Rect::with_size(65,0 , 65, 65))
    .add_sprite(Rect::with_size(130, 0, 65, 65))
    .add_sprite(Rect::with_size(195, 0, 65, 65));
    let bt = BTermBuilder::new()
        .with_sprite_console(640, 480, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console_no_bg(80, 50, "terminal8x8.png")
        .with_sprite_sheet(sp)
        .with_title("Sprite test")
        .build()?;
    main_loop(bt, State::new())
}

pub fn t_out() {
    println!("Function from sprite.rs")
}
