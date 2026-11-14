//! Successor(i, j, x): the successor of x in A[i..=j], with Add(i, v) (2025 notes p.10-11).
//! Every node stores its range's values in a BST (successor in Θ(log n)). Query: compute the successor
//! in every total-overlap node and combine by taking the min: O(log n) nodes × Θ(log n) = Θ(log² n);
//! space Θ(n log n). 2025: "query time can be improved to Θ(log n) with fractional cascading" (not
//! implemented here).
//! [extra] successor = smallest value >= x (2025 p.10, Successor(1, 5, 5) on [3,1,0,4,6,13,2,0] is circled
//! at 6, consistent with both >= and >); BTreeMap value -> count as a multiset; `None` if there is none.

use std::collections::BTreeMap;

pub struct SuccTree {
    n: usize,
    a: Vec<i64>,
    t: Vec<BTreeMap<i64, usize>>,
}

impl SuccTree {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = SuccTree { n, a: a.to_vec(), t: vec![BTreeMap::new(); 4 * n.max(1)] };
        if n > 0 {
            st.build(1, 0, n - 1);
        }
        st
    }
    fn build(&mut self, k: usize, lo: usize, hi: usize) {
        for i in lo..=hi {
            *self.t[k].entry(self.a[i]).or_insert(0) += 1;
        }
        if lo < hi {
            let m = (lo + hi) / 2;
            self.build(2 * k, lo, m);
            self.build(2 * k + 1, m + 1, hi);
        }
    }
    /// A[i] += v: on every node of the leaf-root path remove the old value, insert the new one, Θ(log² n)
    pub fn add(&mut self, i: usize, v: i64) {
        let (old, new) = (self.a[i], self.a[i] + v);
        self.a[i] = new;
        let (mut k, mut lo, mut hi) = (1, 0, self.n - 1);
        loop {
            let c = self.t[k].get_mut(&old).unwrap();
            *c -= 1;
            if *c == 0 {
                self.t[k].remove(&old);
            }
            *self.t[k].entry(new).or_insert(0) += 1;
            if lo == hi {
                return;
            }
            let m = (lo + hi) / 2;
            if i <= m { (k, hi) = (2 * k, m) } else { (k, lo) = (2 * k + 1, m + 1) }
        }
    }
    /// smallest A[p] >= x with l <= p <= r, Θ(log² n)
    pub fn successor(&self, l: usize, r: usize, x: i64) -> Option<i64> {
        self.q(1, 0, self.n - 1, l, r, x)
    }
    fn q(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize, x: i64) -> Option<i64> {
        if hi < l || r < lo {
            return None;
        }
        if l <= lo && hi <= r {
            return self.t[k].range(x..).next().map(|(&v, _)| v); // successor in this node's BST
        }
        let m = (lo + hi) / 2;
        match (self.q(2 * k, lo, m, l, r, x), self.q(2 * k + 1, m + 1, hi, l, r, x)) {
            (Some(p), Some(q)) => Some(p.min(q)), // combine by taking the min
            (p, q) => p.or(q),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn example_2025_notes() {
        let mut t = SuccTree::new(&[3, 1, 0, 4, 6, 13, 2, 0]);
        assert_eq!(t.successor(1, 5, 5), Some(6));
        assert_eq!(t.successor(1, 5, 7), Some(13));
        assert_eq!(t.successor(1, 5, 14), None);
        t.add(5, -8); // 13 -> 5
        assert_eq!(t.successor(1, 5, 5), Some(5));
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x5ACC_E550);
        for _ in 0..200 {
            let n = 1 + rng.below(30) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(20) as i64).collect();
            let mut t = SuccTree::new(&a);
            for _ in 0..80 {
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(9) as i64 - 4;
                    t.add(l, v);
                    a[l] += v;
                } else {
                    let x = rng.below(26) as i64 - 3;
                    assert_eq!(t.successor(l, r, x), a[l..=r].iter().copied().filter(|&y| y >= x).min());
                }
            }
        }
    }
}
