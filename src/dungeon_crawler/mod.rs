mod crawler_map;
mod player;
mod crawler_map_builder;
// use crawler_map::CrawlerMap;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use std::io::{stdin, Read};
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    //pub const FRAME_DURATION: f32 = 60.0;
    pub use crate::dungeon_crawler::crawler_map::*;
    pub use crate::dungeon_crawler::player::*;
}

use self::prelude::*;

struct State {
    map: CrawlerMap,
    player: Player,
}
impl State {
    fn new() -> Self {
        State {
            map: CrawlerMap::new(),
            player: Player::new(Point{x: 10, y: 10}),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &mut self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

pub fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
mod test
{
    #[test]
    fn run() {
        super::main().unwrap()
    }
}