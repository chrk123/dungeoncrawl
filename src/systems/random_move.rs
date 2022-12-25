use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Player)]
#[read_component(Health)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut random_movers = <(Entity, &Point)>::query().filter(component::<MovingRandomly>());
    let mut positions_with_health = <(Entity, &Point)>::query().filter(component::<Health>());

    random_movers.iter(ecs).for_each(|(entity, pos)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(0, 1),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(-1, 0),
        } + *pos;

        let mut attacking = false;

        positions_with_health
            .iter(ecs)
            .filter(|(potential_victim, pos)| {
                **pos == destination
                    && ecs
                        .entry_ref(**potential_victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
            })
            .for_each(|(potential_victim, _)| {
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: *entity,
                        victim: *potential_victim,
                    },
                ));
                attacking = true;
            });

        if !attacking {
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}
