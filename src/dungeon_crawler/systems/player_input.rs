use crate::dungeon_crawler::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]

pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn: &mut TurnState,
) {
    static mut frame: i32 = 0;
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    // let mut cfrm: i32 = 0;
    // unsafe {frame += 1; frm = frame;}
    // println!("Players count: {}, frame {}", players.iter(ecs).count(), frm);
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
        VirtualKeyCode::Key0 => use_item(9, ecs, commands),
        VirtualKeyCode::Key1 => use_item(0, ecs, commands),
        VirtualKeyCode::Key2 => use_item(1, ecs, commands),
        VirtualKeyCode::Key3 => use_item(2, ecs, commands),
        VirtualKeyCode::Key4 => use_item(3, ecs, commands),
        VirtualKeyCode::Key5 => use_item(4, ecs, commands),
        VirtualKeyCode::Key6 => use_item(5, ecs, commands),
        VirtualKeyCode::Key7 => use_item(6, ecs, commands),
        VirtualKeyCode::Key8 => use_item(7, ecs, commands),
        VirtualKeyCode::Key9 => use_item(8, ecs, commands),

        VirtualKeyCode::G => {
            let (player, player_pos) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos)))
                .unwrap();

            let mut items = <(Entity, &Item, &Point)>::query();
            items
                .iter(ecs)
                .filter(|(entity, _, &item_pos)| item_pos == player_pos)
                .for_each(|(entity, _, _)| {
                    commands.remove_component::<Point>(*entity);
                    commands.add_component(*entity, Carried(player));
                });
            Point::new(0, 0)
        }
        _ => {
            // println!("Something else");
            Point::zero()
        }
    };

    let (player_entity, destination) = players
        .iter(ecs)
        .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
        .unwrap();
    let mut monsters = <(Entity, &Point)>::query().filter(component::<Enemy>());

    //let mut did_something = false;
    if delta.x != 0 || delta.y != 0 {
        let mut hit_someone = false;

        monsters
            .iter(ecs)
            .filter(|(_, pos)| **pos == destination)
            .for_each(|(entity, _)| {
                hit_someone = true;
                //did_something = true;
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: player_entity,
                        victim: *entity,
                    },
                ));
            });
        //println!("Hit someone: {}", hit_someone);

        if !hit_someone {
            commands.push((
                (),
                WantsToMove {
                    entity: player_entity,
                    destination,
                },
            ));
            //did_something = true;
        }
    }

    *turn = TurnState::PlayerTurn;
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _)| Some(*entity))
        .unwrap();

    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));
    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
    Point::zero()
}
