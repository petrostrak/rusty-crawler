use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn random_noise_map(
        &mut self,
        rng: &mut RandomNumberGenerator,
        map: &mut Map)
    {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1 ..= 1 {
            for ix in -1 ..= 1 {

                // Don't count the current tile, only count its neighbors.
                if !(ix==0 && iy ==0) && map.tiles[map_idx(x+ix, y+iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1 .. SCREEN_HEIGHT -1 {
            for x in 1 .. SCREEN_WIDTH -1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {

        // Store the center of the map in a Point.
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        let closest_point = map.tiles

            // Iterate all of the map tiles.
            .iter()

            // Call enumerate() to append the tile's index in the tile vector to the result.
            // Each iteration now contains a tuple of (index, tiletype).
            .enumerate()

            // Use filter() to remove all the tiles that aren't a floor. Now we have a list of the
            // index and type of all floors in the map.
            .filter(|(_, t)| **t == TileType::Floor)
            
            // Calculate the Pythagorean distance from each remaining tile to the map's center.
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(
                center,
                map.index_to_point2d(idx)
            )))

            // min_by() finds the lowest value in an iterator set and allows you to specify the
            // comparison technique.
            .min_by(|(_, distance), (_, distance2)|
                distance.partial_cmp(&distance2).unwrap()
            )
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder{
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}