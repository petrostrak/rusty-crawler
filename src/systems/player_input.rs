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
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            
            // Access components with a query. Queries list one or more components, and return
            // references (mutable if &mut is used), to each instance of that component type.
            // If you request more than one component type, only entities with all of those 
            // components will be grouped together.
            let mut players = <&mut Point>::query()
            
                // You don't want to update all Point components, just the player. Otherwise, when
                // monsters and items would move when the player moves. Note that the query doesn't
                // become an iterator until you call iter() or iter_mut(), it's still a query.
                .filter(component::<Player>());
            
            // Calling iter_mut() runs the query and places the results in an iterator.
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                    *turn_state = TurnState::PlayerTurn;
                }
            })
        }
    }
}