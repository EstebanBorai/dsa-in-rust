use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    value: T,
    key: usize,
    forward_pointers: Vec<Link<T>>,
}

impl<T> Node<T> {
    pub fn new(key: usize, level: usize, value: T) -> Self {
        Node {
            key,
            value,
            forward_pointers: Vec::new(),
        }
    }
}

pub struct SkipList<T> {
    max_level: usize,
    probability: f32,
    head: Link<T>,
    last_node: Link<T>,
}

impl<T> SkipList<T> {
    pub fn new(max_level: usize, probability: f32) -> Self {
        SkipList {
            max_level,
            probability,
            head: None,
            last_node: None,
        }
    }
}

impl<T> SkipList<T> {
    pub fn find(&self, search_key: usize) -> Link<T> {
        if let Some(head) = self.head.clone() {
            for i in self.max_level..0 {
                // for every list level
                loop {}
            }
        }

        None
    }
}
