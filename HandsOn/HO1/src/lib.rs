// Hands-On 1 — CPC 2026/27
// Statement: https://pages.di.unipi.it/rossano/blog/2024/handson12425/

// Run with: cargo test -p handson1 ; cargo fmt -p handson1 ; cargo clippy -p handson1s
struct Node {
    //private attributes
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    //create a new node as a leaf (without childrens)
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
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
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
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
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    // ----------------------------

    fn node_key_by_id(&self, node_id: usize) -> u32 {
        self.nodes[node_id].key
    }

    // Exercise #1: a method that checks whether the tree is a Binary Search Tree.
    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0), (u32::MIN, u32::MAX))
    }

    fn rec_is_bst(&self, node_id: Option<usize>, admissible_interval: (u32, u32)) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let key = node.key;

            //check interval
            if key < admissible_interval.0 || key > admissible_interval.1 {
                return false;
            }

            //check left key
            if let Some(left_id) = node.id_left {
                let l_key = self.node_key_by_id(left_id);
                if l_key >= key {
                    return false;
                };
            }

            //check right key
            if let Some(right_id) = node.id_right {
                let r_key = self.node_key_by_id(right_id);
                if r_key < key {
                    return false;
                };
            }

            //check subtrees
            return self.rec_is_bst(node.id_left, (admissible_interval.0, key.saturating_sub(1)))
                && self.rec_is_bst(node.id_right, (key, admissible_interval.1));
        }

        true //base case 
    }

    // Exercise #2: a method that returns the sum of the maximum simple path
    //              connecting two leaves (Maximum Path Sum).

    fn dfs(&self, node_id: Option<usize>, global_max: &mut u32) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let key = node.key;

            //get left and right sub sums
            let l_max: u32 = self.dfs(node.id_left, global_max);
            let r_max: u32 = self.dfs(node.id_right, global_max);

            let local_sum = node.key + l_max + r_max;

            //update max when the node has both childrens
            if node.id_left.is_some() && node.id_right.is_some() {
                *global_max = (*global_max).max(local_sum);
            }

            return key + l_max.max(r_max);
        }

        0
    }

    //O(N) time, O(H) space (N = #nodes of the tree, H = heich of the tree)
    pub fn sum_max_simple_path(&self, node_id: Option<usize>) -> u32 {
        let mut global_max: u32 = 0;

        self.dfs(node_id, &mut global_max);
        global_max
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

    // ---------- Exercise #1: is_bst ----------

    #[test]
    fn single_node_is_bst() {
        assert!(Tree::with_root(42).is_bst());
    }

    #[test]
    fn valid_bst_is_accepted() {
        //        10
        //      5    22
        //       7  20
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 22, false);
        tree.add_node(1, 7, false);
        tree.add_node(2, 20, true);
        assert!(tree.is_bst());
    }

    #[test]
    fn wrong_direct_child_is_rejected() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 15, true);
        assert!(!tree.is_bst());

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, false);
        assert!(!tree.is_bst());
    }

    #[test]
    fn grandchild_violating_ancestor_is_rejected() {
        // 15 sta nel sottoalbero sinistro di 10: non e' un BST
        let mut tree = Tree::with_root(10);
        let a = tree.add_node(0, 5, true);
        tree.add_node(a, 15, false);
        assert!(!tree.is_bst());

        // 3 sta nel sottoalbero destro di 10: non e' un BST
        let mut tree = Tree::with_root(10);
        let b = tree.add_node(0, 20, false);
        tree.add_node(b, 3, true);
        assert!(!tree.is_bst());
    }

    #[test]
    fn duplicates_allowed_only_on_the_right() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 10, false);
        assert!(tree.is_bst());

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 10, true);
        assert!(!tree.is_bst());
    }

    #[test]
    fn bst_with_extreme_keys() {
        let mut tree = Tree::with_root(u32::MAX);
        let a = tree.add_node(0, 0, true);
        tree.add_node(a, u32::MAX - 1, false);
        assert!(tree.is_bst());
    }

    // ---------- Exercise #2: sum_max_simple_path ----------

    #[test]
    fn single_node_has_no_leaf_to_leaf_path() {
        assert_eq!(Tree::with_root(7).sum_max_simple_path(Some(0)), 0);
    }

    #[test]
    fn chain_has_only_one_leaf() {
        let mut tree = Tree::with_root(1);
        let a = tree.add_node(0, 2, true);
        tree.add_node(a, 3, false);
        assert_eq!(tree.sum_max_simple_path(Some(0)), 0);
    }

    #[test]
    fn root_with_two_leaves() {
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 2, true);
        tree.add_node(0, 3, false);
        assert_eq!(tree.sum_max_simple_path(Some(0)), 6);
    }

    #[test]
    fn path_cannot_end_at_a_node_with_one_child() {
        // 1000 ha un solo figlio: non e' una foglia, l'unico cammino e' 1+1+1
        let mut tree = Tree::with_root(1000);
        let a = tree.add_node(0, 1, true);
        tree.add_node(a, 1, true);
        tree.add_node(a, 1, false);
        assert_eq!(tree.sum_max_simple_path(Some(0)), 3);
    }

    #[test]
    fn single_child_node_is_traversed_down_to_the_leaf() {
        //        1
        //      2   3
        //            4      (4 ha solo il figlio sinistro 100)
        //          100
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 2, true);
        let r = tree.add_node(0, 3, false);
        let x = tree.add_node(r, 4, false);
        tree.add_node(x, 100, true);
        assert_eq!(tree.sum_max_simple_path(Some(0)), 2 + 1 + 3 + 4 + 100);
    }

    #[test]
    fn best_path_not_through_root() {
        //          1
        //       2      3
        //     50  60
        let mut tree = Tree::with_root(1);
        let l = tree.add_node(0, 2, true);
        tree.add_node(0, 3, false);
        tree.add_node(l, 50, true);
        tree.add_node(l, 60, false);
        assert_eq!(tree.sum_max_simple_path(Some(0)), 50 + 2 + 60);
    }
}
