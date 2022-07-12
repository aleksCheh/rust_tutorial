use crate::dungeon_crawler::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@')
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &mut CrawlerMap) {
        if let Some(key) = ctx.key {
            
            let delta  = match key {
                VirtualKeyCode::Left => { println!("Left"); Point::new(-1, 0)},
                VirtualKeyCode::Right => { println!("Right"); Point::new(1, 0)},
                VirtualKeyCode::Up => {println!("Up"); Point::new(0, -1)},
                VirtualKeyCode::Down => {println!("Down"); Point::new(0, 1)},
                _ => {println!("Something else"); Point::zero()},
            };
            
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position){
                self.position = new_position;
            }
        }
        
    }
}
