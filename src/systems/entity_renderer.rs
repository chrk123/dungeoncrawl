use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let mut entities_query = <(&Point, &Render)>::query();

    let camera_offset = Point::new(camera.left_x, camera.top_y);

    entities_query.iter(ecs).for_each(|(pos, render)| {
        draw_batch.set(*pos - camera_offset, render.color, render.glyph);
    });

    draw_batch.submit(5000).expect("Batch error");
}
