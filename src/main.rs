#![warn(clippy::pedantic)]

mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2; 
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) { 
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        // This makes the current keyboard state available 
        // to any system that requests it.
        self.resources.insert(ctx.key);

        // Identifying Monsters with Tooltips
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos));

        // self.resources.get::<TYPE> requests a resource of a given type (in this case TurnState) 
        // from the ECS’s resources. The result is returned as an Option, so you need to unwrap() 
        // it to access the contents. The final call to clone() duplicates the state. This ensures 
        // that the resource is no longer borrowed—you’re looking at a copy of the current turn state, 
        // rather than the original. This is another example of working around Rust’s borrow checker.
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(
                &mut self.ecs, 
                &mut self.resources,
            ),
            TurnState::PlayerTurn => {
                self.player_systems.execute(
                    &mut self.ecs, 
                    &mut self.resources
                );
            },
            TurnState::MonsterTurn => {
                self.monster_systems.execute(
                    &mut self.ecs, 
                    &mut self.resources
                );
            }
        }
        render_draw_buffer(ctx).expect("render error")
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Rusty Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH*8, DISPLAY_HEIGHT*2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}