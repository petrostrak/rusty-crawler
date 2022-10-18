use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query()
        .filter(component::<Player>());
    
    let player_health = health_query
        .iter(ecs)
        
        // It retrieves a single entry from an iterator as an option. 
        // You know that there is a player, so it’s safe to unwrap() 
        // the option—extracting the player’s health component.
        .nth(0)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move");
    draw_batch.bar_horizontal(
        Point::zero(), 
        SCREEN_WIDTH*2, 
        player_health.current, 
        player_health.max, 
        ColorPair::new(RED, BLACK)
    );
    draw_batch.print_color_centered(
        0, 
        format!(" Health: {} / {} ",
            player_health.current,
            player_health.max
        ), 
        ColorPair::new(WHITE, RED)
    );
    draw_batch.submit(10000).expect("batch error");
}