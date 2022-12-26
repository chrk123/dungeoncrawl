use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_query = <(&Health, &Point)>::query().filter(component::<Player>());

    let (player_health, player_position) = player_query.iter(ecs).nth(0).unwrap();

    if player_health.current < 1 {
        *turn_state = TurnState::GameOver;
        return;
    }

    if let Some(amulet_position) = <&Point>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .nth(0)
    {
        if *amulet_position == *player_position {
            *turn_state = TurnState::Victory;
            return;
        }
    }

    let new_state = match turn_state {
        TurnState::Victory => return,
        TurnState::GameOver => return,
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    *turn_state = new_state;
}
