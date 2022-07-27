pub use crate::dungeon_crawler::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn end_turn(ecs: &SubWorld, #[resource] state: &mut TurnState) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let current_state = state.clone();
    let mut new_state = match state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };
    player_hp.iter(ecs).for_each(|hp|
    {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        } 
    });

    *state = new_state;
}
