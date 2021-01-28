pub trait Obstacle {
    /// Only return self if it is not an
    /// obstacle!
    fn is_obstacle(&self) -> Option<&Self>;
}

/// The main difference between all
/// algorithms.
pub trait Storage<T> {
    fn empty(&self) -> bool;
    fn pop(&mut self) -> T;
    fn push(&mut self);
}
