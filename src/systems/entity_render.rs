use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    
    // Remember to start a new DrawBatch in each system that writes 
    // to the terminal.
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // Query for all entities that have a Point and Render component.
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            
            // Set the screen character at the position in pos to the 
            // glyph and color specified in the Render component.
            draw_batch.set(
                *pos - offset, 
                render.color, 
                render.glyph,
            );
        }
    );

    // Submit the render batch: 5,000 is used as a sort order because 
    // the map may include 4,000 elements. It’s a good idea to leave 
    // some room in case that changes or you add some user interface 
    // elements.
    draw_batch.submit(5000).expect("batch error")
}