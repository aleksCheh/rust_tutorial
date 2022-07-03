use bracket_lib::prelude::*;
use std::io::{stdin, Read};



struct State {
    x: i32
}
impl State {
    fn new() -> State {
        State {
            x: 0,
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm){

    }
}


pub fn main_run() -> BError {
    
    let sp = SpriteSheet::new("resources/goblin.png").add_sprite(Rect::with_size(0,0,65,65));
    let bt = BTermBuilder::new().with_sprite_console(640, 480, 1).with_sprite_sheet(sp).with_title("Sprite test").build()?;
    main_loop(bt, State::new())
}
pub fn t_out(){
    println!("Function from sprite.rs")
}