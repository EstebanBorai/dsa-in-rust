//! Skip List
//!
//! A List Data Structure where each node hold multiple references to
//! other nodes of the same list.
//!
//! Each node is inserted using a probabilistic approach in order to
//! have a balanced list and avoid ruining performance on "find" operations.
//!
//! Ideally each node should hold half the number of nodes the previous
//! level has.
use rand::random;
use std::cell::RefCell;
use std::rc::Rc;

/// A `Link` to the next `Node`
pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// A `List` `Node` holding a value of type `T` and the `Link` to the
/// next `Node` which could be `None`
#[derive(Clone, Debug)]
pub struct Node<T: std::fmt::Debug> {
    links: Vec<Link<T>>,
    value: T,
}

impl<T> Node<T>
where
    T: std::fmt::Debug,
{
    /// Creates a new `Node` with the provided value
    pub fn new(level: usize, value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            links: vec![None; level],
            value,
        }))
    }

    /// Retrieves a reference to this `Node`'s value
    pub fn peek_value(&self) -> &T {
        &self.value
    }
}

#[derive(Debug)]
pub struct SkipList<T: std::fmt::Debug> {
    head: Link<T>,
    tails: Vec<Link<T>>,
    length: u32,
    max_level: usize,
}

impl<T> SkipList<T>
where
    T: std::fmt::Debug,
{
    pub fn new(max_level: usize) -> Self {
        SkipList {
            head: None,
            tails: vec![None; max_level + 1],
            length: 0,
            max_level,
        }
    }

    pub fn append(&mut self, value: T) {
        // the first node should be present on all levels
        let level = 1 + if self.head.is_none() {
            // the first node should use the maximum level
            self.max_level
        } else {
            // if theres already a `head` node defined
            // use a random level
            self.get_level()
        };

        let new: Rc<RefCell<Node<T>>> = Node::new(level, value);

        // every node should now have their tails updated with the new node
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().links;

                next[i] = Some(new.clone());
            }

            self.tails[i] = Some(new.clone());
        }

        if self.head.is_none() {
            self.head = Some(new.clone());
        }

        self.length += 1;
    }

    fn get_level(&self) -> usize {
        let mut level = 0;

        while random::<bool>() && level < self.max_level {
            level += 1;
        }

        level
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn creates_an_empty_skip_list() {
        let list = SkipList::<String>::new(3);

        assert!(list.head.is_none());
        assert_eq!(list.tails.len(), 4);
        assert_eq!(list.length, 0);
        assert_eq!(list.max_level, 3);
    }

    #[test]
    fn appends_a_node_to_the_list() {
        let mut list = SkipList::<String>::new(3);

        list.append(String::from("Hello World"));

        assert!(list.head.is_some());
        assert_eq!(list.tails.len(), 4);
        assert_eq!(list.length, 1);
        assert_eq!(list.max_level, 3);
    }
}
