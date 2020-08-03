use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::{Ord, Ordering};
use std::rc::Rc;

fn rc_ref_new<T>(item: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(item))
}

type Node<T> = Option<Rc<RefCell<BinaryTree<T>>>>;
pub struct BinaryTree<T> {
    left: Node<T>,
    right: Node<T>,
    item: Option<T>,
}

impl<T> BinaryTree<T>
where
    T: Ord, // Implicitly requires PartialEq and Eq and PartialOrd
{
    /// Initializes an empty binary tree
    pub fn new() -> Self {
        Self {
            right: None,
            left: None,
            item: None,
        }
    }

    pub fn remove(&mut self, item: &T) -> bool {
        // TODO: Implement like remove_rc
        unimplemented!();
    }

    pub fn has_item(&self) -> bool {
        self.item.is_some()
    }

    fn set_left(&mut self, left: Node<T>) {
        self.left = left;
    }

    fn get_left(&self) -> Node<T> {
        match &self.left {
            Some(left) => Some(Rc::clone(left)),
            None => None
        }
    }

    fn get_right(&self) -> Node<T> {
        match &self.right {
            Some(right) => Some(Rc::clone(right)),
            None => None
        }
    }

    fn set_right(&mut self, right: Node<T>) {
        self.right = right;
    }
    /// Initializes a binary tree with the root containg the given item
    pub fn new_from(item: T) -> Self {
        Self {
            right: None,
            left: None,
            item: Some(item),
        }
    }

    /// Inserts the given item in the tree
    /// returns `true` if the insertion created a new node
    /// `false` otherwise
    pub fn insert(&mut self, item: T) -> bool {
        // TODO: Change to iterative from recursive
        match &self.item {
            Some(it) => {
                if it == &item {
                    false
                } else {
                    match &item.cmp(&it) {
                        Ordering::Equal => unreachable!(),
                        Ordering::Less => {
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
                        Ordering::Less => {
                            let left = &container.left;
                            if left.is_none() {
                                return false;
                            }
                            let left = Rc::clone(&left.as_ref().unwrap());
                            return left.as_ref().borrow().contains(item);
                        }
                        Ordering::Greater => {
                            let right = &container.right;
                            if right.is_none() {
                                return false;
                            }
                            let right = Rc::clone(&right.as_ref().unwrap());
                            return right.as_ref().borrow().contains(item);
                        }
                    };
                }
                None => return false,
            }
        }
        false
    }
}

/// Takes an Rc Refcell to a tree, which it consumes and
/// removes the given item from the Tree,
/// returning whether the tree has changed or not
pub fn remove_rc<T>(root: Rc<RefCell<BinaryTree<T>>>, item: &T) -> bool 
where T: Ord
{
    let mut parent: Node<T> = None;
    let mut child: Node<T> = Some(root);
    let mut child_is_left = false;

    while let Some(node) = child {
        let tree = Rc::clone(&node);
        let mut tree = tree.borrow_mut();
        if !tree.has_item() {
            // Reached a dead end
            return false;
        }
        match &tree.item {
            Some(data) => {
                if data == item {
                    // found value, deleting
                    let (left_exists, right_exists) = (tree.left.is_some(), tree.right.is_some());
                    if !left_exists && !right_exists {
                        // No children
                        if let Some(rc_parent) = parent {
                            tree.item = None;
                            let mut rc_parent = rc_parent.borrow_mut();
                            // Make sure the parent reference is updated
                            if child_is_left {
                                rc_parent.left = None;
                            } else {
                                rc_parent.right = None;
                            }
                            return true;
                        } else {
                            // must still be at root, since parent is None
                            // then node is actually root, but was moved
                            tree.item = None;
                            return true;
                        }
                    }
                } else {
                    // Haven't found value yet, going down the tree
                    if item > data {
                        child = tree.get_right();
                        child_is_left = false; // right child of parent
                        parent = Some(node);
                        continue;
                    } else
                    /* item < data */
                    {
                        child = tree.get_left();
                        child_is_left = true;
                        parent = Some(node);
                        continue;
                    }
                }
            }
            None => unreachable!(),
        }
        child = None;
        parent = None;
    }

    false
}
