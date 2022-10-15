use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    // This function checks that the location specified in point 
    // is greater than zero on both the x and y axes, and that 
    // it is less than the screen height and width.
    pub fn in_bounds(&self, point: Point) ->bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
            && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // Determine if the player can enter a tile.
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // Determine the tile's index coordinates. Indicate an error 
    // condition if the requested coordinates fall outside of the 
    // map boundaries.
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}