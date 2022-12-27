use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_in_camera: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name)>::query();

    let mouse_in_world = camera.to_world(mouse_in_camera);

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    positions
        .iter(ecs)
        .filter(|(_, pos, _)| {
            **pos == mouse_in_world && fov.visible_tiles.contains(&mouse_in_world)
        })
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_in_camera * 4;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };

            draw_batch.print(screen_pos, &display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
