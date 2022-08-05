use super::MapArchitect;
use crate::dungeon_crawler::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> CrawlerMapBuilder {
        let mut mb = CrawlerMapBuilder::gen_clean_builder();
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();

        for _ in 0..50 {
            mb.monster_spawn.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ));
        }
        return mb;
    }
}
