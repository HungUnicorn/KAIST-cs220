//! Singly linked list.
//!
//! Consult <https://doc.rust-lang.org/book/ch15-01-box.html>.

use std::fmt::Debug;

/// Node of the list.
#[derive(Debug)]
pub struct Node<T: Debug> {
    /// Value of current node.
    pub value: T,

    /// Pointer to the next node. If it is `None`, there is no next node.
    pub next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    /// Creates a new node.
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

/// A singly-linked list.
#[derive(Debug)]
pub struct SinglyLinkedList<T: Debug> {
    /// Head node of the list. If it is `None`, the list is empty.
    head: Option<Node<T>>,
}

impl<T: Debug> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, value: T) {
        let node = Node {
            value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(node);
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Node { value, next: None });
        } else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(Node { value, next: None }))
        }
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|b| *b);
            Some(node.value)
        } else {
            None
        }
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            self.pop_front()
        } else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.as_ref().unwrap().next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            Some(current.next.take().unwrap().value)
        }
    }

    /// Create a new list from the given vector `vec`.
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut list = Self::new();
        for x in vec.into_iter().rev() {
            list.push_front(x);
        }
        list
    }

    /// Convert the current list into a vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(x) = self.pop_front() {
            vec.push(x);
        }
        vec
    }

    /// Return the length (i.e., number of nodes) of the list.
    pub fn length(&self) -> usize {
        let mut current = self.head.as_ref();
        let mut count = 0;
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref().map(|b| &**b)
        }
        count
    }

    /// Apply function `f` on every element of the list.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2]`, `f`: `|x| x + 1` ==> `[2, 3]`
    pub fn map<F: Fn(T) -> T>(self, f: F) -> Self {
        Self::from_vec(self.into_vec().into_iter().map(f).collect())
    }

    /// Apply given function `f` for each adjacent pair of elements in the list.
    /// If `self.length() < 2`, do nothing.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2, 3, 4]`, `f`: `|x, y| x + y`
    /// // each adjacent pair of elements: `(1, 2)`, `(2, 3)`, `(3, 4)`
    /// // apply `f` to each pair: `f(1, 2) == 3`, `f(2, 3) == 5`, `f(3, 4) == 7`
    /// ==> `[3, 5, 7]`
    pub fn pair_map<F: Fn(T, T) -> T>(self, f: F) -> Self
    where
        T: Clone,
    {
        if self.length() < 2 {
            return Self::new();
        }

        let vec = self.into_vec();
        let new_vec = vec
            .windows(2)
            .map(|w| f(w[0].clone(), w[1].clone()))
            .collect();
        Self::from_vec(new_vec)
    }
}

// A list of lists.
impl<T: Debug> SinglyLinkedList<SinglyLinkedList<T>> {
    /// Flatten the list of lists into a single list.
    ///
    /// # Examples
    /// `self`: `[[1, 2, 3], [4, 5, 6], [7, 8]]`
    /// ==> `[1, 2, 3, 4, 5, 6, 7, 8]`
    pub fn flatten(self) -> SinglyLinkedList<T> {
        SinglyLinkedList::from_vec(
            self.into_vec()
                .into_iter()
                .flat_map(|inner| inner.into_vec())
                .collect(),
        )
    }
}
