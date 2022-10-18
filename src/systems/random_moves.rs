use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    
    // Create a new Query with writable access to Point and read-only access to
    // MovingRandomly.
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    movers 
        .iter_mut(ecs)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            
            // Randomly choose a direction to move and store the delta.
            // Add position to it to determine the destination.
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;
            commands   
                .push(((), WantsToMove{ entity: *entity, destination }));
        }
    );
}