//! Persistent segment tree (2025 notes p.16-19). Operations that change the DS move the timestamp by 1;
//! queries may specify any valid timestamp: Add(i, v) and RangeSum(i, j, t) = sum of A[i..=j] at time t.
//! - Attempt #1: a copy of the ST after each update: RangeSum Θ(log n), but space Θ(n·t) and Add Θ(n).
//! - Attempt #2: only Θ(log n) nodes change per update: every node keeps a list of (value, timestamp);
//!   Add Θ(log n), space Θ(n + t log n), RangeSum(i, j, t) = predecessor of t in the list of every
//!   total-overlap node: Θ(log n · log t).
//! - Attempt #3 (implemented here): the Θ(log n) changed nodes are on a root-to-leaf path: create a new copy
//!   of that path only, sharing every other node with the previous version; an array of roots indexed by
//!   timestamp. RangeSum Θ(log n), Add Θ(log n), space Θ(n + t log n) after t updates.
//! [extra] code mine; version 0 = the initial array; nodes in a Vec arena (indices instead of pointers).

#[derive(Clone, Copy)]
struct Node {
    sum: i64,
    left: usize,
    right: usize,
}

pub struct PersistentTree {
    n: usize,
    nodes: Vec<Node>,
    roots: Vec<usize>,
}

impl PersistentTree {
    pub fn new(a: &[i64]) -> Self {
        assert!(!a.is_empty());
        let mut st = PersistentTree { n: a.len(), nodes: Vec::new(), roots: Vec::new() };
        let r = st.build(0, a.len() - 1, a);
        st.roots.push(r);
        st
    }
    fn build(&mut self, lo: usize, hi: usize, a: &[i64]) -> usize {
        if lo == hi {
            self.nodes.push(Node { sum: a[lo], left: 0, right: 0 });
            return self.nodes.len() - 1;
        }
        let m = (lo + hi) / 2;
        let (l, r) = (self.build(lo, m, a), self.build(m + 1, hi, a));
        self.nodes.push(Node { sum: self.nodes[l].sum + self.nodes[r].sum, left: l, right: r });
        self.nodes.len() - 1
    }
    /// current timestamp (number of updates so far)
    pub fn time(&self) -> usize {
        self.roots.len() - 1
    }
    pub fn nodes(&self) -> usize {
        self.nodes.len()
    }
    /// A[i] += v in a new version; returns its timestamp. Θ(log n) new nodes.
    pub fn add(&mut self, i: usize, v: i64) -> usize {
        let r = self.copy_path(self.roots[self.time()], 0, self.n - 1, i, v);
        self.roots.push(r);
        self.time()
    }
    fn copy_path(&mut self, k: usize, lo: usize, hi: usize, i: usize, v: i64) -> usize {
        let mut node = self.nodes[k]; // copy of the old node
        node.sum += v;
        if lo < hi {
            let m = (lo + hi) / 2;
            if i <= m {
                node.left = self.copy_path(node.left, lo, m, i, v);
            } else {
                node.right = self.copy_path(node.right, m + 1, hi, i, v);
            }
        }
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    /// sum of A[l..=r] at time t, Θ(log n)
    pub fn range_sum(&self, l: usize, r: usize, t: usize) -> i64 {
        self.q(self.roots[t], 0, self.n - 1, l, r)
    }
    fn q(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize) -> i64 {
        if hi < l || r < lo {
            return 0;
        }
        if l <= lo && hi <= r {
            return self.nodes[k].sum;
        }
        let m = (lo + hi) / 2;
        self.q(self.nodes[k].left, lo, m, l, r) + self.q(self.nodes[k].right, m + 1, hi, l, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn example_2025_notes() {
        // p.18-19: A = [3, 1, -2, 4, 6, 13, 2, 0], Add(3, 10) at time 1, then Add(7, 2) (p.19) at time 2
        let mut t = PersistentTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        let before = t.nodes();
        assert_eq!(t.add(3, 10), 1);
        assert_eq!(t.nodes() - before, 4); // one new node per level
        assert_eq!(t.add(7, 2), 2);
        assert_eq!(t.range_sum(0, 7, 0), 27);
        assert_eq!(t.range_sum(0, 7, 1), 37);
        assert_eq!(t.range_sum(0, 7, 2), 39);
        assert_eq!(t.range_sum(0, 3, 1), 16); // [0,3]: 6 -> 16 at time 1
        assert_eq!(t.range_sum(4, 7, 1), 21); // unchanged until time 2
        assert_eq!(t.range_sum(4, 7, 2), 23);
    }

    #[test]
    fn random_vs_copies() {
        let mut rng = Rng(0x9E25_1575);
        for _ in 0..100 {
            let n = 1 + rng.below(30) as usize;
            let a: Vec<i64> = (0..n).map(|_| rng.below(21) as i64 - 10).collect();
            let mut t = PersistentTree::new(&a);
            let mut versions = vec![a]; // attempt #1: a full copy per version
            for _ in 0..60 {
                let i = rng.below(n as u64) as usize;
                let v = rng.below(21) as i64 - 10;
                t.add(i, v);
                let mut b = versions.last().unwrap().clone();
                b[i] += v;
                versions.push(b);
                let tt = rng.below(versions.len() as u64) as usize;
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                assert_eq!(t.range_sum(l, r, tt), versions[tt][l..=r].iter().sum::<i64>());
            }
        }
    }
}
