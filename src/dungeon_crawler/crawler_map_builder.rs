use crate::dungeon_crawler::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct CrawlerMapBuilder {
    pub map: CrawlerMap,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}
impl CrawlerMapBuilder {
    #[allow(dead_code)]
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| {
            *t = tile;
        });
    }

    fn build_randorm_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlaps: bool = false;
            for r in &self.rooms {
                if r.intersect(&room) {
                    overlaps = true;
                }
            }
            if !overlaps {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut cb = Self {
            map: CrawlerMap::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        cb.fill(TileType::Wall);
        cb.build_randorm_rooms(rng);
        cb.build_corridors(rng);
        cb.player_start = cb.rooms[0].center();

        cb
    }
}
