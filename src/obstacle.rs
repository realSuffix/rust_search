pub trait Obstacle {
    /// Only return self if it is not an
    /// obstacle!
    fn is_obstacle(&self) -> Option<&Self>;
}
