use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Point)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|p| {
            player_pos = *p;
        });

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
