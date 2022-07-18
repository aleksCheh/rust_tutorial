pub use crate::dungeon_crawler::*;

#[system]
pub fn end_turn(#[resource] state: &mut TurnState) {
    let new_state = match state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    *state = new_state;
}
