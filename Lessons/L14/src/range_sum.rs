//! Answer to the question left at the end of L13: "is RangeSum on the segment tree linear, or better?"
//! It visits O(log n) nodes (lecture of 02/11, board 20:00-49:00; Octech's notes; prof's 2025 notes p.3-4):
//! - total overlap / no overlap: the node is visited and the recursion stops there;
//! - partial overlap with the query on one side of the node's middle: one of the two calls stops at once
//!   (easy case);
//! - partial overlap across the middle (the only general case, it happens at most once on the way down):
//!   the two children get ranges that share an extreme with the query, so from then on at every level
//!   one child is a total overlap (or no overlap) and only one keeps going.
//!
//! So at most 2 partial-overlap nodes per level and at most 4 visited nodes per level:
//! `visited <= 4 * levels`. Octech's notes: "at most Θ(4 log n), Θ(2 log n) checking before the calls".
//!
//! [extra] Same tree as `l13::segment_tree` (copied), plus `range_sum_counting` and a test of the bound.

pub struct SumTree {
    n: usize,
    t: Vec<i64>,
}

impl SumTree {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = SumTree { n, t: vec![0; 4 * n.max(1)] };
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
            self.t[k] += v;
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

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.range_sum_counting(l, r).0
    }

    /// (sum, number of visited nodes)
    pub fn range_sum_counting(&self, l: usize, r: usize) -> (i64, usize) {
        assert!(l <= r && r < self.n);
        let mut visited = 0;
        let s = self.query(1, 0, self.n - 1, l, r, &mut visited);
        (s, visited)
    }

    fn query(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize, visited: &mut usize) -> i64 {
        *visited += 1;
        if hi < l || r < lo {
            return 0; // no overlap
        }
        if l <= lo && hi <= r {
            return self.t[k]; // total overlap
        }
        let mid = (lo + hi) / 2; // partial overlap
        self.query(2 * k, lo, mid, l, r, visited) + self.query(2 * k + 1, mid + 1, hi, l, r, visited)
    }
}

/// number of levels of the tree built on n leaves (splitting in the middle): 1 + ceil(log2 n)
pub fn levels(n: usize) -> usize {
    let mut h = 1;
    while (1usize << (h - 1)) < n {
        h += 1;
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn example_2025_notes() {
        // Segment_tree_2025.pdf p.2: RangeSum(2, 6) on A = [3, 1, -2, 4, 6, 13, 2, 0]
        let st = SumTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        assert_eq!(st.range_sum(2, 6), 23);
        assert_eq!(st.range_sum_counting(2, 5), (21, 7)); // L13 board
    }

    #[test]
    fn visited_nodes_at_most_4_per_level() {
        // all ranges, n up to 130: the visit is O(log n), never linear
        for n in 1..=130 {
            let a: Vec<i64> = (0..n as i64).collect();
            let st = SumTree::new(&a);
            let bound = 4 * levels(n);
            for l in 0..n {
                for r in l..n {
                    let (s, v) = st.range_sum_counting(l, r);
                    assert_eq!(s, a[l..=r].iter().sum::<i64>());
                    assert!(v <= bound, "n={n} [{l},{r}] visited {v} > {bound}");
                }
            }
        }
    }

    #[test]
    fn board_examples_2026() {
        // board 3:40-13:40: RS(2, 5) = 2 + 19 = 21 on A = [3, 1, -2, 4, 6, 13, 2, 0]; in this small
        // example the visit touches all 7 internal nodes (the prof: "is the example too small?")
        let mut st = SumTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        assert_eq!(st.range_sum_counting(2, 5), (21, 7));
        st.add(4, 3); // Add(4, +3): 6 -> 9, Θ(log n)
        assert_eq!(st.range_sum(2, 5), 24);
        // prof's worst case (46:00): the shortest range with more than one element, split at the root:
        // [n/2 - 1, n/2] -> two root-to-leaf paths, each with an "easy" sibling at every level
        let n = 1 << 10;
        let st = SumTree::new(&vec![1; n]);
        let (s, v) = st.range_sum_counting(n / 2 - 1, n / 2);
        assert_eq!(s, 2);
        // root + its two children + 4 nodes (2 on each path, each with its easy sibling) on every lower level
        assert_eq!(v, 1 + 2 + 4 * (levels(n) - 2)); // = 39 for n = 1024: about 4 log n
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
        for _ in 0..200 {
            let n = 1 + rng.below(40) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(21) as i64 - 10).collect();
            let mut st = SumTree::new(&a);
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
