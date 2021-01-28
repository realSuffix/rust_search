pub mod neighbours;
pub mod node;

use std::fmt::Debug;

/// A location.
/// First value is y, second value is x.
pub struct Location(pub usize, pub usize);

#[derive(Debug)]
pub struct Neighbours<'a, T> {
    pub left: Option<&'a T>,
    pub right: Option<&'a T>,
    pub top: Option<&'a T>,
    pub bottom: Option<&'a T>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Node {
    Empty,
    Obstacle,
    Start,
    Goal,
    Path,
}
