use crate::dungeon_crawler::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]

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
            //println!("Left");
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
           // println!("Something else");
            Point::zero()
        }
    };
    
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player_entity, destination) = players.iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
    let mut monsters = <(Entity, &Point)>::query().filter(component::<Enemy>());

    let mut did_something = false;
    if delta.x != 0 || delta.y != 0 {
        let mut hit_someone = false;
             
        monsters.iter(ecs).filter(|(_, pos)|**pos == destination)
        .for_each(|(entity, _)| {
            hit_someone = true;
            did_something = true;
            commands.push(((), WantsToAttack{attacker: player_entity, victim: *entity}));
        });
        println!("Hit someone: {}", hit_someone);

        if !hit_someone {            
            commands.push(((), WantsToMove{entity: player_entity, destination}));
            did_something = true;
        }

    }
    if !did_something {
        if let Ok(mut health) = ecs.entry_mut(player_entity).unwrap().get_component_mut::<Health>(){
            health.current = i32::min(health.max, health.current +1);
        }
    }
    
    *turn = TurnState::PlayerTurn;
}
