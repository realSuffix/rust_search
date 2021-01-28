use crate::traits::Storage;
use std::collections::VecDeque;

pub struct BFSStorage<T> {
    items: VecDeque<T>,
}

impl<T> Storage<T> for BFSStorage<T> {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }
    fn empty(&self) -> bool {
        self.items.len() == 0
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    fn push(&mut self, node: T) {
        self.items.push_back(node);
    }
}
