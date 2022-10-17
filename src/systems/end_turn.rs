use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    // Set the turn resource to the chosen value. The asterisk 
    // (*) de-references the variable, allowing you to write 
    // directly to the stored resource.
    *turn_state = new_state;
}