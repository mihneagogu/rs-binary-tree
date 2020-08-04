mod binary_tree;
use binary_tree::*;
fn main() {
}

#[cfg(test)]
mod tests {
    use crate::binary_tree as btree;
    use btree::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn tree_all() {
        let mut tree = BinaryTree::new_from(1);
        assert!(tree.contains(&1));

        tree.insert(2);
        assert!(tree.contains(&2));

        tree.insert(4);
        assert!(tree.contains(&4));

        tree.insert(3);
        assert!(tree.contains(&3));

        tree.insert(-1);
        assert!(tree.contains(&-1));

    }

    #[test]
    fn tree_already_in() {
        let mut tree = BinaryTree::new();
        tree.insert(0);
        assert!(!tree.insert(0));
    }

    #[test]
    fn tree_empty_remove() {
        let tree = BinaryTree::new();
        let tree = Rc::new(RefCell::new(tree));
        let removed = remove_rc(tree, &1);
        assert!(!removed);
    }

    #[test]
    fn tree_remove() {
        let mut tree = BinaryTree::new_from(2);
        tree.insert(3);
        tree.insert(1);
        let tree = Rc::new(RefCell::new(tree));
        let tree_clone = Rc::clone(&tree);

        let removed = remove_rc(tree_clone, &1);
        let tree = tree.borrow_mut();

        assert!(removed);
        assert!(!tree.contains(&1));
        assert!(tree.contains(&2));
        assert!(tree.contains(&3));
    }

    #[test]
    fn tree_remove_2_children() {
        let mut tree = BinaryTree::new_from(15);
        tree.insert(17);
        tree.insert(11);
        tree.insert(14);
        tree.insert(9);
        tree.insert(12);
        tree.insert(13);

        assert!(tree.contains(&17));
        assert!(tree.contains(&11));
        assert!(tree.contains(&14));
        assert!(tree.contains(&14));
        assert!(tree.contains(&13));
        assert!(tree.contains(&9));

        let rc_tree = Rc::new(RefCell::new(tree));
        let tree_clone = Rc::clone(&rc_tree);
        assert!(remove_rc(tree_clone, &11));

        let tree_ref_mut = rc_tree.borrow_mut();
        assert!(!tree_ref_mut.contains(&11));

        assert!(tree_ref_mut.contains(&17));
        assert!(tree_ref_mut.contains(&14));
        assert!(tree_ref_mut.contains(&14));
        assert!(tree_ref_mut.contains(&13));
        assert!(tree_ref_mut.contains(&9));

    }

    #[test]
    fn tree_remove_1_child_left() {
        let mut tree = BinaryTree::new_from(15);
        tree.insert(17);
        tree.insert(11);
        tree.insert(14);
        tree.insert(9);
        tree.insert(12);

        let rc_tree = Rc::new(RefCell::new(tree));
        let tree_clone = Rc::clone(&rc_tree);

        assert!(remove_rc(tree_clone, &14));

        let tree_ref_mut = rc_tree.borrow_mut();
        assert!(!tree_ref_mut.contains(&14));
        assert!(tree_ref_mut.contains(&12));

        assert!(tree_ref_mut.contains(&17));
        assert!(tree_ref_mut.contains(&11));
        assert!(tree_ref_mut.contains(&9));

    }

    #[test]
    fn tree_remove_1_child_right() {
        let mut tree = BinaryTree::new_from(15);
        tree.insert(17);
        tree.insert(11);
        tree.insert(9);
        tree.insert(12);
        tree.insert(14);

        let rc_tree = Rc::new(RefCell::new(tree));
        let tree_clone = Rc::clone(&rc_tree);

        assert!(remove_rc(tree_clone, &12));

        let tree_ref_mut = rc_tree.borrow_mut();
        assert!(!tree_ref_mut.contains(&12));
        assert!(tree_ref_mut.contains(&14));

        assert!(tree_ref_mut.contains(&17));
        assert!(tree_ref_mut.contains(&11));
        assert!(tree_ref_mut.contains(&9));

    }


}
