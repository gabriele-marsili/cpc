//! Prof's code, copied VERBATIM from his 2026 notes "Segment Tree"
//! (https://pages.di.unipi.it/rossano/blog/2026/segmenttree/, 14/11/2026): `SegmentTree`
//! (sections "Segment Tree in Rust" and "Searching on a Segment Tree"). Only this header and the
//! tests are mine [extra]. Layout: root = node 0, children 2u+1 and 2u+2, vector of 4n nodes
//! (my `range_sum`/`search` modules use root 1 and children 2k, 2k+1: same tree).
//! Running example of the notes: A = [5, 1, 3, 1, 4, 7, 2, 1].

#[derive(Debug)]
pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    pub fn from_slice(a: &[i64]) -> Self {
        assert!(!a.is_empty());
        let n = a.len();
        let mut st = Self { n, tree: vec![0; 4 * n] };
        st.build(a, 0, 0, n - 1);
        st
    }

    pub fn len(&self) -> usize {
        self.n
    }

    /// A[i] += v
    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.n);
        self.add_rec(0, 0, self.n - 1, i, v);
    }

    /// A[i] + A[i+1] + ... + A[j]
    pub fn range_sum(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j && j < self.n);
        self.range_sum_rec(0, 0, self.n - 1, i, j)
    }

    // Node u covers [l, r]; its children are 2u + 1 and 2u + 2.
    fn build(&mut self, a: &[i64], u: usize, l: usize, r: usize) {
        if l == r {
            self.tree[u] = a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(a, 2 * u + 1, l, m);
        self.build(a, 2 * u + 2, m + 1, r);
        self.tree[u] = self.tree[2 * u + 1] + self.tree[2 * u + 2]; // combine
    }

    fn add_rec(&mut self, u: usize, l: usize, r: usize, i: usize, v: i64) {
        if l == r {
            self.tree[u] += v; // the leaf of position i
            return;
        }
        let m = (l + r) / 2;
        if i <= m {
            self.add_rec(2 * u + 1, l, m, i, v);
        } else {
            self.add_rec(2 * u + 2, m + 1, r, i, v);
        }
        self.tree[u] = self.tree[2 * u + 1] + self.tree[2 * u + 2]; // combine
    }

    fn range_sum_rec(&self, u: usize, l: usize, r: usize, i: usize, j: usize) -> i64 {
        if i <= l && r <= j {
            return self.tree[u]; // total overlap
        }
        if r < i || j < l {
            return 0; // no overlap
        }
        // partial overlap
        let m = (l + r) / 2;
        self.range_sum_rec(2 * u + 1, l, m, i, j) + self.range_sum_rec(2 * u + 2, m + 1, r, i, j)
    }
}

impl SegmentTree {
    /// Requires non-negative values. Returns the smallest i such that
    /// A[0] + ... + A[i] >= s, or None if the total sum is smaller than s.
    pub fn search(&self, s: i64) -> Option<usize> {
        if self.tree[0] < s {
            return None; // without this check, the descent would end in the last leaf
        }
        let (mut u, mut l, mut r, mut s) = (0, 0, self.n - 1, s);
        while l < r {
            let m = (l + r) / 2;
            let left = self.tree[2 * u + 1];
            if left >= s {
                u = 2 * u + 1;
                r = m;
            } else {
                s -= left;
                u = 2 * u + 2;
                l = m + 1;
            }
        }
        Some(l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    const A: [i64; 8] = [5, 1, 3, 1, 4, 7, 2, 1]; // running example of the 2026 notes

    #[test]
    fn notes_examples() {
        let mut st = SegmentTree::from_slice(&A);
        assert_eq!(st.len(), 8);
        assert_eq!(st.range_sum(0, 7), 24);
        assert_eq!(st.range_sum(4, 7), 14); // "the node [4,7] stores 14 = 11 + 3"
        assert_eq!(st.range_sum(2, 6), 17); // total overlaps [2,3] + [4,5] + [6,6] = 4 + 11 + 2
        assert_eq!(st.search(20), Some(5)); // prefix sums 5 6 9 10 14 21 23 24
        assert_eq!(st.search(25), None);
        st.add(4, 3); // add(4, 3): the four nodes on the path increase by 3
        assert_eq!(st.range_sum(0, 7), 27);
        assert_eq!(st.range_sum(2, 6), 20);
        // n = 6: "the leaf 4 is node 12" -> needs more than 2n - 1 slots, 4n is safe
        let st6 = SegmentTree::from_slice(&[0, 0, 0, 0, 9, 0]);
        assert_eq!(st6.tree[12], 9);
    }

    #[test]
    fn lecture_board_examples() {
        // L13/L14 board: A = [3, 1, -2, 4, 6, 13, 2, 0], RS(2, 5) = 21, Add(4, +3)
        let mut st = SegmentTree::from_slice(&[3, 1, -2, 4, 6, 13, 2, 0]);
        assert_eq!(st.range_sum(2, 5), 21);
        st.add(4, 3);
        assert_eq!(st.range_sum(2, 5), 24);
        // L14 board, Search(22) = 5 on [3, 1, 0, 4, 6, 13, 2, 0]
        assert_eq!(SegmentTree::from_slice(&[3, 1, 0, 4, 6, 13, 2, 0]).search(22), Some(5));
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x2026_5E67);
        for _ in 0..300 {
            let n = 1 + rng.below(40) as usize;
            let mut a: Vec<i64> = (0..n).map(|_| rng.below(6) as i64).collect();
            let mut st = SegmentTree::from_slice(&a);
            for _ in 0..80 {
                let i = rng.below(n as u64) as usize;
                let j = i + rng.below((n - i) as u64) as usize;
                match rng.below(3) {
                    0 => {
                        let v = rng.below(4) as i64;
                        st.add(i, v);
                        a[i] += v;
                    }
                    1 => assert_eq!(st.range_sum(i, j), a[i..=j].iter().sum::<i64>()),
                    _ => {
                        let s = rng.below(6 * n as u64 + 2) as i64;
                        let mut p = 0;
                        let want = a.iter().position(|&x| {
                            p += x;
                            p >= s
                        });
                        assert_eq!(st.search(s), want, "a={a:?} s={s}");
                    }
                }
            }
        }
    }
}
