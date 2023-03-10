use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // this adds a Point component (want_move.destination) to the given entity
        // since this component already exists, it replaces it and thus updates the position
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(fov) = ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<FieldOfView>()
        {
            commands.add_component(want_move.entity, fov.clone_dirty());

            if ecs
                .entry_ref(want_move.entity)
                .unwrap()
                .get_component::<Player>()
                .is_ok()
            {
                camera.on_player_move(want_move.destination);
                fov.visible_tiles.iter().for_each(|visible_tile| {
                    map.revealed_tiles[map_idx(visible_tile.x, visible_tile.y)] = true;
                });
            }
        }
    }
    commands.remove(*entity);
}
