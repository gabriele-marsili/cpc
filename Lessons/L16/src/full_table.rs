//! Solution #1: "how many distinct questions? n(n+1)/2 ranges: tabulate them all".
//! T[i][j] = position of the minimum of A[i..=j]. Query O(1), space Θ(n²).
//! Build: computing each cell by a scan is Θ(n³); the prof's way is ROW-WISE: fix the start i and move
//! the end j one step at a time, updating the answer (the trick of sliding window maxima): Θ(n²),
//! which is optimal because there are Θ(n²) cells to write.

pub struct FullTable {
    t: Vec<Vec<usize>>, // t[i][j - i]: only j >= i is stored
}

impl FullTable {
    pub fn new<T: Ord>(a: &[T]) -> Self {
        let n = a.len();
        let mut t = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = Vec::with_capacity(n - i);
            let mut best = i;
            for j in i..n {
                if a[j] < a[best] {
                    best = j; // T[i][j] from T[i][j-1] and A[j] in O(1)
                }
                row.push(best);
            }
            t.push(row);
        }
        FullTable { t }
    }

    /// Position of the (leftmost) minimum of A[i..=j]. O(1).
    pub fn rmq(&self, i: usize, j: usize) -> usize {
        self.t[i][j - i]
    }

    /// Number of stored cells: n(n+1)/2.
    pub fn cells(&self) -> usize {
        self.t.iter().map(|r| r.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LECTURE_A, rmq_naive, rng::Rng};

    #[test]
    fn lecture_example() {
        let ft = FullTable::new(&LECTURE_A);
        assert_eq!(LECTURE_A[ft.rmq(0, 6)], 1);
        assert_eq!(ft.cells(), 8 * 9 / 2);
    }

    #[test]
    fn against_brute_force() {
        let mut r = Rng(16);
        for _ in 0..200 {
            let n = 1 + r.below(30);
            let a = r.vec(n, -4, 5);
            let ft = FullTable::new(&a);
            for i in 0..n {
                for j in i..n {
                    assert_eq!(ft.rmq(i, j), rmq_naive(&a, i, j));
                }
            }
        }
    }
}
