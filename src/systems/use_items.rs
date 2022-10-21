use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_items(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    <(Entity, &ActivateItem)>::query().iter(ecs)
    .for_each(|(entity, activate)| {

        let item = ecs.entry_ref(activate.item);
        if let Ok(item) = item {
            if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                healing_to_apply.push((activate.used_by, healing.amount));
            }

            if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);
            }
        }

        commands.remove(activate.item);
        commands.remove(*entity);
    });
}
