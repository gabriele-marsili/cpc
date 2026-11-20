//! Prof's code, copied VERBATIM from his 2026 notes "Segment Tree"
//! (https://pages.di.unipi.it/rossano/blog/2026/segmenttree/, 14/11/2026): `SegmentTree` (needed by
//! `LazySegmentTree::from_slice`), `LazySegmentTree` (section "Range Updates with Lazy Propagation") and
//! `PersistentSegmentTree` (section "Persistent Segment Tree", attempt #3, path copying).
//! Only this header and the tests are mine [extra].
//! Running example of the notes: A = [5, 1, 3, 1, 4, 7, 2, 1].

#[derive(Debug)]
pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    pub fn from_slice(a: &[i64]) -> Self {
        assert!(!a.is_empty());
        let n = a.len();
        let mut st = Self { n, tree: vec![0; 4 * n] };
        st.build(a, 0, 0, n - 1);
        st
    }

    pub fn len(&self) -> usize {
        self.n
    }

    /// A[i] += v
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.n);
        self.add_rec(0, 0, self.n - 1, i, v);
    }

    /// A[i] + A[i+1] + ... + A[j]
    pub fn range_sum(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j && j < self.n);
        self.range_sum_rec(0, 0, self.n - 1, i, j)
    }

    // Node u covers [l, r]; its children are 2u + 1 and 2u + 2.
    fn build(&mut self, a: &[i64], u: usize, l: usize, r: usize) {
        if l == r {
            self.tree[u] = a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(a, 2 * u + 1, l, m);
        self.build(a, 2 * u + 2, m + 1, r);
        self.tree[u] = self.tree[2 * u + 1] + self.tree[2 * u + 2]; // combine
    }

    fn add_rec(&mut self, u: usize, l: usize, r: usize, i: usize, v: i64) {
        if l == r {
            self.tree[u] += v; // the leaf of position i
            return;
        }
        let m = (l + r) / 2;
        if i <= m {
            self.add_rec(2 * u + 1, l, m, i, v);
        } else {
            self.add_rec(2 * u + 2, m + 1, r, i, v);
        }
        self.tree[u] = self.tree[2 * u + 1] + self.tree[2 * u + 2]; // combine
    }

    fn range_sum_rec(&self, u: usize, l: usize, r: usize, i: usize, j: usize) -> i64 {
        if i <= l && r <= j {
            return self.tree[u]; // total overlap
        }
        if r < i || j < l {
            return 0; // no overlap
        }
        // partial overlap
        let m = (l + r) / 2;
        self.range_sum_rec(2 * u + 1, l, m, i, j) + self.range_sum_rec(2 * u + 2, m + 1, r, i, j)
    }
}

/// range_update (add v to every entry in [i, j]) and range_sum.
/// Invariant: tree[u] is always the correct sum of the range of u;
/// lazy[u] is an addition still to be applied to every entry below u.
#[derive(Debug)]
pub struct LazySegmentTree {
    n: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    pub fn from_slice(a: &[i64]) -> Self {
        let st = SegmentTree::from_slice(a);
        Self { n: st.n, lazy: vec![0; st.tree.len()], tree: st.tree }
    }

    pub fn range_update(&mut self, i: usize, j: usize, v: i64) {
        assert!(i <= j && j < self.n);
        self.range_update_rec(0, 0, self.n - 1, i, j, v);
    }

    // &mut self: a query may push lazy values down, so it changes the tree.
    pub fn range_sum(&mut self, i: usize, j: usize) -> i64 {
        assert!(i <= j && j < self.n);
        self.range_sum_rec(0, 0, self.n - 1, i, j)
    }

    // Add v to every entry below u, which covers [l, r].
    fn apply(&mut self, u: usize, l: usize, r: usize, v: i64) {
        self.tree[u] += v * (r - l + 1) as i64;
        if l < r {
            self.lazy[u] += v; // a leaf has nothing below it
        }
    }

