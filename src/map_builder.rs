use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    // build_random_rooms() accepts a RandomNumberGenerator as a parameter.
    // It's a good idea to use the same PRNG throughout your map generation,
    // os if you re-use the same seed, you always get the same result.
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        
        // Keep generating rooms until there are NUM_ROOMS rooms on the map.
        while self.rooms.len() < NUM_ROOMS {

            // Generates a randomly positioned room with random sizes.
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10), 
                rng.range(1, SCREEN_HEIGHT -10), 
                rng.range(2, 10), 
                rng.range(2, 10),
            );

            // Test the new room against each placed room, and flag it as
            // overlapping if rooms intersect.
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {

                // If they don't overlap, check that they are within
                // the map boundaries and set their contents to floors.
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0
                        && p.y < SCREEN_HEIGHT 
                    {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room)
            }
        }
    }
}