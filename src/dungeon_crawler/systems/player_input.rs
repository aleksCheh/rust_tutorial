use crate::dungeon_crawler::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]

pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &CrawlerMap,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => {
                println!("Left");
                Point::new(-1, 0)
            }
            VirtualKeyCode::Right => {
                println!("Right");
                Point::new(1, 0)
            }
            VirtualKeyCode::Up => {
                println!("Up");
                Point::new(0, -1)
            }
            VirtualKeyCode::Down => {
                println!("Down");
                Point::new(0, 1)
            }
            _ => {
                println!("Something else");
                Point::zero()
            }
        };

        if delta.x != 0 || delta.y != 0 {
            println!("Point is valid");
            let mut players = <&mut Point>::query().filter(component::<Player>());

            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                println!("destination x: {}, y: {}", destination.x, destination.y);
                if map.can_enter_tile(destination) {
                    println!("can enter tile");
                    *pos = destination;
                    println!("new pos x: {}, y: {}", pos.x, pos.y);
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
