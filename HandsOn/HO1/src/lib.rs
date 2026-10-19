// Hands-On 1 — CPC 2026/27
// Statement: https://pages.di.unipi.it/rossano/blog/2024/handson12425/

// Run with: cargo test -p handson1 ; cargo fmt -p handson1 ; cargo clippy -p handson1
struct Node {
    //private attributes
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    //create a new node as a leaf (without children)
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

    // Exercise #1: a method that checks whether the tree is a Binary Search Tree.
    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0), (u32::MIN, u32::MAX))
    }

    fn rec_is_bst(&self, node_id: Option<usize>, admissible_interval: (u32, u32)) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let key = node.key;
            let low = admissible_interval.0;
            let high = admissible_interval.1;

            //check interval
            if key < low || key > high {
                return false;
            }

            //check subtrees
            if let Some(h) = key.checked_sub(1) {
                return self.rec_is_bst(node.id_left, (low, h))
                    && self.rec_is_bst(node.id_right, (key, high));
            } else {
                return node.id_left.is_none() && self.rec_is_bst(node.id_right, (key, high));
            }
        }

        true //base case
    }

    // Exercise #2: a method that returns the sum of the maximum simple path
    //              connecting two leaves (Maximum Path Sum).

    fn dfs(&self, node_id: Option<usize>, global_max: &mut Option<u32>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];
            let key = node.key;

            //get left and right sub sums
            let l_max: u32 = self.dfs(node.id_left, global_max);
            let r_max: u32 = self.dfs(node.id_right, global_max);

            let local_sum = key + l_max + r_max;

            //update max when the node has both children
            if node.id_left.is_some() && node.id_right.is_some() {
                *global_max = Some(global_max.map_or(local_sum, |v| v.max(local_sum)));
            }

            return key + l_max.max(r_max);
        }

        0
    }

    //O(N) time, O(H) space (N = #nodes of the tree, H = height of the tree)
    pub fn sum_max_simple_path(&self) -> Option<u32> {
        let mut global_max: Option<u32> = None;

        self.dfs(Some(0), &mut global_max);
        global_max
    }
}

