
use crate::dungeon_crawler::prelude::*;
use super::MapArchitect;
pub struct CellularAutomataBuilder{}

impl CellularAutomataBuilder {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut CrawlerMap) {                
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {*t = TileType::Wall;}
            else {*t = TileType::Floor;}
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &CrawlerMap) -> i32 {
        let mut neighbors = 0;
        for dx in -1 .. 1 {
            for dy in -1 .. 1 {
                if !(dx == 0 && dy == 0) && map.tiles[map_idx(x+dx, y+dy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        println!("Neighbors count: {}", neighbors);
        neighbors
    }
    fn iteration (&mut self, map: &mut CrawlerMap) {
        let mut new_tiles = map.tiles.clone();
        for x in 1 .. SCREEN_WIDTH -1 {
            for y in 1 .. SCREEN_HEIGHT - 1 {
                let n = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                //live - wall
                //dies - floor
                if ((n == 2 || n == 3) && new_tiles[idx] == TileType::Wall) 
                    || (n > 3 && new_tiles[idx] == TileType::Floor) {

                    new_tiles[idx] = TileType::Wall;
                    
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&mut self, map: &CrawlerMap) -> Point {
        let center = Point::new(SCREEN_WIDTH /2 , SCREEN_HEIGHT / 2);
        let closest_tiles =map.tiles.iter().enumerate().filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx))))
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())
            .map(|(idx, _)| idx).unwrap();
        map.index_to_point2d(closest_tiles)
    }

   
}

impl MapArchitect for CellularAutomataBuilder {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> CrawlerMapBuilder {
        println!("Cellular Automata Creation");
        let mut map_builder = CrawlerMapBuilder::gen_clean_builder();
        self.random_noise_map(rng, &mut map_builder.map);
        for _ in 0..2 {
            self.iteration(&mut map_builder.map);
        }
        let start = self.find_start(&map_builder.map);
        map_builder.monster_spawn = map_builder.spawn_monsters(&start, rng);
        map_builder.player_start = start;
        map_builder.amulet_start = map_builder.find_most_distant();

        map_builder

    }
}