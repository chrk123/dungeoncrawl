use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let mut entities_query = <(&Point, &Render)>::query();

    entities_query.iter(ecs).for_each(|(pos, render)| {
        draw_batch.set(camera.from_world(pos), render.color, render.glyph);
    });

    draw_batch.submit(5000).expect("Batch error");
}
