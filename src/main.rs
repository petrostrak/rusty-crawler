mod map;

mod prelude {
    pub use bracket_lib::prelude::*; 
    pub const SCREEN_WIDTH: i32 = 80; 
    pub const SCREEN_HEIGHT: i32 = 50; 
    pub use crate::map::*;
}

use prelude::*;

fn main() {
    println!("Hello, world!");
}
