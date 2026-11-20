//! Solution #3: split A into blocks of b ≈ log n elements, A'[t] = minimum of block t (|A'| = n / log n),
//! and build the sparse table on A': (n / log n) · log n = Θ(n) space.
//! Query: cut [i, j] in three parts — the blocks fully inside (O(1) with the sparse table on A'),
//! the prefix and the suffix inside the two border blocks (scanned: O(log n)). Query Θ(log n).
//! The prof: "essentially useless" asymptotically — the segment tree already gives Θ(log n) and Θ(n) —
//! but it is the stepping stone to Solution #4 (only the in-block queries remain to be solved in O(1)).
//! [extra] the case "i and j in the same block / adjacent blocks" (no middle part) is mine: not discussed.

use crate::sparse::SparseTable;

/// The blocks and their minima (A'); `BlockRmqFast` adds the sparse table on A'.
pub struct BlockRmq<'a, T> {
    a: &'a [T],
    b: usize,
    mins: Vec<usize>, // position (in A) of the minimum of each block
    vals: Vec<&'a T>, // A'[t] = A[mins[t]]
}

impl<'a, T: Ord> BlockRmq<'a, T> {
    pub fn new(a: &'a [T]) -> Self {
        let n = a.len().max(2);
        let b = (usize::BITS - n.leading_zeros()) as usize; // ≈ log2 n, at least 2
        let mut mins = Vec::new();
        for s in (0..a.len()).step_by(b) {
            let e = (s + b - 1).min(a.len() - 1);
            mins.push(crate::rmq_naive(a, s, e));
        }
        let vals = mins.iter().map(|&p| &a[p]).collect();
        BlockRmq { a, b, mins, vals }
    }

    pub fn block_size(&self) -> usize {
        self.b
    }

    fn scan(&self, i: usize, j: usize) -> usize {
        crate::rmq_naive(self.a, i, j)
    }
}

/// Solution #3: blocks + sparse table on A' (built once).
pub struct BlockRmqFast<'a, T> {
    inner: BlockRmq<'a, T>,
    st: Vec<Vec<usize>>,
}

impl<'a, T: Ord> BlockRmqFast<'a, T> {
    pub fn new(a: &'a [T]) -> Self {
        let inner = BlockRmq::new(a);
        let st = SparseTable::build(&inner.vals).table().clone();
        BlockRmqFast { inner, st }
    }

    pub fn rmq(&self, i: usize, j: usize) -> usize {
        let s = &self.inner;
        let (bi, bj) = (i / s.b, j / s.b);
        if bi == bj {
            return s.scan(i, j);
        }
        let mut best = s.scan(i, (bi + 1) * s.b - 1);
        if bi + 1 < bj {
            let (l, r) = (bi + 1, bj - 1);
            let k = (usize::BITS - 1 - (r - l + 1).leading_zeros()) as usize;
            let (x, y) = (self.st[l][k], self.st[r + 1 - (1 << k)][k]);
            let t = if s.vals[x] <= s.vals[y] { x } else { y };
            let m = s.mins[t];
            if s.a[m] < s.a[best] {
                best = m;
            }
        }
        let suf = s.scan(bj * s.b, j);
        if s.a[suf] < s.a[best] { suf } else { best }
    }

    /// Stored positions: block minima + sparse table on n / b elements = Θ(n).
    pub fn cells(&self) -> usize {
        self.inner.mins.len() + self.st.iter().map(|r| r.len()).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LECTURE_A, rmq_naive, rng::Rng};

    #[test]
    fn lecture_example() {
        let q = BlockRmqFast::new(&LECTURE_A);
        assert_eq!(q.rmq(0, 6), 5);
    }

    #[test]
    fn against_brute_force() {
        let mut r = Rng(31415);
        for _ in 0..300 {
            let n = 1 + r.below(90);
            let a = r.vec(n, -5, 6);
            let q = BlockRmqFast::new(&a);
            for i in 0..n {
                for j in i..n {
                    let want = rmq_naive(&a, i, j);
                    assert_eq!(q.rmq(i, j), want, "a={a:?} i={i} j={j}");
                }
            }
        }
    }

    #[test]
    fn linear_space() {
        for &n in &[1usize << 10, 1 << 14, 1 << 17] {
            let a: Vec<i64> = (0..n as i64).rev().collect();
            let q = BlockRmqFast::new(&a);
            assert!(q.cells() <= 3 * n, "n={n} cells={}", q.cells());
        }
    }
}
