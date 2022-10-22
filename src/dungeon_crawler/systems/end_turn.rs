use crate::dungeon_crawler::crawler_map;
pub use crate::dungeon_crawler::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] state: &mut TurnState, #[resource] map: &CrawlerMap) {
    let (player_hp, player_pos) = <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();
    let current_state = state.clone();
    let mut new_state = match state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let mut amulets = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_default = Point::new(-1, -1);  
    let amulet_pos = amulets.iter(ecs).nth(0).unwrap_or(&amulet_default);

    if player_hp.current < 1 {
        new_state = TurnState::GameOver;
    }
    if player_pos == amulet_pos {
        new_state = TurnState::Victory;
    }
    let idx = map.point2d_to_index(*player_pos);
    if map.tiles[idx] == TileType::Exit {
        new_state = TurnState::NextLevel;
    }
    *state = new_state;

}
