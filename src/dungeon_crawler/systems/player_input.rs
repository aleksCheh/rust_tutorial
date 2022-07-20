use crate::dungeon_crawler::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]

pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn: &mut TurnState,
) {
    let key = match key {
        Some(it) => it,
        _ => return,
    };
    let delta = match key {
        VirtualKeyCode::Left => {
            // println!("Left");
            Point::new(-1, 0)
        }
        VirtualKeyCode::Right => {
            //println!("Right");
            Point::new(1, 0)
        }
        VirtualKeyCode::Up => {
            //println!("Up");
            Point::new(0, -1)
        }
        VirtualKeyCode::Down => {
            //println!("Down");
            Point::new(0, 1)
        }
        _ => {
            //println!("Something else");
            Point::zero()
        }
    };
    let mut players = <(Entity, &mut Point)>::query().filter(component::<Player>());
    players.iter_mut(ecs).for_each(|(entity, pos)| {
        let destination = *pos + delta;
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    });
    *turn = TurnState::PlayerTurn;
}
