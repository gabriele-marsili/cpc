//! [extra] Segment tree for dynamic prefix sums (lecture of 28/10, last 30 min). The prof gave no code:
//! this is my implementation of what was on the board.
//!
//! - binary tree whose leaves are the elements; every node stores THE ANSWER FOR ITS RANGE (here: the sum);
//! - build BOTTOM-UP: n + n/2 + n/4 + ... <= 2n nodes, Θ(n) (top-down rescanning the array would be Θ(n log n));
//! - Add(i, v): the path leaf -> root, Θ(log n); done top-down so no parent pointers are needed;
//! - RangeSum(l, r) top-down from the root, three cases per node: node range disjoint from [l, r]
//!   -> stop, contributes 0; node range fully inside [l, r] -> take the node's answer, stop;
//!   otherwise (partial overlap) -> go to BOTH children.
//!
//! Layout: node 1 is the root, children of k are 2k and 2k+1, array of size 4n (enough for any n).
//! Positions are 0-based, as on the board.

pub struct SegmentTree {
    n: usize,
    t: Vec<i64>,
}

impl SegmentTree {
    /// Θ(n): each node is computed once from its two children (post-order = bottom-up).
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = SegmentTree { n, t: vec![0; 4 * n.max(1)] };
        if n > 0 {
            st.build(1, 0, n - 1, a);
        }
        st
    }

    fn build(&mut self, k: usize, lo: usize, hi: usize, a: &[i64]) {
        if lo == hi {
            self.t[k] = a[lo];
            return;
        }
        let mid = (lo + hi) / 2;
        self.build(2 * k, lo, mid, a);
        self.build(2 * k + 1, mid + 1, hi, a);
        self.t[k] = self.t[2 * k] + self.t[2 * k + 1];
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// A[i] += v, Θ(log n)
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.n);
        let (mut k, mut lo, mut hi) = (1, 0, self.n - 1);
        loop {
            self.t[k] += v; // every node on the root-leaf path contains position i
            if lo == hi {
                break;
            }
            let mid = (lo + hi) / 2;
            if i <= mid {
                k *= 2;
                hi = mid;
            } else {
                k = 2 * k + 1;
                lo = mid + 1;
            }
        }
    }

    /// A[l] + ... + A[r] (0-based, l <= r < n)
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.range_sum_counting(l, r).0
    }

    /// Same as `range_sum`, also returns the number of nodes visited (for the weekend question:
    /// "is this algorithm linear, or better?" — experiment with it before reading any answer).
    pub fn range_sum_counting(&self, l: usize, r: usize) -> (i64, usize) {
        assert!(l <= r && r < self.n);
        let mut visited = 0;
        let s = self.query(1, 0, self.n - 1, l, r, &mut visited);
        (s, visited)
    }

    fn query(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize, visited: &mut usize) -> i64 {
        *visited += 1;
        if hi < l || r < lo {
            return 0; // disjoint: "unrelated answer, not useful"
        }
        if l <= lo && hi <= r {
            return self.t[k]; // node range inside the query range: take the answer and stop
        }
        let mid = (lo + hi) / 2; // partial overlap: both directions
        self.query(2 * k, lo, mid, l, r, visited) + self.query(2 * k + 1, mid + 1, hi, l, r, visited)
    }

    /// value stored in node k (1 = root), for the tests
    pub fn node(&self, k: usize) -> i64 {
        self.t[k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn board_example() {
        // board 73:00-96:00: A = [3, 1, -2, 4, 6, 13, 2, 0], positions 0..7
        let a = [3, 1, -2, 4, 6, 13, 2, 0];
        let mut st = SegmentTree::new(&a);
        assert_eq!(st.node(1), 27); // [0,7]
        assert_eq!((st.node(2), st.node(3)), (6, 21)); // [0,3], [4,7]
        assert_eq!((st.node(4), st.node(5), st.node(6), st.node(7)), (4, 2, 19, 2)); // pairs
        // RS(2, 5) = 21, touching 7 nodes as counted in class
        assert_eq!(st.range_sum_counting(2, 5), (21, 7));
        // Add(4, +3): +3 on the leaf and on [4,5], [4,7], [0,7]
        st.add(4, 3);
        assert_eq!((st.node(1), st.node(3), st.node(6)), (30, 24, 22));
        assert_eq!(st.range_sum(2, 5), 24);
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0xD1B5_4A32_D192_ED03);
        for _ in 0..200 {
            let n = 1 + rng.below(40) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(21) as i64 - 10).collect();
            let mut st = SegmentTree::new(&a);
            for _ in 0..100 {
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(21) as i64 - 10;
                    st.add(l, v);
                    a[l] += v;
                } else {
                    assert_eq!(st.range_sum(l, r), a[l..=r].iter().sum::<i64>());
                }
            }
        }
    }
}
