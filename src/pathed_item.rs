use std::path::Path;

/// An item with an associated path.
pub struct PathedItem<P: AsRef<Path>, T> {
    path: P,
    item: T,
}

impl<P: AsRef<Path>, T> PathedItem<P, T> {
    /// Creates a new [`PathedItem`] with the specified values.
    pub fn new(path: P, item: T) -> PathedItem<P, T> {
        Self { path, item }
    }

    /// Returns the [`Path`] associated with the item.
    pub fn path(&self) -> &P {
        &self.path
    }

    /// Returns a mutable reference to the [`Path`] associated with the item.
    pub fn path_mut(&mut self) -> &mut P {
        &mut self.path
    }

    /// Converts the item into a `Box<T>`.
    pub fn into_boxed_item(self) -> PathedItem<P, Box<T>> {
        PathedItem {
            path: self.path,
            item: Box::new(self.item),
        }
    }
}

impl<P: AsRef<Path>, T> std::ops::Deref for PathedItem<P, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<P: AsRef<Path>, T> std::ops::DerefMut for PathedItem<P, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}
