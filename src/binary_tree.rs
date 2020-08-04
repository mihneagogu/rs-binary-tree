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

    /// WARNING! If used on a tree you will modify, it will break the tree
    pub fn item_into(&mut self) -> Option<T> {
        self.item.take()
    }

    pub fn right_into(&mut self) -> Node<T> {
        self.right.take()
    }

    pub fn left_into(&mut self) -> Node<T> {
        self.left.take()
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
            None => None,
        }
    }

    fn get_right(&self) -> Node<T> {
        match &self.right {
            Some(right) => Some(Rc::clone(right)),
            None => None,
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
where
    T: Ord,
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
                    return remove_node(&mut *tree, parent, &child_is_left);
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

/// Does the removal of a node
/// PRE: tree is the child of parent
/// and tree.item == &item from remove_rc
fn remove_node<T>(tree: &mut BinaryTree<T>, parent: Node<T>, child_is_left: &bool) -> bool
where
    T: Ord,
{
    let (left_exists, right_exists) = (tree.left.is_some(), tree.right.is_some());

    if !left_exists && !right_exists {
        // No children
        if let Some(rc_parent) = parent {
            tree.item = None;
            let mut rc_parent = rc_parent.borrow_mut();
            // Make sure the parent reference is updated
            if *child_is_left {
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
    } else if left_exists && !right_exists {
        // only one left child, must replace tree with left node
        if let Some(rc_parent) = parent {
            // Substituting current node to its left child
            let mut rc_parent = rc_parent.borrow_mut();
            if *child_is_left {
                rc_parent.left = Some(Rc::clone(&tree.left.as_ref().unwrap()));
            } else {
                rc_parent.right = Some(Rc::clone(&tree.left.as_ref().unwrap()));
            }
            return true;
        } else {
            // tree is actually the root
            // so replace all of its data with left node data
            let left = Rc::clone(&tree.left.as_ref().unwrap());
            let mut left = left.borrow_mut();
            tree.item = left.item_into();
            tree.left = left.left.take();
            tree.right = left.right.take();
        }
    } else if right_exists && !left_exists {
        // only one right child, must replace tree with right node
        if let Some(rc_parent) = parent {
            // Substituting current node to its left child
            let mut rc_parent = rc_parent.borrow_mut();
            if *child_is_left {
                rc_parent.left = Some(Rc::clone(&tree.right.as_ref().unwrap()));
            } else {
                rc_parent.right = Some(Rc::clone(&tree.right.as_ref().unwrap()));
            }
            return true;
        } else {
            // tree is actually the root
            // so replace all of its data with right node data
            let right = Rc::clone(&tree.right.as_ref().unwrap());
            let mut right = right.borrow_mut();
            tree.item = right.item_into();
            tree.left = right.left.take();
            tree.right = right.right.take();
        }
    } else {
        return remove_node_two_children(tree, child_is_left);
    }
    false
}

/// Removes a node who has two children
/// PRE: tree has 2 children
fn remove_node_two_children<T>(tree: &mut BinaryTree<T>, child_is_left: &bool) -> bool
where
    T: Ord,
{
    // The ugly bit, tree has two children
    // so replace it with leftmost node of the right child
    let mut leftmost_node: Node<T> = tree.get_right();
    let mut leftmost_parent: Node<T> = None;
    let mut changed = false;

    // Go down the tree untill you get the leftmost node in the right subtree
    while let Some(rc_node) = leftmost_node {
        let lower_left = RefCell::borrow(&rc_node).get_left();
        if lower_left.is_none() {
            break;
        }
        leftmost_node = lower_left;
        leftmost_parent = Some(rc_node);
    }

    // Needed to get it from the parent, otherwise the compiler complains
    let leftmost_parent = leftmost_parent.unwrap();
    let leftmost_node = RefCell::borrow(&*leftmost_parent).get_left();
    // Swap the node to be deleted with the leftmost child of the right subtree
    let leftmost_node = leftmost_node.unwrap();
    let mut leftmost_node = leftmost_node.borrow_mut();
    tree.item = leftmost_node.item_into();
    leftmost_parent
        .borrow_mut()
        .set_left(leftmost_node.right_into());

    true
}
