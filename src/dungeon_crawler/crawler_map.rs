use crate::dungeon_crawler::prelude::*;

const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct CrawlerMap {
    pub tiles: Vec<TileType>,
}

impl CrawlerMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
    #[allow(dead_code)]
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bot_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            YELLOW,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Wall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            YELLOW,
                            BLACK,
                            to_cp437('#'),
                        ),
                    }
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(map_idx(point.x, point.y))
        } else {
            None
        }
    }
    pub fn valid_exit(&self, loc: &Point, delta: &Point) -> Option<usize> {
        let destination = *loc + *delta;
        if self.in_bounds(destination) && self.can_enter_tile(destination) {
            let idx = self.point2d_to_index(destination);
            Some(idx)
        } else {
            None
        }
    }
}

impl Algorithm2D for CrawlerMap {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, p: Point) -> bool {
        return self.in_bounds(p);
    }
}

impl BaseMap for CrawlerMap {
    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(_idx);
        if let Some(idx) = self.valid_exit(&location, &Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(&location, &Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(&location, &Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(&location, &Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }
        exits
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(self.index_to_point2d(_idx1), self.index_to_point2d(_idx2))
    }
}
