use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    
    // Create a new Query with writable access to Point and read-only access to
    // MovingRandomly.
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    movers
        .iter(ecs)
        .for_each(| (entity, pos, _) | {
        let mut rng = RandomNumberGenerator::new();

            // Randomly choose a direction to move and store the delta.
            // Add position to it to determine the destination.
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;
            
            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs.entry_ref(*victim)
                    .unwrap().get_component::<Player>().is_ok()
                    {
                        commands
                            .push(((), WantsToAttack{
                                attacker: *entity,
                                victim: *victim
                            }));
                    }
                    attacked = true;
                }
            );
    
            if !attacked {
                commands
                    .push(((), WantsToMove{ entity: *entity, destination }));
            }
    
        }
    );
}