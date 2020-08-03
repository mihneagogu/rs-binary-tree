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
        let removed = remove_rc(tree, &1);
        assert!(removed);
    }


}
