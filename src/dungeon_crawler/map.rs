
const NUM_TILES: i64 = SCREEN_HEIGHT * SCREEN_WIDTH;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType{
    Wall,
    Floor,
}

pub struct Map {
    tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor, NUM_TILES],
        }
    }
    pub fn map_idx(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, YELLOW, BLACK, to_cp437('#')),
                }
            }
        }
    }
}