    // Move the pending addition of u to its children.
    fn push(&mut self, u: usize, l: usize, r: usize) {
        if self.lazy[u] != 0 {
            let m = (l + r) / 2;
            let v = std::mem::take(&mut self.lazy[u]);
            self.apply(2 * u + 1, l, m, v);
            self.apply(2 * u + 2, m + 1, r, v);
        }
    }

    fn range_update_rec(&mut self, u: usize, l: usize, r: usize, i: usize, j: usize, v: i64) {
        if i <= l && r <= j {
            self.apply(u, l, r, v); // total overlap
            return;
        }
        if r < i || j < l {
            return; // no overlap
        }
        self.push(u, l, r); // partial overlap
        let m = (l + r) / 2;
        self.range_update_rec(2 * u + 1, l, m, i, j, v);
        self.range_update_rec(2 * u + 2, m + 1, r, i, j, v);
        self.tree[u] = self.tree[2 * u + 1] + self.tree[2 * u + 2];
    }

    fn range_sum_rec(&mut self, u: usize, l: usize, r: usize, i: usize, j: usize) -> i64 {
        if i <= l && r <= j {
            return self.tree[u];
        }
        if r < i || j < l {
            return 0;
        }
        self.push(u, l, r);
        let m = (l + r) / 2;
        self.range_sum_rec(2 * u + 1, l, m, i, j) + self.range_sum_rec(2 * u + 2, m + 1, r, i, j)
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    sum: i64,
    left: usize, // index in `nodes`; unused for leaves
    right: usize,
}

/// Every `add` creates a new version; version 0 is the initial array.
#[derive(Debug)]
pub struct PersistentSegmentTree {
    n: usize,
    nodes: Vec<Node>,
    roots: Vec<usize>, // roots[t] is the root of version t
}

impl PersistentSegmentTree {
    pub fn from_slice(a: &[i64]) -> Self {
        assert!(!a.is_empty());
        let mut pst = Self { n: a.len(), nodes: Vec::with_capacity(2 * a.len()), roots: vec![] };
        let root = pst.build(a, 0, a.len() - 1);
        pst.roots.push(root);
        pst
    }

    /// Number of versions (updates performed + 1).
    pub fn versions(&self) -> usize {
        self.roots.len()
    }

    /// A[i] += v on the latest version; returns the new timestamp.
    pub fn add(&mut self, i: usize, v: i64) -> usize {
        assert!(i < self.n);
        let last = *self.roots.last().unwrap();
        let root = self.add_rec(last, 0, self.n - 1, i, v);
        self.roots.push(root);
        self.roots.len() - 1
    }

    /// A[i] + ... + A[j] as it was at time t.
    pub fn range_sum(&self, i: usize, j: usize, t: usize) -> i64 {
        assert!(i <= j && j < self.n && t < self.roots.len());
        self.range_sum_rec(self.roots[t], 0, self.n - 1, i, j)
    }

    fn new_node(&mut self, sum: i64, left: usize, right: usize) -> usize {
        self.nodes.push(Node { sum, left, right });
        self.nodes.len() - 1
    }

    fn build(&mut self, a: &[i64], l: usize, r: usize) -> usize {
        if l == r {
            return self.new_node(a[l], 0, 0);
        }
        let m = (l + r) / 2;
        let left = self.build(a, l, m);
        let right = self.build(a, m + 1, r);
        let sum = self.nodes[left].sum + self.nodes[right].sum;
        self.new_node(sum, left, right)
    }

    // Path copying: a new copy of every node on the root-to-leaf path of i;
    // the child off the path is shared with the previous version.
    fn add_rec(&mut self, u: usize, l: usize, r: usize, i: usize, v: i64) -> usize {
        let Node { sum, mut left, mut right } = self.nodes[u];
        if l == r {
            return self.new_node(sum + v, 0, 0);
        }
        let m = (l + r) / 2;
        if i <= m {
            left = self.add_rec(left, l, m, i, v);
        } else {
            right = self.add_rec(right, m + 1, r, i, v);
        }
        let sum = self.nodes[left].sum + self.nodes[right].sum; // combine
        self.new_node(sum, left, right)
    }

