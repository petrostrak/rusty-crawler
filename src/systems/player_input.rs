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
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key : &Option<VirtualKeyCode>,
    #[resource] turn_state : &mut TurnState
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)) )
                .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut did_something = false;
        if delta.x !=0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| {
                    **pos == destination
                })
                .for_each(|(entity, _) | {
                    hit_something = true;
                    did_something = true;

                    commands
                        .push(((), WantsToAttack{
                            attacker: player_entity,
                            victim: *entity,
                        }));
                });

            if !hit_something {
                did_something = true;
                commands
                    .push(((), WantsToMove{
                        entity: player_entity,
                        destination
                    }));
            }

            if !did_something {
                if let Ok(mut health) = ecs
                    .entry_mut(player_entity)
                    .unwrap()
                    .get_component_mut::<Health>()
                {
                    health.current = i32::min(health.max, health.current+1);
                }
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}