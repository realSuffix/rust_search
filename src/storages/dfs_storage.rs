use crate::traits::Storage;

pub struct DFSStorage<T> {
    items: Vec<T>,
}

impl<T> Storage<T> for DFSStorage<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    fn empty(&self) -> bool {
        self.items.len() == 0
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn push(&mut self, node: T) {
        self.items.push(node);
    }
}
