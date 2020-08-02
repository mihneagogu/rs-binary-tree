use std::cmp::{Ord, Ordering};
use std::cell::RefCell;
use std::rc::Rc;

type Node<T> = Option<Rc<RefCell<BinaryTree<T>>>>;
pub struct BinaryTree<T> {
    left: Node<T>,
    right: Node<T>,
    item: Option<T>
}

impl<T> BinaryTree<T> 
where T: Ord // Implicitly requires PartialEq and Eq and PartialOrd
{
    /// Initializes an empty binary tree
    pub fn new() -> Self {
        Self { right: None, left: None, item: None }
    }

    /// Initializes a binary tree with the root containg the given item
    pub fn new_from(item: T) -> Self {
        Self { right: None, left: None, item: Some(item) }
    }

    /// Inserts the given item in the tree
    /// returns `true` if the insertion created a new node
    /// `false` otherwise
    pub fn insert(&mut self, item: T) -> bool { 
        match &self.item {
            Some(it) => { 
                if it == &item {
                    false
                } else {
                    match &item.cmp(&it) {
                        Ordering::Equal => unreachable!(),
                        Ordering::Less =>  {
                            let left = &self.left;
                            if left.is_none() {
                                let left_tree = BinaryTree::new_from(item);
                                let left_tree = Rc::new(RefCell::new(left_tree));
                                self.left = Some(left_tree);
                                return true;
                            }
                            let left = Rc::clone(&left.as_ref().unwrap());
                            return left.borrow_mut().insert(item);
                        }
                        Ordering::Greater => {
                            let right = &self.right;
                            if right.is_none() {
                                let right_tree = BinaryTree::new_from(item);
                                let right_tree = Rc::new(RefCell::new(right_tree));
                                self.right = Some(right_tree);
                                return true;
                            }
                            let right = Rc::clone(&right.as_ref().unwrap());
                            return right.borrow_mut().insert(item);
                        }

                    }
                }
            }
            None => {
                self.item = Some(item);
                true
            }
        }
    }

    /// Returns whether the tree contains the item T
    /// (or rather a copy of the item, since the tree takes ownership of the items)
    pub fn contains(&self, item: &T) -> bool {
        // TODO: Change to iterative from recursive
        let node = Some(self);
        if let Some(container) = node {
            match &container.item {
                Some(data) => {
                    return match item.cmp(data) {
                        Ordering::Equal => return true,
                        Ordering::Less =>  {
                            let left = &container.left;
                            if left.is_none() {
                                return false;
                            }
                            let left = Rc::clone(&left.as_ref().unwrap());
                            return left.borrow().contains(item);
                        }
                        Ordering::Greater => {
                            let right = &container.right;
                            if right.is_none() {
                                return false;
                            }
                            let right = Rc::clone(&right.as_ref().unwrap());
                            return right.borrow().contains(item);

                        }
                    };

                }
                None => return false
            }
        }
        false
    }
}
