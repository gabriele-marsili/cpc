//! Solution #2: sparse table — the PROF's code (board, "Sparse table: the code and an example"),
//! translated line by line from `prof/sparse_table.py`. Comments with "prof:" quote his comments.
//!
//! Key property: the min of a union is the min of the two parts EVEN IF THEY OVERLAP (min is idempotent),
//! so any range [i, j] is covered by the two ranges of length 2^k, k = ⌊log2(j-i+1)⌋, that start in i
//! and end in j: [i, i+2^k-1] and [j-2^k+1, j]. Both are in column k of the table.

pub struct SparseTable<'a, T> {
    a: &'a [T],
    t: Vec<Vec<usize>>, // prof: T[i][k] = RMQ(i, i + 2^k - 1); row i is ragged (no entry once the range leaves A)
}

impl<'a, T: Ord> SparseTable<'a, T> {
    /// prof: build(A). Θ(n log n) time and space.
    pub fn build(a: &'a [T]) -> Self {
        let n = a.len();
        let mut t: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect(); // prof: T[i][0] = i
        let mut k = 1;
        while (1usize << k) <= n {
            let h = 1usize << (k - 1);
            for i in 0..n - (1 << k) + 1 {
                let (x, y) = (t[i][k - 1], t[i + h][k - 1]);
                let m = if a[x] <= a[y] { x } else { y };
                t[i].push(m);
            }
            k += 1;
        }
        SparseTable { a, t }
    }

    /// prof: rmq(A, T, i, j) — both ends included. O(1).
    pub fn rmq(&self, i: usize, j: usize) -> usize {
        // prof: largest 2^k <= j-i+1   (Python's bit_length() - 1 = position of the most significant bit)
        let k = (usize::BITS - 1 - (j - i + 1).leading_zeros()) as usize;
        // prof: T[j - (1 << k) + 1][k]. In Rust (usize) write j + 1 - (1 << k): with i = 0 and j + 1 = 2^k,
        // j - 2^k underflows BEFORE the + 1 (panic in debug). Python has no such problem.
        let (x, y) = (self.t[i][k], self.t[j + 1 - (1 << k)][k]);
        if self.a[x] <= self.a[y] { x } else { y }
    }

    /// The raw table, to check the example of the lecture.
    pub fn table(&self) -> &Vec<Vec<usize>> {
        &self.t
    }

    /// Number of stored positions: Θ(n log n).
    pub fn cells(&self) -> usize {
        self.t.iter().map(|r| r.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LECTURE_A, rmq_naive, rng::Rng};

    #[test]
    fn lecture_table_and_query() {
        let a = LECTURE_A;
        let st = SparseTable::build(&a);
        // the prof filled the table with VALUES ("let's use the min, just not to become crazy")
        let vals: Vec<Vec<i64>> = st.table().iter().map(|r| r.iter().map(|&p| a[p]).collect()).collect();
        assert_eq!(vals[0], vec![5, 2, 2, 1]);
        assert_eq!(vals[1], vec![2, 2, 2]); // the board has a 1 in the grey cell (1, 3): not stored here
        assert_eq!(vals[3][2], 1);
        // RMQ(0, 6): length 7, k = 2, T[0][2] (value 2) and T[3][2] (value 1, row 6 - 4 + 1 = 3): answer 1
        let k = (usize::BITS - 1 - 7usize.leading_zeros()) as usize;
        assert_eq!(k, 2);
        assert_eq!(a[st.table()[0][2]], 2);
        assert_eq!(a[st.table()[3][2]], 1);
        assert_eq!(st.rmq(0, 6), 5);
    }

    #[test]
    fn against_brute_force() {
        let mut r = Rng(1618);
        for _ in 0..300 {
            let n = 1 + r.below(70);
            let a = r.vec(n, -6, 7);
            let st = SparseTable::build(&a);
            for i in 0..n {
                for j in i..n {
                    assert_eq!(st.rmq(i, j), rmq_naive(&a, i, j), "a={a:?} i={i} j={j}");
                }
            }
        }
    }

    #[test]
    fn space_is_n_log_n() {
        let a: Vec<i64> = (0..1024).collect();
        let st = SparseTable::build(&a);
        // sum over k = 0..=10 of (n - 2^k + 1)
        let expected: usize = (0..=10).map(|k| 1024 - (1 << k) + 1).sum();
        assert_eq!(st.cells(), expected);
    }
}
