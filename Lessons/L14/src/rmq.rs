//! Dynamic Range Minimum Queries (lecture of 02/11, board 50:00-60:20, "easy"):
//! - Add(i, v): A[i] += v
//! - RMQ(i, j) = argmin A[i..=j], the POSITION of the minimum.
//!
//! Every node stores the answer for its range: the pair (pos, value) of its minimum — or only pos, and
//! the value is read in A (Octech's notes). RMQ: the same three-case visit as RangeSum, combining the
//! candidates with min. Add: change the leaf and recompute the minima on the path up to the root,
//! Θ(log n) (Octech's notes: early stopping is possible when the node does not change).
//!
//! [extra] Ties: the leftmost position (not specified in the lecture).

pub struct RmqTree {
    n: usize,
    a: Vec<i64>,
    t: Vec<usize>, // position of the minimum of the node's range
}

impl RmqTree {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = RmqTree { n, a: a.to_vec(), t: vec![0; 4 * n.max(1)] };
        if n > 0 {
            st.build(1, 0, n - 1);
        }
        st
    }

    /// leftmost position of the minimum between positions p and q
    fn better(&self, p: usize, q: usize) -> usize {
        if self.a[q] < self.a[p] || (self.a[q] == self.a[p] && q < p) { q } else { p }
    }

    fn build(&mut self, k: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.t[k] = lo;
            return;
        }
        let mid = (lo + hi) / 2;
        self.build(2 * k, lo, mid);
        self.build(2 * k + 1, mid + 1, hi);
        self.t[k] = self.better(self.t[2 * k], self.t[2 * k + 1]);
    }

    /// A[i] += v, Θ(log n): recursion down to the leaf, recomputation on the way back up.
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.n);
        self.a[i] += v;
        self.update(1, 0, self.n - 1, i);
    }

    fn update(&mut self, k: usize, lo: usize, hi: usize, i: usize) {
        if lo == hi {
            return; // the leaf stores i itself, the value is in a[i]
        }
        let mid = (lo + hi) / 2;
        if i <= mid {
            self.update(2 * k, lo, mid, i);
        } else {
            self.update(2 * k + 1, mid + 1, hi, i);
        }
        self.t[k] = self.better(self.t[2 * k], self.t[2 * k + 1]);
    }

    /// (pos, value) of the minimum stored in node k (1 = root), for the tests
    pub fn node(&self, k: usize) -> (usize, i64) {
        (self.t[k], self.a[self.t[k]])
    }

    /// argmin A[l..=r] (leftmost), O(log n)
    pub fn rmq(&self, l: usize, r: usize) -> usize {
        assert!(l <= r && r < self.n);
        self.query(1, 0, self.n - 1, l, r).unwrap()
    }

    fn query(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize) -> Option<usize> {
        if hi < l || r < lo {
            return None;
        }
        if l <= lo && hi <= r {
            return Some(self.t[k]);
        }
        let mid = (lo + hi) / 2;
        match (self.query(2 * k, lo, mid, l, r), self.query(2 * k + 1, mid + 1, hi, l, r)) {
            (Some(p), Some(q)) => Some(self.better(p, q)),
            (p, q) => p.or(q),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    fn naive(a: &[i64], l: usize, r: usize) -> usize {
        (l..=r).fold(l, |b, i| if a[i] < a[b] { i } else { b })
    }

    #[test]
    fn example_2025_notes() {
        // Segment_tree_2025.pdf p.5: A = [3, 1, -2, 4, 6, 13, 2, 0], nodes store (pos, value)
        let mut st = RmqTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        assert_eq!(st.node(1), (2, -2));
        assert_eq!((st.node(2), st.node(3)), ((2, -2), (7, 0)));
        assert_eq!((st.node(4), st.node(5), st.node(6), st.node(7)), ((1, 1), (2, -2), (4, 6), (7, 0)));
        // Add(2, 4): -2 -> 2, then [2,3] = (2,2), [0,3] = (1,1), root = (7,0)
        st.add(2, 4);
        assert_eq!(st.node(5), (2, 2));
        assert_eq!(st.node(2), (1, 1));
        assert_eq!(st.node(1), (7, 0));
        assert_eq!(st.rmq(2, 6), 2); // A[2..=6] = [2, 4, 6, 13, 2]: leftmost 2
    }

    #[test]
    fn board_example_2026() {
        // board 52:00-60:00: A = [3, 1, -2, 4, 6, 13, 2, 0]; RMQ(2, 5) = min of the total-overlap nodes
        // [2,3] (-2) and [4,5] (6) -> position 2
        let mut st = RmqTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        assert_eq!(st.rmq(2, 5), 2);
        // Add(2, +7): -2 -> 5; [2,3] = min(5, 4) = 4; [0,3] = min(1, 4) = 1; root = min(1, 0) = 0
        st.add(2, 7);
        assert_eq!(st.node(5), (3, 4));
        assert_eq!(st.node(2), (1, 1));
        assert_eq!(st.node(1), (7, 0));
        assert_eq!(st.rmq(2, 5), 3);
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0xA076_1D64_78BD_642F);
        for _ in 0..300 {
            let n = 1 + rng.below(40) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(11) as i64 - 5).collect();
            let mut st = RmqTree::new(&a);
            for _ in 0..100 {
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(11) as i64 - 5;
                    st.add(l, v);
                    a[l] += v;
                } else {
                    assert_eq!(st.rmq(l, r), naive(&a, l, r));
                }
            }
        }
    }
}
