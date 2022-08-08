
use crate::dungeon_crawler::prelude::*;
use super::MapArchitect;
pub struct RoomArchitect {}

const NUM_ROOMS: usize = 20;

impl RoomArchitect {

    fn build_randorm_rooms(&mut self, rng: &mut RandomNumberGenerator, map: &mut CrawlerMap) -> Vec<Rect>{
        let mut rooms: Vec<Rect> = Vec::new();
        while rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlaps: bool = false;
            for r in &rooms {
                if r.intersect(&room) {
                    overlaps = true;
                }
            }
            if !overlaps {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        map.tiles[idx] = TileType::Floor;
                    }
                });
                rooms.push(room);
            }            
        }
        rooms
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32, map: &mut CrawlerMap) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = map.try_idx(Point::new(x, y)) {
                map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32, map: &mut CrawlerMap) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = map.try_idx(Point::new(x, y)) {
                map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator, rms: &Vec<Rect>, map: &mut CrawlerMap) {
        let mut rooms = rms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y, map);
                self.apply_vertical_tunnel(prev.y, new.y, new.x, map);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x, map);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y, map);
            }
        }
    }
}

impl MapArchitect for RoomArchitect{
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> CrawlerMapBuilder {
        println!("Room architect new");
        let mut map_builder = CrawlerMapBuilder::gen_clean_builder();

        map_builder.fill(TileType::Wall);
        let mut rooms = self.build_randorm_rooms(rng, &mut map_builder.map);
        println!("Room architect rooms count {}", rooms.len());
        self.build_corridors(rng, &rooms, &mut map_builder.map);
        map_builder.player_start = rooms[0].center();
        map_builder.monster_spawn = rooms.iter().skip(1).map(|r| r.center()).collect();
        map_builder.amulet_start = map_builder.find_most_distant();

        map_builder
    }
}
