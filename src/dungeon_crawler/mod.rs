mod camera;
mod crawler_map;
mod crawler_map_builder;
mod player;
// use crawler_map::CrawlerMap;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use std::io::{stdin, Read};
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    //pub const FRAME_DURATION: f32 = 60.0;
    pub use crate::dungeon_crawler::camera::*;
    pub use crate::dungeon_crawler::crawler_map::*;
    pub use crate::dungeon_crawler::crawler_map_builder::*;
    pub use crate::dungeon_crawler::player::*;
}

use self::prelude::*;

struct State {
    map: CrawlerMap,
    player: Player,
    camera: Camera,
}
impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let crawlerMapBuilder = CrawlerMapBuilder::new(&mut rng);
        State {
            map: crawlerMapBuilder.map,
            player: Player::new(crawlerMapBuilder.player_start),
            camera: Camera::new(crawlerMapBuilder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &mut self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}
pub fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_resource_path("resources/")
        .with_tile_dimensions(32, 32)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
mod test {
    #[test]
    fn run() {
        super::main().unwrap()
    }
}
