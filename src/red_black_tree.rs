//! Red-Black Tree
//!
//! A Red-Black Tree is a self-balancing tree which
//! implies a color based logic to balance the tree
//! on every insertion.
//!
//! Under the hood this tree implements a BST but
//! extra logic is also applied to keep such tree
//! efficient when searching.
//!
//! - Each node must hold a color, either red or black
//! - The root node should be a black node
//! - Every leaf from the tree is considered black nodes
//! - A red node can only have black child nodes
//! - Any path from the root to the leaves of the tree
//! should have the same number of black nodes
use std::cell::RefCell;
use std::cmp::{Ord, PartialEq};
use std::fmt::Debug;
use std::rc::Rc;

/// A BareTree contains the actual value of a Node consumed in the
/// Red-Black Tree.
///
/// The Node is wrapped by a `ReferenceCounted` pointer and also a
/// `RefCell` cell, both allows the tree to support multiple ownership
/// and also support runtime checked mutability
type BareTree<T: Clone + Debug + Ord + PartialEq> = Rc<RefCell<Node<T>>>;

/// Wraps a `BareTree` into an `Option<T>`, given that a `Tree` may and
/// may not have a `Node` (The path is empty or it was removed before)
type Tree<T: Clone + Debug + Ord + PartialEq> = Option<BareTree<T>>;

/// A Red-Black Tree `Node` contains the actual value and also holds
/// a color which is used to balance the tree on every insertion.
///
/// The Node also holds its childrens such as `left` and `right` which
/// are also `Tree`s.
pub struct Node<T>
where
    T: Clone + Debug + Ord + PartialEq,
{
    color: Color,
    value: T,
    left: Tree<T>,
    right: Tree<T>,
    parent: Tree<T>,
}

impl<T> PartialEq for Node<T>
where
    T: Clone + Debug + Ord + PartialEq,
{
    fn eq(&self, other: &Node<T>) -> bool {
        self.value == other.value
    }
}

impl<T> Node<T>
where
    T: Clone + Debug + Ord + PartialEq,
{
    pub fn new(value: T) -> Tree<T> {
        Some(Rc::new(RefCell::new(Node {
            value,
            color: Color::Red,
            parent: None,
            left: None,
            right: None,
        })))
    }
}

/// A Color used during rebalance producedure to mark `Node`s
pub enum Color {
    Black,
    Red,
}

/// Rotation direction used to define the direction in which the rotation is
/// being made during the rebalance of the Tree
pub enum Rotation {
    Left,
    Right,
}

pub struct RedBlackTree<T>
where
    T: Clone + Debug + Ord + PartialEq,
{
    length: usize,
    root: Tree<T>,
}
