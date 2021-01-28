use crate::misc::*;
use crate::obstacle::Obstacle;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Grid<T: Debug> {
    /// The actual grid
    content: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Debug,
{
    /// Creates a new grid instance.
    /// The provided closure is used to obtain the respective
    /// items of the grid.
    pub fn new<I>(dimensions: usize, mut init: I) -> Self
    where
        I: FnMut() -> T,
    {
        let mut rows = Vec::with_capacity(dimensions);

        for _ in 0..dimensions {
            let mut columns = Vec::with_capacity(dimensions);
            for _ in 0..dimensions {
                let next_item = init();
                columns.push(next_item);
            }
            rows.push(columns);
        }

        Self { content: rows }
    }

    /// Returns all the neighbours for a given position
    pub fn get_neighbours(&self, location: Location) -> Neighbours<'_, T>
    where
        T: Obstacle,
    {
        if !self.is_in_bounds(&location) {
            return Neighbours::empty();
        }

        let top = location
            .0
            .checked_sub(1)
            .map(|index| self.content.get(index).map(|items| items.get(location.1)))
            .flatten()
            .flatten()
            .map(|item| item.is_obstacle())
            .flatten();

        let bottom = self
            .content
            .get(location.0 + 1)
            .map(|items| items.get(location.1))
            .flatten()
            .map(|item| item.is_obstacle())
            .flatten();

        let left = self
            .content
            .get(location.0)
            .map(|items| location.1.checked_sub(1).map(|index| items.get(index)))
            .flatten()
            .flatten()
            .map(|item| item.is_obstacle())
            .flatten();

        let right = self
            .content
            .get(location.0)
            .map(|items| items.get(location.1 + 1))
            .flatten()
            .map(|item| item.is_obstacle())
            .flatten();

        Neighbours::new(left, right, top, bottom)
    }

    pub fn replace_node(&mut self, new_node: T, location: Location) {
        // make sure its in bounds
        if self.is_in_bounds(&location) {
            self.content[location.0][location.1] = new_node;
        }
    }

    fn is_in_bounds(&self, location: &Location) -> bool {
        let bounds = self.content.len();
        !(location.0 >= bounds || location.1 >= bounds)
    }
}

impl<T: fmt::Display + fmt::Debug> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for col in self.content.iter() {
            for cell in col.iter() {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
