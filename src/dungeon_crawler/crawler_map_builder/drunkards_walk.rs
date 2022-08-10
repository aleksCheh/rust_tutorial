use super::MapArchitect;
use crate::dungeon_crawler::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const NUM_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

const STAGGER_DISTANCE: usize = 400;
impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut CrawlerMap) {
        let mut drunkard_pos = start.clone();
        let mut staggered = 0;

        loop {
            let drunkard_index = map.point2d_to_index(drunkard_pos);
            map.tiles[drunkard_index] = TileType::Floor;

            match rng.range(0, 4) {
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
        println!("Drunkards Walk new");
        let mut map_builder = CrawlerMapBuilder::gen_clean_builder();

        map_builder.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut map_builder.map);

        while map_builder
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < NUM_FLOOR
        {
            self.drunkard(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut map_builder.map,
            );
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![map_builder.map.point2d_to_index(center)],
                &map_builder.map,
                1024.0,
            );
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance < &2000.0)
                .for_each(|(idx, _)| map_builder.map.tiles[idx] = TileType::Floor);
        }

        map_builder.monster_spawn = map_builder.spawn_monsters(&center, rng);
        map_builder.player_start = center;
        map_builder.amulet_start = map_builder.find_most_distant();

        map_builder
    }
}
