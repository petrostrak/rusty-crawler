use crate::prelude::*;

// The #[system] line annotates the player_input 
// function with a procedural macro named system. 
// This macro transforms your function name into 
// player_input_system, and wraps it with all of 
// the extra code Legion requires to construct a 
// system.
// Your player_input system needs access to some 
// of the resources you inserted into Legion’s 
// Resources manager, and some of the components 
// you have defined.
#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        // Calling iter_mut() runs the query and places the results in an iterator.
        players.iter(ecs).for_each(| (entity, pos) | {
            let destination = *pos + delta;
            commands
                .push(((), WantsToMove{ entity: *entity, destination }));
        });
        *turn_state = TurnState::PlayerTurn;
    }
}