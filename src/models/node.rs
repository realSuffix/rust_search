use crate::grid::Grid;
use crate::traits::*;
use rand::distributions::{Distribution, Uniform};
use crate::models::Node;
use std::fmt;

impl Node {
    pub fn new_grid(size: usize, dispersion: f32) -> Grid<Self>{
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0.0..1.0);

        Grid::new(size, || {
            let curr = range.sample(&mut rng);
            if curr < dispersion {
                Node::Obstacle
            } else {
                Node::Empty
            }
        })
    }
}

impl Obstacle for Node {
    fn is_obstacle(&self) -> Option<&Self> {
        if *self == Node::Obstacle {
            None
        } else {
            Some(self)
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "E"),
            Self::Obstacle => write!(f, "X"),
            Self::Start => write!(f, "S"),
            Self::Goal => write!(f, "G"),
            Self::Path => write!(f, "*"),
        }
    }
}
