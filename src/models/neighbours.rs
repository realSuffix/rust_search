use std::fmt::Debug;
use std::iter::IntoIterator;
use crate::models::Neighbours;

impl<'a, T> Neighbours<'a, T>
where
    T: Debug,
{
    pub fn new(
        left: Option<&'a T>,
        right: Option<&'a T>,
        top: Option<&'a T>,
        bottom: Option<&'a T>,
    ) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn empty() -> Self {
        Self {
            left: None,
            right: None,
            top: None,
            bottom: None,
        }
    }
}



// and we'll implement IntoIterator
impl<'a, T> IntoIterator for Neighbours<'a, T> {
    type Item = &'a T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut items = Vec::new();

        if let Some(item) = self.left {
            items.push(item);
        }

        if let Some(item) = self.right {
            items.push(item);
        }
        if let Some(item) = self.top {
            items.push(item);
        }

        if let Some(item) = self.bottom {
            items.push(item);
        }

        items.into_iter()
    }
}