    fn range_sum_rec(&self, u: usize, l: usize, r: usize, i: usize, j: usize) -> i64 {
        if i <= l && r <= j {
            return self.nodes[u].sum;
        }
        if r < i || j < l {
            return 0;
        }
        let m = (l + r) / 2;
        let node = self.nodes[u];
        self.range_sum_rec(node.left, l, m, i, j) + self.range_sum_rec(node.right, m + 1, r, i, j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    const A: [i64; 8] = [5, 1, 3, 1, 4, 7, 2, 1];

    #[test]
    fn lazy_notes_example() {
        let mut t = LazySegmentTree::from_slice(&A);
        t.range_update(0, 4, 20);
        // [0,3] has a total overlap: 10 + 4*20 = 90, lazy +20; leaf 4 -> 24
        assert_eq!(t.tree[1], 90);
        assert_eq!(t.lazy[1], 20);
        assert_eq!(t.tree[0], 24 + 100);
        // range_sum(0, 2) = 46 + 23 = 69, pushing +20 to [0,1] and [2,3] (46 and 44)
        assert_eq!(t.range_sum(0, 2), 69);
        assert_eq!((t.tree[3], t.tree[4]), (46, 44));
        assert_eq!(t.lazy[1], 0);
        // range_update(1, 2, 1): pushes the pending +20 of [0,1] -> leaves 25, 21, then leaf 1 -> 22
        t.range_update(1, 2, 1);
        let now: Vec<i64> = (0..8).map(|k| t.range_sum(k, k)).collect();
        assert_eq!(now, vec![25, 22, 24, 21, 24, 7, 2, 1]);
    }

    #[test]
    fn persistent_notes_example() {
        let mut p = PersistentSegmentTree::from_slice(&A);
        assert_eq!(p.add(3, 10), 1);
        assert_eq!(p.add(7, 2), 2);
        assert_eq!(p.versions(), 3);
        let at = |i, j| (0..3).map(|t| p.range_sum(i, j, t)).collect::<Vec<_>>();
        assert_eq!(at(0, 7), vec![24, 34, 36]);
        assert_eq!(at(2, 3), vec![4, 14, 14]);
        assert_eq!(at(4, 7), vec![14, 14, 16]);
        assert_eq!(p.nodes.len(), 15 + 2 * 4); // 2n - 1 nodes, then one new node per level per add
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x2026_1A2F);
        for _ in 0..200 {
            let n = 1 + rng.below(30) as usize;
            let a: Vec<i64> = (0..n).map(|_| rng.below(21) as i64 - 10).collect();
            let mut lz = LazySegmentTree::from_slice(&a);
            let mut ps = PersistentSegmentTree::from_slice(&a);
            let mut cur = a.clone();
            let mut versions = vec![a.clone()];
            for _ in 0..60 {
                let i = rng.below(n as u64) as usize;
                let j = i + rng.below((n - i) as u64) as usize;
                let v = rng.below(21) as i64 - 10;
                if rng.below(2) == 0 {
                    lz.range_update(i, j, v);
                    for x in &mut cur[i..=j] {
                        *x += v;
                    }
                } else {
                    assert_eq!(lz.range_sum(i, j), cur[i..=j].iter().sum::<i64>());
                }
                ps.add(i, v);
                let mut b = versions.last().unwrap().clone();
                b[i] += v;
                versions.push(b);
                let t = rng.below(versions.len() as u64) as usize;
                assert_eq!(ps.range_sum(i, j, t), versions[t][i..=j].iter().sum::<i64>());
            }
        }
    }
}
