//! Problem (lecture of 02/11, Octech's notes stop at the statement): A[0, n-1] of integers,
//! - Add(i, v)
//! - #occs(i, j, x): number of occurrences of x in A[i..=j].
//!
//! [not in Octech's notes] Solution from the prof's 2025 notes (p.9): every node stores a HASH MAP
//! value -> number of occurrences in the node's range. Query: the usual visit, summing the counters of x
//! in the O(log n) total-overlap nodes: Θ(log n) w.h.p. (expected, hashing). Space O(n log n): every
//! position appears in the map of each of its log n ancestors ("more space efficient solution: wavelet trees").
//!
//! [extra] Add(i, v) (not detailed in the notes): on each of the log n nodes of the path, decrement the
//! counter of the old value (removing it at 0) and increment the counter of the new one. Code mine.

use std::collections::HashMap;

pub struct OccsTree {
    n: usize,
    a: Vec<i64>,
    t: Vec<HashMap<i64, u32>>,
}

impl OccsTree {
    /// O(n log n) expected: each level of the tree holds n (value, counter) entries in total
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = OccsTree { n, a: a.to_vec(), t: vec![HashMap::new(); 4 * n.max(1)] };
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
            let mid = (lo + hi) / 2;
            self.build(2 * k, lo, mid);
            self.build(2 * k + 1, mid + 1, hi);
        }
    }

    /// A[i] += v, Θ(log n) expected
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.n);
        if v == 0 {
            return;
        }
        let (old, new) = (self.a[i], self.a[i] + v);
        self.a[i] = new;
        let (mut k, mut lo, mut hi) = (1, 0, self.n - 1);
        loop {
            let m = &mut self.t[k];
            let c = m.get_mut(&old).unwrap();
            *c -= 1;
            if *c == 0 {
                m.remove(&old);
            }
            *m.entry(new).or_insert(0) += 1;
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

    /// number of occurrences of x in A[l..=r], Θ(log n) expected
    pub fn occs(&self, l: usize, r: usize, x: i64) -> u32 {
        assert!(l <= r && r < self.n);
        self.q(1, 0, self.n - 1, l, r, x)
    }

    fn q(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize, x: i64) -> u32 {
        if hi < l || r < lo {
            return 0;
        }
        if l <= lo && hi <= r {
            return self.t[k].get(&x).copied().unwrap_or(0);
        }
        let mid = (lo + hi) / 2;
        self.q(2 * k, lo, mid, l, r, x) + self.q(2 * k + 1, mid + 1, hi, l, r, x)
    }

    /// total number of (value, counter) entries in all the maps (space)
    pub fn entries(&self) -> usize {
        self.t.iter().map(|m| m.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn example_2025_notes() {
        // 2025 notes p.9: A = [3, 1, 0, 4, 6, 13, 2, 0], #occs(1, 7, 0) = 2; node [4,7] = {6:1, 13:1, 2:1, 0:1}
        let mut st = OccsTree::new(&[3, 1, 0, 4, 6, 13, 2, 0]);
        assert_eq!(st.occs(1, 7, 0), 2);
        assert_eq!(st.t[3], HashMap::from([(6, 1), (13, 1), (2, 1), (0, 1)]));
        st.add(5, -13); // 13 -> 0
        assert_eq!(st.occs(1, 7, 0), 3);
        assert_eq!(st.occs(0, 7, 13), 0);
    }

    #[test]
    fn space_n_log_n() {
        // distinct values: every position is in the map of each ancestor -> n * levels entries
        let n = 1024;
        let st = OccsTree::new(&(0..n as i64).collect::<Vec<_>>());
        assert_eq!(st.entries(), n * 11);
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x4F1B_BCDC_BFA5_3E0B);
        for _ in 0..300 {
            let n = 1 + rng.below(40) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(6) as i64).collect();
            let mut st = OccsTree::new(&a);
            for _ in 0..100 {
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(5) as i64 - 2;
                    st.add(l, v);
                    a[l] += v;
                } else {
                    let x = rng.below(8) as i64 - 1;
                    let c = a[l..=r].iter().filter(|&&y| y == x).count() as u32;
                    assert_eq!(st.occs(l, r, x), c);
                }
            }
        }
    }
}
