//! Range update with lazy propagation (2025 notes p.14-15; course page row of 02/11/2026).
//! RangeUpdate(i, j, v): A[k] += v for k in [i, j]; Sum(i) = RangeSum(0, i) (and RangeSum(i, j)).
//! Same three-case visit (2025 p.14):
//! - partial overlap: apply the change to the node in the ST (+v · |overlap|) and recurse;
//! - total overlap: apply the change to the node and REMEMBER the update in the lazy tree (unless it is a leaf),
//!   without going down;
//! - no overlap: do nothing.
//! The pending updates are pushed to the children when a later visit goes below the node.
//! Both operations Θ(log n). Board example 2025: RangeUpdate(0, 4, 20) on [3,1,-2,4,6,13,2,0]: root 27 -> 127.
//! [extra] code mine.

pub struct LazyTree {
    n: usize,
    t: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazyTree {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = LazyTree { n, t: vec![0; 4 * n.max(1)], lazy: vec![0; 4 * n.max(1)] };
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
        let m = (lo + hi) / 2;
        self.build(2 * k, lo, m, a);
        self.build(2 * k + 1, m + 1, hi, a);
        self.t[k] = self.t[2 * k] + self.t[2 * k + 1];
    }
    /// apply "+v to every position" to node k covering [lo, hi]
    fn apply(&mut self, k: usize, lo: usize, hi: usize, v: i64) {
        self.t[k] += v * (hi - lo + 1) as i64;
        if lo < hi {
            self.lazy[k] += v; // remembered for the children
        }
    }
    fn push(&mut self, k: usize, lo: usize, hi: usize) {
        if self.lazy[k] != 0 {
            let m = (lo + hi) / 2;
            let v = self.lazy[k];
            self.apply(2 * k, lo, m, v);
            self.apply(2 * k + 1, m + 1, hi, v);
            self.lazy[k] = 0;
        }
    }
    pub fn range_update(&mut self, l: usize, r: usize, v: i64) {
        self.upd(1, 0, self.n - 1, l, r, v);
    }
    fn upd(&mut self, k: usize, lo: usize, hi: usize, l: usize, r: usize, v: i64) {
        if hi < l || r < lo {
            return; // no overlap
        }
        if l <= lo && hi <= r {
            self.apply(k, lo, hi, v); // total overlap: stop here
            return;
        }
        self.push(k, lo, hi); // partial overlap
        let m = (lo + hi) / 2;
        self.upd(2 * k, lo, m, l, r, v);
        self.upd(2 * k + 1, m + 1, hi, l, r, v);
        self.t[k] = self.t[2 * k] + self.t[2 * k + 1];
    }
    pub fn range_sum(&mut self, l: usize, r: usize) -> i64 {
        self.q(1, 0, self.n - 1, l, r)
    }
    /// Sum(i) = RangeSum(0, i)
    pub fn sum(&mut self, i: usize) -> i64 {
        self.range_sum(0, i)
    }
    fn q(&mut self, k: usize, lo: usize, hi: usize, l: usize, r: usize) -> i64 {
        if hi < l || r < lo {
            return 0;
        }
        if l <= lo && hi <= r {
            return self.t[k];
        }
        self.push(k, lo, hi);
        let m = (lo + hi) / 2;
        self.q(2 * k, lo, m, l, r) + self.q(2 * k + 1, m + 1, hi, l, r)
    }
    pub fn root(&self) -> i64 {
        self.t[1]
    }
    /// pending update stored in node k (for the tests)
    pub fn pending(&self, k: usize) -> i64 {
        self.lazy[k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn example_2025_notes() {
        // p.14: RangeUpdate(0, 4, 20): root 27 + 100; [0,3] is a total overlap: 6 + 80, lazy +20;
        // [4,7]: 21 + 20; [4,5]: 19 + 20; leaf 4: 6 + 20 (leaf: nothing remembered)
        let mut t = LazyTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        t.range_update(0, 4, 20);
        assert_eq!(t.root(), 127);
        assert_eq!(t.pending(2), 20);
        assert_eq!(t.pending(3), 0);
        // p.15: Sum(3) = RangeSum(0, 3) in Θ(log n)
        assert_eq!(t.sum(3), 6 + 80);
        assert_eq!(t.range_sum(1, 1), 21); // pushes the pending +20 down
        assert_eq!(t.range_sum(4, 7), 21 + 20);
    }

    #[test]
    fn notes_2026_example() {
        // same example as l15::prof_blog: range_update(0, 4, 20), then range_sum(0, 2) = 69
        let mut t = LazyTree::new(&[5, 1, 3, 1, 4, 7, 2, 1]);
        t.range_update(0, 4, 20);
        assert_eq!(t.pending(2), 20); // node [0,3] (root = 1 here)
        assert_eq!(t.range_sum(0, 2), 69);
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x1A2_7E5);
        for _ in 0..300 {
            let n = 1 + rng.below(40) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(21) as i64 - 10).collect();
            let mut t = LazyTree::new(&a);
            for _ in 0..80 {
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(21) as i64 - 10;
                    t.range_update(l, r, v);
                    for x in &mut a[l..=r] {
                        *x += v;
                    }
                } else {
                    assert_eq!(t.range_sum(l, r), a[l..=r].iter().sum::<i64>());
                }
            }
        }
    }
}
