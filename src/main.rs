mod binary_tree;
use binary_tree::*;
fn main() {
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::BinaryTree;

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


}
