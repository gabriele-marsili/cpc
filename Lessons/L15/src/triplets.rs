//! Triplets (2025 notes p.12-13): count i < j < k with A[i] < A[j] < A[k].
//! Θ(n³): check every triplet. Better: fix the middle j; with
//!   l_j = #{i < j : A[i] < A[j]} and r_j = #{k > j : A[j] < A[k]}
//! the triplets with that middle are l_j · r_j (2025 example: A = [1,2,3,4,1], j = 2 -> 2 · 1 = 2).
//! Θ(n log n): l_j with a Fenwick tree of frequencies scanning left to right (after remapping the values
//! to ranks, as for the inversions in L12), r_j the same scanning right to left.
//! [extra] code mine; Θ(n²) version too, for the tests.

struct Fenwick(Vec<u64>);

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick(vec![0; n + 1])
    }
    fn add(&mut self, mut i: usize) {
        i += 1;
        while i < self.0.len() {
            self.0[i] += 1;
            i += i & i.wrapping_neg();
        }
    }
    /// how many inserted values have rank < i
    fn below(&self, mut i: usize) -> u64 {
        let mut s = 0;
        while i > 0 {
            s += self.0[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

/// Θ(n log n)
pub fn count_triplets(a: &[i64]) -> u64 {
    let n = a.len();
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let rank: Vec<usize> = a.iter().map(|x| sorted.binary_search(x).unwrap()).collect();
    let m = sorted.len();
    let mut left = vec![0u64; n];
    let mut ft = Fenwick::new(m);
    for j in 0..n {
        left[j] = ft.below(rank[j]); // strictly smaller, to the left
        ft.add(rank[j]);
    }
    let mut ft = Fenwick::new(m);
    let mut total = 0;
    for j in (0..n).rev() {
        let seen = (n - 1 - j) as u64;
        let right = seen - ft.below(rank[j] + 1); // strictly larger, to the right
        total += left[j] * right;
        ft.add(rank[j]);
    }
    total
}

/// Θ(n²): the same counting with a scan for every middle j
pub fn count_triplets_quadratic(a: &[i64]) -> u64 {
    (0..a.len())
        .map(|j| {
            let l = a[..j].iter().filter(|&&x| x < a[j]).count() as u64;
            let r = a[j + 1..].iter().filter(|&&x| x > a[j]).count() as u64;
            l * r
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    fn cubic(a: &[i64]) -> u64 {
        let n = a.len();
        let mut c = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    if a[i] < a[j] && a[j] < a[k] {
                        c += 1;
                    }
                }
            }
        }
        c
    }

    #[test]
    fn example_2025_notes() {
        // A = [1, 2, 3, 4, 1]: (1,2,3), (1,2,4), (2,3,4), (1,3,4) -> 4
        assert_eq!(count_triplets(&[1, 2, 3, 4, 1]), 4);
        assert_eq!(count_triplets(&[]), 0);
        assert_eq!(count_triplets(&[5, 5, 5]), 0);
    }

    #[test]
    fn random_vs_cubic() {
        let mut rng = Rng(0x7219_1E75);
        for _ in 0..300 {
            let n = rng.below(25) as usize;
            let a: Vec<i64> = (0..n).map(|_| rng.below(8) as i64 - 3).collect();
            let c = cubic(&a);
            assert_eq!(count_triplets(&a), c, "{a:?}");
            assert_eq!(count_triplets_quadratic(&a), c);
        }
    }
}
