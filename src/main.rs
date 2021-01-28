pub mod grid;
pub mod models;
pub mod storages;
pub mod traits;
pub mod algorithm;

use crate::models::*;

pub const SPREAD: f32 = 0.0;
pub const SIZE: usize = 2;

fn main() {
    let mut grid = Node::new_grid(SIZE, SPREAD);
    grid.replace_node(Node::Start, Location(1, 2));
    println!("{}", grid);
}
