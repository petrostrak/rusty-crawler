use crate::prelude::*;

#[system]
// Your system doesn’t use any components, but does need access to the map 
// and camera. It requests these as parameters to the map_render() function 
// and uses the #[resource] annotation to indicate that they are resources.
// Instead of immediate-mode rendering, the system starts a 
// drawing batch. DrawBatch::new() starts a new batch. Draw 
// commands are appended to the batch in the order that you 
// want them to occur.
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'), 
                };

                // You add draw commands to a batch with the same commands as 
                // immediate mode, but calling the batch rather than the context.
                draw_batch.set( pt - offset,
                    ColorPair::new( WHITE,
                    BLACK ),
                    glyph 
                );
            }
        }
    }

    // ❸ Submitting the batch adds it to the global command list. It accepts a single 
    // integer parameter, serving as a sort order. Zero renders first, ensuring that 
    // your map is drawn at the beginning of the render cycle.
    draw_batch.submit(0).expect("batch error");
}