pub mod grid;
pub mod misc;
pub mod node;
pub mod obstacle;

use crate::misc::*;
use crate::node::Node;
use crate::node::init_node_grid;

pub const SPREAD: f32 = 0.0;
pub const SIZE: usize = 2;

fn main() {
    let mut grid = init_node_grid(SIZE, SPREAD);
    grid.replace_node(Node::Start, Location(1, 2));
    println!("{}", grid);
}
