use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = *key {
        // pass the round and heal the player by one when pressing SPACE
        if key == VirtualKeyCode::Space {
            let mut player_health = <&mut Health>::query()
                .filter(component::<Player>())
                .iter_mut(ecs)
                .nth(0)
                .unwrap();
            player_health.current = i32::min(player_health.max, player_health.current + 1);

            *turn_state = TurnState::PlayerTurn;
            return;
        }

        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

            let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
                .unwrap();

            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, p)| **p == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }

            *turn_state = TurnState::PlayerTurn;
        };
    }
}
