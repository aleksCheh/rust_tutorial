use crate::dungeon_crawler::prelude::*;
use super::MapArchitect;


const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const NUM_FLOOR: usize = NUM_TILES /3;

pub struct DrunkardsWalkArchitect {}

const STAGGER_DISTANCE: usize = 400;
impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut CrawlerMap) {
        let mut drunkard_pos = start.clone();
        let mut staggered = 0;

        loop {
            let drunkard_index = map.point2d_to_index(drunkard_pos);
            map.tiles[drunkard_index] = TileType::Floor;

            match rng.range(0,4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }
            
            if !map.in_bounds(drunkard_pos) {
                break;
            }

            staggered += 1;
            if staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }



}
impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> CrawlerMapBuilder {

        let mut map_builder = CrawlerMapBuilder::gen_clean_builder();

        map_builder
    }
}