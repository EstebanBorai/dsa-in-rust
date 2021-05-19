//! Dynamic Array
//!
//! An array with expansion capabilities thanks to a `capacity` and
//! `length` metadata values.
//!
//! This implementation makes use of OpenJDK 8 `ArrayList` class,
//! `grow` method algorithm to "grow" the underlying array when inserting
//! more items than the current `cap` value.
use std::cmp;

pub struct DynamicArray<T: Clone> {
    buffer: Box<[Option<T>]>,
    capacity: usize,
    length: usize,
}

impl<T: Clone> DynamicArray<T> {
    pub fn new() -> Self {
        let buffer = Vec::new();
        let buffer = buffer.into_boxed_slice();

        DynamicArray {
            buffer,
            length: 0,
            capacity: 0,
        }
    }

    /// This is a Rust implementation of OpenJDK 8 ArrayList.grow method
    ///
    /// Source code is available here: https://hg.openjdk.java.net/jdk8/jdk8/jdk/file/tip/src/share/classes/java/util/ArrayList.java#l237
    fn grow(&mut self, min_cap: usize) {
        let current_capacity = self.buffer.len();
        let mut extended_capacity = current_capacity + (current_capacity >> 1);

        extended_capacity = cmp::max(extended_capacity, min_cap);
        extended_capacity = cmp::min(extended_capacity, usize::MAX);
        self.capacity = extended_capacity;

        let buffer = self.buffer.clone();

        self.buffer = vec![None; extended_capacity].into_boxed_slice();
        self.buffer[..buffer.len()].clone_from_slice(&buffer);
    }

    fn ensure_capacity(&mut self, items_to_add: usize) {
        if self.length + items_to_add > self.capacity {
            self.grow(self.length + items_to_add);
        }
    }

    pub fn item_at(&mut self, index: usize) -> Option<T> {
        if self.length > index {
            return self.buffer[index].clone();
        }

        None
    }

    pub fn add(&mut self, value: T) {
        self.ensure_capacity(1);
        self.buffer[self.length] = Some(value);
        self.length += 1;
    }
}

pub struct DynamicArrayIterator<T: Clone> {
    current: usize,
    data: Box<[T]>,
}

impl<T: Clone> Iterator for DynamicArrayIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current < self.data.len() {
            let item = self.data[self.current].clone();
            self.current += 1;

            return Some(item);
        }

        None
    }
}

impl<T: Clone> DoubleEndedIterator for DynamicArrayIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = self.data[self.current].clone();

            if self.current == 0 {
                self.current = self.data.len() - 1;
            } else {
                self.current -= 1;
            }

            return Some(item);
        }

        None
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn extends_internal_buffer_size() {
        let mut list = DynamicArray::<String>::new();

        assert_eq!(list.capacity, 0);
        assert_eq!(list.length, 0);
        assert_eq!(list.buffer.len(), 0);

        list.grow(10);

        assert_eq!(list.capacity, 10);
        assert_eq!(list.length, 0);
        assert_eq!(list.buffer.len(), 10);
    }

    #[test]
    fn adds_item_at_the_end_of_the_array() {
        let mut list = DynamicArray::<String>::new();

        list.add(String::from("foo"));
        list.add(String::from("bar"));

        assert_eq!(list.capacity, 2);
        assert_eq!(list.length, 2);
    }

    #[test]
    fn gets_items_from_array() {
        let mut list = DynamicArray::<String>::new();

        list.add(String::from("foo"));
        list.add(String::from("bar"));

        assert_eq!(list.item_at(0), Some(String::from("foo")));
        assert_eq!(list.item_at(1), Some(String::from("bar")));
        assert_eq!(list.item_at(2), None);
    }
}
