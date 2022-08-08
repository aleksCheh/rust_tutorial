pub mod cellular_automata;
pub mod drunkards_walk;
mod room_architect;
pub mod empty;

use crate::dungeon_crawler::prelude::*;
use cellular_automata::*;
use drunkards_walk::*;
use room_architect::*;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> CrawlerMapBuilder;
}
pub struct CrawlerMapBuilder {
    pub map: CrawlerMap,
    pub rooms: Vec<Rect>,
    pub monster_spawn: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}
impl CrawlerMapBuilder {
    
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| {
            *t = tile;
        });
    }


    pub fn new(rng: &mut RandomNumberGenerator) -> Self {

            // let mut architect: Box<dyn MapArchitect> = match rng.range(0,3) {
            //     0 => Box::new(DrunkardsWalkArchitect{}),
            //     1 => Box::new(RoomArchitect{}),
            //     _ => Box::new(CellularAutomataBuilder{}),
            // };
            
            // let mut mb =  architect.new(rng);
            let mut mb = CellularAutomataBuilder{}; 
            mb.new(rng)
        }
    

    pub fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;

        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }
    pub fn gen_clean_builder() -> CrawlerMapBuilder {
        CrawlerMapBuilder {
            map: CrawlerMap::new(),
            rooms: Vec::new(),
            monster_spawn: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        }
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
}

