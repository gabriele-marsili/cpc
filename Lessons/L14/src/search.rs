//! Problem (lecture of 02/11, board 70:00-87:40): A[1, n] of NON-NEGATIVE integers,
//! - Add(i, v), RangeSum(i, j)
//! - Search(s): the position i of the shortest prefix of A with A[1] + ... + A[i] >= s.
//!
//! Specialized solution, Θ(log n): from the root, go left whenever the left child's sum is >= s;
//! otherwise subtract the left child's sum from s and go right ("subtract only when we go right").
//! At the leaf, its value is >= what is still missing.
//! Naive solution: binary search for the answer on i with RangeSum(0, i) >= s: Θ(log n) steps, each
//! Θ(log n), so Θ(log² n) (board/transcript 84:00-86:00; Octech's notes; 2025 notes p.8). The prof: it
//! is "particularly inefficient" because every step restarts from the root and recomputes the same nodes.
//! Non-negative values make the prefix sums non-decreasing: that is what both solutions need.
//!
//! [extra] code mine; positions 0-based (the board example uses positions 0..7); `None` if the whole
//! array sums to less than s.

use crate::range_sum::SumTree;

pub struct SearchTree {
    n: usize,
    t: Vec<u64>,
}

impl SearchTree {
    pub fn new(a: &[u64]) -> Self {
        let n = a.len();
        let mut st = SearchTree { n, t: vec![0; 4 * n.max(1)] };
        if n > 0 {
            st.build(1, 0, n - 1, a);
        }
        st
    }

    fn build(&mut self, k: usize, lo: usize, hi: usize, a: &[u64]) {
        if lo == hi {
            self.t[k] = a[lo];
            return;
        }
        let mid = (lo + hi) / 2;
        self.build(2 * k, lo, mid, a);
        self.build(2 * k + 1, mid + 1, hi, a);
        self.t[k] = self.t[2 * k] + self.t[2 * k + 1];
    }

    /// A[i] += v (v may be negative, but A[i] must stay >= 0), Θ(log n)
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.n);
        let (mut k, mut lo, mut hi) = (1, 0, self.n - 1);
        loop {
            self.t[k] = self.t[k].checked_add_signed(v).expect("A must stay non-negative");
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

    /// smallest i with A[0] + ... + A[i] >= s, Θ(log n)
    pub fn search(&self, mut s: u64) -> Option<usize> {
        if self.n == 0 || self.t[1] < s {
            return None;
        }
        let (mut k, mut lo, mut hi) = (1, 0, self.n - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.t[2 * k] >= s {
                k *= 2; // the answer is in the left half
                hi = mid;
            } else {
                s -= self.t[2 * k]; // skip the whole left half
                k = 2 * k + 1;
                lo = mid + 1;
            }
        }
        Some(lo)
    }
}

/// Naive solution: binary search for the answer with RangeSum, Θ(log² n).
pub fn search_binary(st: &SumTree, s: i64) -> Option<usize> {
    let n = st.len();
    if n == 0 || st.range_sum(0, n - 1) < s {
        return None;
    }
    let (mut lo, mut hi) = (0, n - 1); // the answer is in [lo, hi]
    while lo < hi {
        let mid = (lo + hi) / 2;
        if st.range_sum(0, mid) >= s { hi = mid } else { lo = mid + 1 }
    }
    Some(lo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    fn naive(a: &[u64], s: u64) -> Option<usize> {
        let mut p = 0;
        for (i, &x) in a.iter().enumerate() {
            p += x;
            if p >= s {
                return Some(i);
            }
        }
        None
    }

    #[test]
    fn examples() {
        // board 74:40 (and 2025 notes p.7): A = [3, 1, 0, 4, 6, 13, 2, 0], Search(22) = 5 (prefix sums 3 4 4 8 14 27)
        // board 86:00: root 29, 22 vs 8 -> right with 22-8 = 14; 14 vs 19 -> left; leaf 6 < 14 -> right with 8; leaf 13 >= 8
        let a = [3, 1, 0, 4, 6, 13, 2, 0];
        let st = SearchTree::new(&a);
        assert_eq!(st.search(22), Some(5));
        assert_eq!(st.search(23), Some(5)); // 2025 notes p.8
        // Octech's notes have 5 instead of 6 at position 4: same answer (prefix sums 3 4 4 8 13 26)
        assert_eq!(SearchTree::new(&[3, 1, 0, 4, 5, 13, 2, 0]).search(22), Some(5));
        assert_eq!(st.search(29), Some(6)); // total = 29
        assert_eq!(st.search(30), None);
        assert_eq!(st.search(0), Some(0));
        // prof's 2026 notes: search(20) = 5 on A = [5, 1, 3, 1, 4, 7, 2, 1]
        assert_eq!(SearchTree::new(&[5, 1, 3, 1, 4, 7, 2, 1]).search(20), Some(5));
        let sum = SumTree::new(&a.map(|x| x as i64));
        assert_eq!(search_binary(&sum, 22), Some(5));
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x8EBC_6AF0_9C88_C6E3);
        for _ in 0..300 {
            let n = 1 + rng.below(40) as usize;
            let mut a: Vec<u64> = (0..n).map(|_| rng.below(6)).collect();
            let mut st = SearchTree::new(&a);
            let mut sum = SumTree::new(&a.iter().map(|&x| x as i64).collect::<Vec<_>>());
            for _ in 0..100 {
                if rng.below(2) == 0 {
                    let i = rng.below(n as u64) as usize;
                    let v = rng.below(5) as i64 - (a[i].min(2) as i64);
                    st.add(i, v);
                    sum.add(i, v);
                    a[i] = (a[i] as i64 + v) as u64;
                } else {
                    let s = rng.below(6 * n as u64 + 2);
                    assert_eq!(st.search(s), naive(&a, s));
                    assert_eq!(search_binary(&sum, s as i64), naive(&a, s));
                }
            }
        }
    }
}
