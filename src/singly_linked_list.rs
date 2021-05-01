//! Singly Linked List
//!
//! A Data Structure where each `Node` holds a pointer to the next `Node`
use std::cell::RefCell;
use std::rc::Rc;

/// A `Link` to the next `Node`
pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// A `List` `Node` holding a value of type `T` and the `Link` to the
/// next `Node` which could be `None`
#[derive(Debug)]
pub struct Node<T> {
    next: Link<T>,
    value: T,
}

impl<T> Node<T>
where
    T: std::fmt::Debug,
{
    /// Creates a new `Node` with the provided value
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { next: None, value }))
    }

    /// Retrieves a reference to this `Node`'s value
    pub fn peek_value(&self) -> &T {
        &self.value
    }
}

/// A collection of `Node`s which the total count of `Node`s (`length`), `head`
/// the firt `Node` from the list and the `tail`, the very last `Node` from the
/// list
#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: u32,
}

impl<T> SinglyLinkedList<T>
where
    T: std::fmt::Debug,
{
    /// Creates a new `SinglyLinkedList` with neither `head` and `tail`
    /// (of length 0)
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Retrieves the lenght of this list which is the total count of `Node`s
    pub fn len(&self) -> u32 {
        self.length
    }

    /// Appends a value to the end (tail) of the `List`
    pub fn append(&mut self, value: T) {
        let node = Node::new(value);

        match self.tail.take() {
            Some(current_tail_node) => current_tail_node.borrow_mut().next = Some(Rc::clone(&node)),
            None => self.head = Some(Rc::clone(&node)),
        }

        self.length += 1;
        self.tail = Some(Rc::clone(&node));
    }

    /// Pops a value from the start (head) of the `List`
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            self.length -= 1;

            Rc::try_unwrap(head)
                .ok()
                .expect("Failed to retrieve node from list")
                .into_inner()
                .value
        })
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_an_empty_singly_linked_list() {
        let list = SinglyLinkedList::<String>::new();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.length, 0);
    }

    #[test]
    fn appends_a_value_to_the_list() {
        let mut list = SinglyLinkedList::<String>::new();
        let initial_length = list.len();

        list.append(String::from("MyValue"));

        let head_node = list.head.clone();
        let head_node_value = head_node.unwrap();
        let head_node_value = head_node_value.borrow();
        let head_node_value = head_node_value.peek_value();

        assert_eq!(String::from("MyValue"), *head_node_value);
        assert_eq!(initial_length, 0);
        assert_eq!(list.length, 1);
    }

    #[test]
    fn retrieves_the_list_length() {
        let mut list = SinglyLinkedList::<String>::new();

        list.append(String::from("MyValue"));

        assert_eq!(list.len(), 1);
        assert_eq!(list.length, 1);

        list.append(String::from("MySecondValue"));

        assert_eq!(list.len(), 2);
        assert_eq!(list.length, 2);

        list.append(String::from("MyLastValue"));

        assert_eq!(list.len(), 3);
        assert_eq!(list.length, 3);
    }

    #[test]
    fn props_a_node_from_the_list() {
        let mut list = SinglyLinkedList::<String>::new();

        list.append(String::from("MyValue"));
        list.append(String::from("MySecondValue"));
        list.append(String::from("MyLastValue"));

        let first_element = list.pop().unwrap();
        assert_eq!(list.len(), 2);

        let second_element = list.pop().unwrap();
        assert_eq!(list.len(), 1);

        let third_element = list.pop().unwrap();
        assert_eq!(list.len(), 0);

        assert_eq!(first_element, String::from("MyValue"));
        assert_eq!(second_element, String::from("MySecondValue"));
        assert_eq!(third_element, String::from("MyLastValue"));
        assert_eq!(list.len(), 0);
    }
}