// Tests generated with Claude (AI assistant).
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
        // 15 is in the left subtree of 10: not a BST
        let mut tree = Tree::with_root(10);
        let a = tree.add_node(0, 5, true);
        tree.add_node(a, 15, false);
        assert!(!tree.is_bst());

        // 3 is in the right subtree of 10: not a BST
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
        assert_eq!(Tree::with_root(7).sum_max_simple_path(), None);
    }

    #[test]
    fn chain_has_only_one_leaf() {
        let mut tree = Tree::with_root(1);
        let a = tree.add_node(0, 2, true);
        tree.add_node(a, 3, false);
        assert_eq!(tree.sum_max_simple_path(), None);
    }

    #[test]
    fn root_with_two_leaves() {
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 2, true);
        tree.add_node(0, 3, false);
        assert_eq!(tree.sum_max_simple_path(), Some(6));
    }

    #[test]
    fn path_cannot_end_at_a_node_with_one_child() {
        // 1000 has only one child, so it is not a leaf: the only path is 1+1+1
        let mut tree = Tree::with_root(1000);
        let a = tree.add_node(0, 1, true);
        tree.add_node(a, 1, true);
        tree.add_node(a, 1, false);
        assert_eq!(tree.sum_max_simple_path(), Some(3));
    }

    #[test]
    fn single_child_node_is_traversed_down_to_the_leaf() {
        //        1
        //      2   3
        //            4      (4 has only the left child 100)
        //          100
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 2, true);
        let r = tree.add_node(0, 3, false);
        let x = tree.add_node(r, 4, false);
        tree.add_node(x, 100, true);
        assert_eq!(tree.sum_max_simple_path(), Some(2 + 1 + 3 + 4 + 100));
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
        assert_eq!(tree.sum_max_simple_path(), Some(50 + 2 + 60));
    }

    // ---------- Randomized tests against brute force ----------

    // ---------- Helpers for randomized tests ----------

    /// Small deterministic pseudo-random generator (xorshift), no external crates.
    struct Rng(u64);

    impl Rng {
        fn below(&mut self, n: u64) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0 % n
        }
    }

    /// Random binary tree with `n` nodes and keys in `[0, max_key)`.
    fn random_tree(rng: &mut Rng, n: usize, max_key: u64) -> Tree {
        let mut tree = Tree::with_root(rng.below(max_key) as u32);
        while tree.nodes.len() < n {
            let parent = rng.below(tree.nodes.len() as u64) as usize;
            let is_left = rng.below(2) == 0;
            let free = if is_left {
                tree.nodes[parent].id_left.is_none()
            } else {
                tree.nodes[parent].id_right.is_none()
            };
            if free {
                tree.add_node(parent, rng.below(max_key) as u32, is_left);
            }
        }
        tree
    }

    /// Random BST (duplicates go right) built by repeated insertions.
    fn random_bst(rng: &mut Rng, n: usize, max_key: u64) -> Tree {
        let mut tree = Tree::with_root(rng.below(max_key) as u32);
        for _ in 1..n {
            let key = rng.below(max_key) as u32;
            let mut cur = 0;
            loop {
                let go_left = key < tree.nodes[cur].key;
                let next = if go_left {
                    tree.nodes[cur].id_left
                } else {
                    tree.nodes[cur].id_right
                };
                match next {
                    Some(child) => cur = child,
                    None => {
                        tree.add_node(cur, key, go_left);
                        break;
                    }
                }
            }
        }
        tree
    }

    fn subtree_keys(tree: &Tree, node_id: Option<usize>, out: &mut Vec<u32>) {
        if let Some(id) = node_id {
            out.push(tree.nodes[id].key);
            subtree_keys(tree, tree.nodes[id].id_left, out);
            subtree_keys(tree, tree.nodes[id].id_right, out);
        }
    }

    /// O(n^2) check straight from the definition.
    fn brute_is_bst(tree: &Tree) -> bool {
        (0..tree.nodes.len()).all(|id| {
            let key = tree.nodes[id].key;
            let (mut left, mut right) = (Vec::new(), Vec::new());
            subtree_keys(tree, tree.nodes[id].id_left, &mut left);
            subtree_keys(tree, tree.nodes[id].id_right, &mut right);
            left.iter().all(|&k| k < key) && right.iter().all(|&k| k >= key)
        })
    }

    /// O(n^3) check: tries every pair of leaves, the path goes through their lowest common ancestor.
    /// Returns None if there are fewer than two leaves.
    fn brute_max_path_sum(tree: &Tree) -> Option<u32> {
        let n = tree.nodes.len();
        let mut parent = vec![None; n];
        for id in 0..n {
            for child in [tree.nodes[id].id_left, tree.nodes[id].id_right]
                .into_iter()
                .flatten()
            {
                parent[child] = Some(id);
            }
        }
        let ancestors = |mut x: usize| {
            let mut v = vec![x];
            while let Some(p) = parent[x] {
                v.push(p);
                x = p;
            }
            v
        };
        let leaves: Vec<usize> = (0..n)
            .filter(|&id| tree.nodes[id].id_left.is_none() && tree.nodes[id].id_right.is_none())
            .collect();
        let mut best = None;
        for i in 0..leaves.len() {
            for j in i + 1..leaves.len() {
                let (a, b) = (ancestors(leaves[i]), ancestors(leaves[j]));
                let lca = *a.iter().find(|x| b.contains(x)).unwrap();
                let side = |v: &Vec<usize>| -> u32 {
                    v.iter()
                        .take_while(|&&x| x != lca)
                        .map(|&x| tree.nodes[x].key)
                        .sum()
                };
                let s = side(&a) + side(&b) + tree.nodes[lca].key;
                best = Some(best.map_or(s, |m: u32| m.max(s)));
            }
        }
        best
    }

    #[test]
    fn is_bst_matches_definition_on_random_trees() {
        let mut rng = Rng(12345);
        for round in 0..20_000 {
            let n = 1 + rng.below(10) as usize;
            let tree = if round % 2 == 0 {
                random_tree(&mut rng, n, if round % 4 == 0 { 6 } else { 1000 })
            } else {
                // mostly valid BSTs, sometimes with one key changed
                let mut t = random_bst(&mut rng, n, 20);
                if rng.below(2) == 0 {
                    let id = rng.below(t.nodes.len() as u64) as usize;
                    t.nodes[id].key = rng.below(20) as u32;
                }
                t
            };
            assert_eq!(tree.is_bst(), brute_is_bst(&tree));
        }
    }

    #[test]
    fn deep_chain_bst() {
        let mut tree = Tree::with_root(0);
        let mut cur = 0;
        for k in 1..10_000 {
            cur = tree.add_node(cur, k, false);
        }
        assert!(tree.is_bst());
    }

    #[test]
    fn key_zero_cannot_have_left_child() {
        let mut tree = Tree::with_root(0);
        tree.add_node(0, 0, true);
        assert!(!tree.is_bst());
    }

    #[test]
    fn zero_keys_are_distinguished_from_no_path() {
        let mut tree = Tree::with_root(0);
        tree.add_node(0, 0, true);
        tree.add_node(0, 0, false);
        assert_eq!(tree.sum_max_simple_path(), Some(0));
    }

    #[test]
    fn example_tree_of_the_statement_path() {
        //        10
        //      5    22
        //       7  20        leaves 7 and 20: 7 + 5 + 10 + 22 + 20
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 22, false);
        tree.add_node(1, 7, false);
        tree.add_node(2, 20, true);
        assert_eq!(tree.sum_max_simple_path(), Some(64));
    }

    #[test]
    fn max_path_matches_brute_force_on_random_trees() {
        let mut rng = Rng(777);
        for round in 0..20_000 {
            let n = 1 + rng.below(12) as usize;
            let max_key = if round % 2 == 0 { 5 } else { 1000 };
            let tree = random_tree(&mut rng, n, max_key);
            assert_eq!(tree.sum_max_simple_path(), brute_max_path_sum(&tree));
        }
    }
}
