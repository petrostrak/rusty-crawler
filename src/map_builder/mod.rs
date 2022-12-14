use crate::prelude::*;

mod automata;
mod drunkard;
mod prefab;
mod rooms;
mod themes;
use self::{drunkard::DrunkardsWalkArchitect, prefab::apply_prefab, rooms::RoomsArchitect, automata::CellularAutomataArchitect, themes::*};

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect{}),
            1 => Box::new(RoomsArchitect{}),
            _ => Box::new(CellularAutomataArchitect{})
        };
        let mut mb = architect.new(rng);
        apply_prefab(&mut mb, rng);

        mb.theme = match rng.range(0, 2) {
            0 => DungeonTheme::new(),
            _ => ForestTheme::new(),
        };
        
        mb
    }

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

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        use std::cmp::{min,max};

        // Range iterators expect that the starting value of a range will be
        // the minimum value, and the destination the maximum.
        // This function uses min() and max() to find the lowest and highest
        // of a pair of values - in this case, the starting position. It then
        // iterates y from the start to the end of the corridor, carving the
        // tunnel along the way.
        for y in min(y1, y2) ..= max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x,y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    // apply_horizontal_tunnel() works the same way as the apply_vertical_tunnel()
    // but it traverses the x axis instead of the y axis.
    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        use std::cmp::{min, max};
        for x in min(x1,x2) ..= max(x1,x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x,y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        
        // Vectors include a sort_by() to sort their contents. It requires a closure,
        // an inline function, that calls the cmp() function on two elements of the 
        // vector's contents. cmp() returns an indicator if two elements are the same,
        // or one is greated than the other. Sorting the rooms by their center point 
        // before allocating corridors makes it more likely that corridors will connect 
        // adjacent rooms and not snake across the whole map.
        //
        // sort_by() sends pairs of rooms to the closure. The closure receives these as 
        // a and b. a.center().x finds the x coordinate of room A. This is then compared 
        // via the cmp() function with the center of room B.
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));
        
        // enumerate() counts items in the iterator and includes them as the first entry
        // in a tuple. The (i, room) extracts the counter into the variable i. skip() allows
        // you to ignore some entries in the iretator, in this case, the 1st one.
        for (i, room) in rooms.iter().enumerate().skip(1) {
            
            // Obtain the center position, as a Point struct, of the current and previous
            // rooms. This is why we skip the 1st entry, the previous would be invalid.
            let prev = rooms[i-1].center();
            let new = room.center();

            // Randomly dig the horizontal and then vertical parts of the corridor,
            // or vice versa.
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a,b| a.1.partial_cmp(b.1).unwrap())
                .unwrap().0
        )
    }

    fn spawn_monsters(
        &self, 
        start: &Point, 
        rng: &mut RandomNumberGenerator
    ) -> Vec<Point> {
        const NUM_MONSTERS : usize = 50;
        let mut spawnable_tiles : Vec<Point> = self.map.tiles
            .iter()
            .enumerate()

            // Filter to only include tiles that are floors and more tha 10 tiles distant
            // from the player's start point.
            .filter(|(idx, t)|
                **t == TileType::Floor &&
                    DistanceAlg::Pythagoras.distance2d(
                        *start,
                        self.map.index_to_point2d(*idx)
                    ) > 10.0
            )
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0 .. NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles)
                .unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
}