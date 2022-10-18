use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
// This code iterates the victims collection you just created. Then it uses if 
// let to activate only if the victim has health (once again, preventing you 
// from mind- lessly beating up inanimate objects). It then reduces the 
// victim’s current health by one. If the victim’s hit points are less than 
// one, the victim is deleted from the game. Finally, it deletes the WantsToAttack message.
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims : Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();
    
    victims
        .iter()
        .for_each(|(message, victim)| {
            if let Ok(mut health) = ecs
                .entry_mut(*victim)
                .unwrap() 
                .get_component_mut::<Health>()
            {
                println!("Health before attack: {}", health.current); 
                health.current -= 1;
                if health.current < 1 {
                    commands.remove(*victim); 
                }
                println!("Health after attack: {}", health.current); 
            }
            commands.remove(*message);
        });
}