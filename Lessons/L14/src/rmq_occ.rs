//! Range Minimum Queries with occurrences (lecture of 02/11, board 60:20-68:00):
//! - Add(i, v)
//! - query(i, j) = the minimum value in A[i..=j] and its number of occurrences in A[i..=j].
//!
//! Every node stores the pair (min, count) of its range. Combining two children (m, c) and (m', c'):
//! if m == m' the result is (m, c + c'), otherwise the pair with the smaller minimum (prof's 2025 notes p.6).
//! Query: same three-case visit, combining the pairs; Add: recompute the pairs up to the root, Θ(log n).
//!
//! [extra] code mine.

pub struct MinCountTree {
    n: usize,
    t: Vec<(i64, u64)>,
}

/// combine the answers of two adjacent ranges
pub fn combine(x: (i64, u64), y: (i64, u64)) -> (i64, u64) {
    if x.0 == y.0 {
        (x.0, x.1 + y.1)
    } else if x.0 < y.0 {
        x
    } else {
        y
    }
}

impl MinCountTree {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = MinCountTree { n, t: vec![(0, 0); 4 * n.max(1)] };
        if n > 0 {
            st.build(1, 0, n - 1, a);
        }
        st
    }

    fn build(&mut self, k: usize, lo: usize, hi: usize, a: &[i64]) {
        if lo == hi {
            self.t[k] = (a[lo], 1);
            return;
        }
        let mid = (lo + hi) / 2;
        self.build(2 * k, lo, mid, a);
        self.build(2 * k + 1, mid + 1, hi, a);
        self.t[k] = combine(self.t[2 * k], self.t[2 * k + 1]);
    }

    /// A[i] += v, Θ(log n)
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.n);
        self.update(1, 0, self.n - 1, i, v);
    }

    fn update(&mut self, k: usize, lo: usize, hi: usize, i: usize, v: i64) {
        if lo == hi {
            self.t[k].0 += v;
            return;
        }
        let mid = (lo + hi) / 2;
        if i <= mid {
            self.update(2 * k, lo, mid, i, v);
        } else {
            self.update(2 * k + 1, mid + 1, hi, i, v);
        }
        self.t[k] = combine(self.t[2 * k], self.t[2 * k + 1]);
    }

    pub fn node(&self, k: usize) -> (i64, u64) {
        self.t[k]
    }

    /// (min A[l..=r], number of occurrences of the min in A[l..=r]), O(log n)
    pub fn query(&self, l: usize, r: usize) -> (i64, u64) {
        assert!(l <= r && r < self.n);
        self.q(1, 0, self.n - 1, l, r).unwrap()
    }

    fn q(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize) -> Option<(i64, u64)> {
        if hi < l || r < lo {
            return None;
        }
        if l <= lo && hi <= r {
            return Some(self.t[k]);
        }
        let mid = (lo + hi) / 2;
        match (self.q(2 * k, lo, mid, l, r), self.q(2 * k + 1, mid + 1, hi, l, r)) {
            (Some(x), Some(y)) => Some(combine(x, y)),
            (x, y) => x.or(y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn example_2025_notes() {
        // Segment_tree_2025.pdf p.6: A = [3, 0, 0, 4, 0, 13, 2, 0]
        let mut st = MinCountTree::new(&[3, 0, 0, 4, 0, 13, 2, 0]);
        assert_eq!(st.node(1), (0, 4));
        assert_eq!((st.node(2), st.node(3)), ((0, 2), (0, 2)));
        assert_eq!(st.node(7), (0, 1)); // [6,7]
        // A[7]: 0 -> -2: [6,7] = (-2,1), [4,7] = (-2,1), root = (-2,1)
        st.add(7, -2);
        assert_eq!((st.node(7), st.node(3), st.node(1)), ((-2, 1), (-2, 1), (-2, 1)));
        assert_eq!(st.query(0, 6), (0, 3));
    }

    #[test]
    fn board_example_2026() {
        // board 62:00-68:00: A = [3, 0, 0, 4, 0, 13, 2, 0], node = (value, #occs)
        let mut st = MinCountTree::new(&[3, 0, 0, 4, 0, 13, 2, 0]);
        assert_eq!((st.node(1), st.node(2), st.node(3)), ((0, 4), (0, 2), (0, 2)));
        assert_eq!(st.query(2, 5), (0, 2)); // [2,3] = (0,1) and [4,5] = (0,1): same min, sum the counters
        // Add(4, 13): 0 -> 13; [4,5] = (13,2), [4,7] = (0,1), root = (0,3)
        st.add(4, 13);
        assert_eq!((st.node(6), st.node(3), st.node(1)), ((13, 2), (0, 1), (0, 3)));
        assert_eq!(st.query(2, 5), (0, 1)); // (0,1) and (13,2): take the smaller min, "0 1, not 0 2"
    }

    #[test]
    fn notes_2026_example() {
        // prof's 2026 notes: A = [5, 1, 3, 1, 4, 7, 2, 1], root (1, 3); after add(1, -1): (0, 1)
        let mut st = MinCountTree::new(&[5, 1, 3, 1, 4, 7, 2, 1]);
        assert_eq!(st.node(1), (1, 3));
        st.add(1, -1);
        assert_eq!(st.node(1), (0, 1));
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0xE703_7ED1_A0B4_28DB);
        for _ in 0..300 {
            let n = 1 + rng.below(40) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(5) as i64).collect();
            let mut st = MinCountTree::new(&a);
            for _ in 0..100 {
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(5) as i64 - 2;
                    st.add(l, v);
                    a[l] += v;
                } else {
                    let m = *a[l..=r].iter().min().unwrap();
                    let c = a[l..=r].iter().filter(|&&x| x == m).count() as u64;
                    assert_eq!(st.query(l, r), (m, c));
                }
            }
        }
    }
}
