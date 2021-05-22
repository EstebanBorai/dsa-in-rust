//! Binary Search Tree
//!
//! A Binary Search Tree is a rooted binary tree whose internal nodes each
//! store a key greater than all the keys in the node's left subtree and less
//! than those in its right subtree
use std::mem::replace;

/// A sub-tree from this Binary Search Tree
type Tree<T: std::cmp::Ord + std::fmt::Debug + Clone> = Option<Box<Node<T>>>;

/// A node containing a value of type `T`, and their childrens.
/// Each children of a `Node` on a Binary Search Tree is also a
/// `Tree` where the `left` side `Tree` contains all `Node`s lower
/// than  the current's `Node` value. And the right side of the tree
/// contains all `Node`s with a greather value than this `Node`'s
/// value
#[derive(Debug)]
pub struct Node<T: std::cmp::Ord + std::fmt::Debug + Clone> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: std::cmp::Ord + std::fmt::Debug + Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn boxed(value: T) -> Box<Self> {
        Box::new(Node::new(value))
    }

    pub fn peek(&self) -> &T {
        &self.value
    }
}

/// A Binary Search Tree is a rooted binary tree whose internal nodes each
/// store a key greater than all the keys in the node's left subtree and less
/// than those in its right subtree
#[derive(Debug)]
pub struct BinarySearchTree<T: std::cmp::Ord + std::fmt::Debug + Clone> {
    root: Tree<T>,
    length: u64,
}

impl<T: std::cmp::Ord + std::fmt::Debug + Clone> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            length: 0,
        }
    }

    /// Adds a `value` to the `BinarySearchTree<T>`
    pub fn add(&mut self, value: T) {
        let root = replace(&mut self.root, None);

        self.root = self.add_recursive(root, value);
        self.length += 1;
    }

    /// Walks the tree recursively looking for the right position the
    /// `Node<T>` belongs to.
    ///
    /// If the `value` is greather than the `Tree<T>`, the function is
    /// executed passing the `value` and the `right` side of the `Tree<T>`,
    /// otherwise is passed to the `left` side of the `Tree<T>`.
    ///
    /// The value is added when the `value` of the `Tree<T>` equals to `None`
    fn add_recursive(&mut self, node: Tree<T>, value: T) -> Tree<T> {
        if let Some(mut node) = node {
            if value <= node.value {
                node.left = self.add_recursive(node.left, value);
                return Some(node);
            }

            node.right = self.add_recursive(node.right, value);
            return Some(node);
        }

        Some(Node::boxed(value))
    }

    /// Searches the tree for the provided value
    pub fn find(&self, value: T) -> Option<T> {
        self.find_recursive(&self.root, value)
    }

    /// Walks the tree recursively looking for the node with the exact
    /// value as `value`
    fn find_recursive(&self, node: &Tree<T>, value: T) -> Option<T> {
        println!("Node: {:#?} - Value: {:#?}", node, value);
        if let Some(node) = node {
            return match node.value.cmp(&value) {
                std::cmp::Ordering::Less => self.find_recursive(&node.right, value),
                std::cmp::Ordering::Equal => Some(node.value.clone()),
                std::cmp::Ordering::Greater => self.find_recursive(&node.left, value),
            };
        }

        None
    }

    pub fn walk(&self, func: impl Fn(&T) -> ()) {
        self.walk_in_order(&self.root, &func);
    }

    fn walk_in_order(&self, node: &Tree<T>, func: &impl Fn(&T) -> ()) {
        if let Some(node) = node {
            self.walk_in_order(&node.left, func);
            func(&node.value);
            self.walk_in_order(&node.right, func);
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn creates_an_empty_bst() {
        let bst = BinarySearchTree::<u64>::new();

        assert!(bst.root.is_none());
        assert_eq!(bst.length, 0);
    }

    #[test]
    fn adds_values_to_a_bst() {
        let mut bst = BinarySearchTree::<u64>::new();

        assert!(bst.root.is_none());
        assert_eq!(bst.length, 0);

        bst.add(5);
        bst.add(3);
        bst.add(7);
        bst.add(6);
        bst.add(8);
        bst.add(1);
        bst.add(2);
        bst.add(9);

        assert!(bst.root.is_some());
        assert_eq!(bst.length, 8);
        assert_eq!(bst.root.unwrap().value, 5);
    }

    #[test]
    fn finds_value_in_bst() {
        let mut bst = BinarySearchTree::<u64>::new();

        bst.add(5);
        bst.add(10);
        bst.add(3);
        bst.add(4);

        let searches: [Option<u64>; 8] = [
            bst.find(11),
            bst.find(5),
            bst.find(1),
            bst.find(3),
            bst.find(10),
            bst.find(8),
            bst.find(4),
            bst.find(7),
        ];

        let results: [Option<u64>; 8] =
            [None, Some(5), None, Some(3), Some(10), None, Some(4), None];

        assert_eq!(searches, results);
    }

    #[test]
    fn walks_in_order_the_bst() {
        let mut bst = BinarySearchTree::<u64>::new();
        let touched_items: Rc<RefCell<Vec<u64>>> = Rc::new(RefCell::new(Vec::new()));
        let expect = vec![3, 4, 5, 7, 8, 10];

        bst.add(8);
        bst.add(7);
        bst.add(5);
        bst.add(10);
        bst.add(3);
        bst.add(4);

        bst.walk(|val| {
            let mut t = touched_items.borrow_mut();

            t.push(*val);
        });

        println!("{:?}", touched_items);

        let touched_items = touched_items.borrow();
        let touched_items = touched_items.clone();

        assert_eq!(touched_items, expect);
    }
}
