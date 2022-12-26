use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_health_query = <&Health>::query().filter(component::<Player>());
    let is_game_over = player_health_query.iter(ecs).any(|h| h.current < 1);

    if is_game_over {
        *turn_state = TurnState::GameOver;
        return;
    }

    let new_state = match turn_state {
        TurnState::GameOver => return,
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    *turn_state = new_state;
}
