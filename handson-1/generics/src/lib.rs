pub struct Node <T>{
    key: T,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl<T> Node<T> {
    fn new(key: T) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree <T>{
    nodes: Vec<Node<T>>,
}

///T needs to be bounded by some traits
/// - Ord: permits to have the comparison operations
/// - Add<Output=T>: permits to have the addition operation and 
///                 guarantees that the output of an operation between T operands is always of type T
/// - Default: to have a default value for whatever T is
/// - Copy: instead of moving ownership, this creates a copy of the value (copy semantics)
/// Note: Ord trait doesn't cover floating number because NaN can occur and it is not handled  
impl<T: Ord + std::ops::Add<Output=T> + Default + Copy> Tree<T> {
    pub fn with_root(key: T) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: T, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> T {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> T {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            // Copy trait is essential here. Without the copy trait,
            // the ownership of node.key is passed to the addition operation,
            // in that way the Node instance has no longer the ownership and 
            // node.key is no longer accessible.
            return sum_left + sum_right + node.key;
        }
        T::default()
    }

    /// Returns if the tree is a binary search tree or not
    pub fn is_bst(&self) -> bool {
        self.rec_bst(Some(0), &mut None)
    }

    /// Auxiliary function to check if the tree is a binary search tree with the in-order visit.
    /// The parameters are the current node and a reference to value of the previous node
    /// An alternative is to use a reference to the index of the previous node.
    fn rec_bst(&self, current_node: Option<usize>, previous_node: &mut Option<T>) -> bool {
        if let Some(current_id) = current_node {
            assert!(current_id < self.nodes.len(), "Node id is out of range");

            let tree = &self.nodes;

            if !self.rec_bst(tree[current_id].id_left, previous_node) {
                return false;
            }

            if let Some(previous_id) = previous_node {
                if *previous_id > tree[current_id].key {
                    return false;
                }
            }

            *previous_node = Some(tree[current_id].key);

            return self.rec_bst(tree[current_id].id_right, previous_node);
        }
        true
    }

    /// Returns the maximum path sum. If the tree is empty, it returns None.
    pub fn max_path_sum(&self) -> Option<T> {
        self.rec_max_path_sum(Some(0)).0
    }

    fn rec_max_path_sum(&self, current_node: Option<usize>) -> (Option<T>, Option<T>) {
        if let Some(current_id) = current_node {
            assert!(current_id < self.nodes.len(), "Node id is out of range");

            let node = &self.nodes[current_id];
            let (left_max, left_sum) = self.rec_max_path_sum(node.id_left);
            let (right_max, right_sum) = self.rec_max_path_sum(node.id_right);

            let path_sum = Some(node.key + left_sum.unwrap_or(T::default()).max(right_sum.unwrap_or(T::default())));

            // Compute the maximum path sum so far, comparing maximum sum of left path,
            // maximum sum of right path and the sum of the actual path.
            // Generate one iterator on the Option left_max and chain with:
            // - the iterator on right_sum (generated by chain() itself)
            // - the iterator on key + left_sum + right_sum (generated always by chain itself)
            // Then compute the max on the resultant iterator.
            let max_sum = left_max
                .into_iter()
                .chain(right_max)
                .chain(Some(
                    node.key + left_sum.unwrap_or(T::default()) + right_sum.unwrap_or(T::default()),
                ))
                .max();

            return (max_sum, path_sum);
        }
        (None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_sum_balanced_tree() {
        // Create the root of the BST
        let mut tree = Tree::with_root(40);

        // Add nodes to create the structure
        tree.add_node(0, 30, true); // 30 is the left child of 40
        tree.add_node(0, 50, false); // 50 is the right child of 40

        tree.add_node(1, 25, true); // 25 is the left child of 30
        tree.add_node(1, 35, false); // 35 is the right child of 30

        tree.add_node(2, 45, true); // 45 is the left child of 50
        tree.add_node(2, 60, false); // 60 is the right child of 50

        //       40
        //     /    \
        //   30      50
        //  / \     / \
        // 25 35  45  60

        assert_eq!(tree.sum(), 285);
    }

    #[test]
    fn test_sum_with_all_zeros() {
        let mut tree = Tree::with_root(0);

        tree.add_node(0, 0, true); // id 1
        tree.add_node(0, 0, false); // id 2

        // Expected sum = 0 + 0 + 0 = 0
        assert_eq!(tree.sum(), 0);
    }

    #[test]
    fn test_sum_with_unbalanced_tree() {
        let mut tree = Tree::with_root(1);

        tree.add_node(0, 2, false); // id 1
        tree.add_node(1, 3, false); // id 2
        tree.add_node(2, 4, false); // id 3

        // Expected sum = 1 + 2 + 3 + 4 = 10
        assert_eq!(tree.sum(), 10);
    }

    #[test]
    fn test_is_bst_single_node() {
        // Create a tree with a single root node
        let tree = Tree::with_root(10);

        // A single-node tree is always a valid BST
        assert!(tree.is_bst());
    }

    #[test]
    fn test_is_bst_valid_balanced_tree() {
        // Create the root of the BST
        let mut tree = Tree::with_root(40);

        // Add nodes to create the structure
        tree.add_node(0, 30, true); // 30 is the left child of 40
        tree.add_node(0, 50, false); // 50 is the right child of 40

        tree.add_node(1, 25, true); // 25 is the left child of 30
        tree.add_node(1, 35, false); // 35 is the right child of 30

        tree.add_node(2, 45, true); // 45 is the left child of 50
        tree.add_node(2, 60, false); // 60 is the right child of 50

        //       40
        //     /    \
        //   30      50
        //  / \     / \
        // 25 35  45  60

        assert!(tree.is_bst());
    }

    #[test]
    fn test_is_bst_valid_unbalanced_tree() {
        // Create a tree with root 10
        let mut tree = Tree::with_root(10);

        // Add nodes to form a valid BST
        tree.add_node(0, 5, true); // id 1: left of root (10 > 5)

        //       10
        //      /
        //     5
        assert!(tree.is_bst());

        // Add more nodes, maintaining the BST property
        tree.add_node(1, 3, true); // id 3
        tree.add_node(2, 1, true); // id 5

        //       10
        //      /
        //     5
        //    /
        //   3
        //  /
        // 1

        // The tree should still be a valid BST
        assert!(tree.is_bst());
    }

    #[test]
    fn test_is_bst_invalid_tree_left() {
        // Create a tree with root 10
        let mut tree = Tree::with_root(10);

        // Add nodes to violate BST property on the left
        tree.add_node(0, 12, true); // id 1

        //     10
        //    /
        //   12

        // Violates BST property since 12 > 10
        assert!(!tree.is_bst());
    }

    #[test]
    fn test_is_bst_invalid_tree_right() {
        // Create a tree with root 10
        let mut tree = Tree::with_root(10);

        // Add nodes to violate BST property on the right
        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 8, false); // id 2

        //     10
        //    / \
        //   5   8

        // Violates BST property since 8 < 10
        assert!(!tree.is_bst());
    }

    #[test]
    fn test_max_sum_single_node_tree() {
        let tree = Tree::with_root(5);
        // Only one node, so max path sum is 5
        assert_eq!(tree.max_path_sum(), Some(5));
    }

    #[test]
    fn test_max_sum_tree_with_zero() {
        let mut tree = Tree::with_root(0);
        tree.add_node(0, 2, true); // Left of root
        tree.add_node(0, 3, false); // Right of root

        //     0
        //    / \
        //   2   3
        
        // Max path sum should from 2 -> 0 -> 3
        assert_eq!(tree.max_path_sum(), Some(5));
    }

    #[test]
    fn test_max_sum_unbalanced_tree() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true); // Left of root
        tree.add_node(1, 3, true); // Left of 5
        tree.add_node(2, 4, false); // Right of 3

        //       10
        //      /
        //     5
        //    /
        //   3
        //    \
        //     4

        // Max path should be from 4 (left leaf) -> 3 -> 5 -> 10
        assert_eq!(tree.max_path_sum(), Some(22));
    }

    #[test]
    fn test_max_sum_leaf_to_leaf_without_root() {
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 1, true); // Left of root
        tree.add_node(0, 1, false); // Right of root
        tree.add_node(1, 1, true);
        tree.add_node(1, 1, false);
        tree.add_node(4, 20, true);
        tree.add_node(4, 25, false);

        //        1
        //      /  \
        //     1    1
        //    / \
        //   1   1
        //      / \
        //    20  25

        // Max path should be from 20 -> 1 -> 25
        assert_eq!(tree.max_path_sum(), Some(46));
    }

    #[test]
    fn test_max_sum_balanced_tree() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 15, true); // Left of root
        tree.add_node(0, 20, false); // Right of root
        tree.add_node(1, 25, true); // Left of 15
        tree.add_node(1, 30, false); // Right of 15
        tree.add_node(2, 25, true); // Left of 15
        tree.add_node(2, 30, false); // Right of 15

        //         10
        //      /     \
        //     15      20
        //    / \     /  \
        //   25  30  25  30

        // Max path should be: 30 -> 15 -> 10 -> 20 -> 30
        assert_eq!(tree.max_path_sum(), Some(105));
    }
}
