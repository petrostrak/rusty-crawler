use crate::prelude::*;
use super::MapArchitect;

// How far a miner can stumble before passing out.
const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkardsWalkArchitect {}

impl DrunkardsWalkArchitect {
    fn drunkard(
        &mut self,
        start: &Point,
        rng: &mut RandomNumberGenerator,
        map: &mut Map
    ) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            // Randomly pick a direction and adjust the digger's current position
            // accordingly.
            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }

            // If the digger has left the map, break.
            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map : Map::new(),
            rooms : Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero()
        };

        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH /2, SCREEN_HEIGHT/2);

        // Start the digger at a random location on the map.
        self.drunkard(&center, rng, &mut mb.map);
        while mb.map.tiles.iter()
            .filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR
        {
            self.drunkard(
                &Point::new(
                    rng.range(0, SCREEN_WIDTH),
                    rng.range(0, SCREEN_HEIGHT)
                ),
                rng,
                &mut mb.map
            );
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0
            );
            dijkstra_map.map
                .iter()
                .enumerate()

                // Use filter() to retain values with a distance greater that 2000 tiles from
                // the starting point.
                .filter(|(_, distance)| *distance > &2000.0)
                
                // For each remaining entry in the iterator, convert the tile to a wall.
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}