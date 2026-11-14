//! #occs(i, j, x): number of occurrences of x in A[i..=j], with Add(i, v) (2025 notes p.9).
//! Every node stores a hash map value -> number of occurrences in its range. Query: the usual visit,
//! summing the counter of x in the O(log n) total-overlap nodes: Θ(log n) w.h.p. (hashing).
//! Space: every position is in the map of its O(log n) ancestors -> O(n log n)
//! (2025: "more space-efficient solution: wavelet trees").
//! [extra] Add: on the leaf-root path, -1 for the old value and +1 for the new one.

use std::collections::HashMap;

pub struct OccsTree {
    n: usize,
    a: Vec<i64>,
    t: Vec<HashMap<i64, usize>>,
}

impl OccsTree {
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
            let m = (lo + hi) / 2;
            self.build(2 * k, lo, m);
            self.build(2 * k + 1, m + 1, hi);
        }
    }
    /// A[i] += v, Θ(log n) expected
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
    pub fn occs(&self, l: usize, r: usize, x: i64) -> usize {
        self.q(1, 0, self.n - 1, l, r, x)
    }
    fn q(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize, x: i64) -> usize {
        if hi < l || r < lo {
            return 0;
        }
        if l <= lo && hi <= r {
            return *self.t[k].get(&x).unwrap_or(&0);
        }
        let m = (lo + hi) / 2;
        self.q(2 * k, lo, m, l, r, x) + self.q(2 * k + 1, m + 1, hi, l, r, x)
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
        // p.9: A = [3, 1, 0, 4, 6, 13, 2, 0], #occs(1, 7, 0) = 2; node [4,7] = {6:1, 13:1, 2:1, 0:1}
        let mut t = OccsTree::new(&[3, 1, 0, 4, 6, 13, 2, 0]);
        assert_eq!(t.occs(1, 7, 0), 2);
        assert_eq!(t.t[3].len(), 4);
        t.add(4, -6); // 6 -> 0
        assert_eq!(t.occs(1, 7, 0), 3);
        assert_eq!(t.occs(0, 7, 6), 0);
        // distinct values: n entries per level -> n * (log n + 1)
        assert_eq!(OccsTree::new(&(0..64).collect::<Vec<_>>()).entries(), 64 * 7);
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x15_0CC5);
        for _ in 0..200 {
            let n = 1 + rng.below(30) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(4) as i64).collect();
            let mut t = OccsTree::new(&a);
            for _ in 0..80 {
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(5) as i64 - 2;
                    t.add(l, v);
                    a[l] += v;
                } else {
                    let x = rng.below(5) as i64 - 1;
                    assert_eq!(t.occs(l, r, x), a[l..=r].iter().filter(|&&y| y == x).count());
                }
            }
        }
    }
}
