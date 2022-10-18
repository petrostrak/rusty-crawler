use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        
        // It’s safer and more efficient to use commands, rather than 
        // directly modi- fying the component. Legion can batch the 
        // updates and perform them all at once very quickly. Adding a 
        // component that already exists replaces the old one.
        commands.add_component(want_move.entity, want_move.destination);

        // Accessing components on an entity outside of your query is 
        // a little more complicated. You can access the details of 
        // another component with the entry_ref() method. This returns 
        // a Result.
        if ecs.entry_ref(want_move.entity)
            
            // You know the entity that wishes to move exists, 
            // so you can unwrap() the Option to access its contents.
            .unwrap()
            .get_component::<Player>().is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity);
}