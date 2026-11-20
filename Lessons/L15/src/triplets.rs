//! Triplets (2025 notes p.12-13): count i < j < k with A[i] < A[j] < A[k].
//! Θ(n³): check every triplet. Better: fix the middle j; with
//!   l_j = #{i < j : A[i] < A[j]} and r_j = #{k > j : A[j] < A[k]}
//! the triplets with that middle are l_j · r_j (2025 example: A = [1,2,3,4,1], j = 2 -> 2 · 1 = 2).
//! Θ(n log n): l_j with a Fenwick tree of frequencies scanning left to right (after remapping the values
//! to ranks, as for the inversions in L12), r_j the same scanning right to left.
//! 2026 notes: the same idea with two SEGMENT trees indexed by value (`count_triplets_st`); "it could be
//! solved with a Fenwick tree as well" (`count_triplets`, as in the 2025 notes).
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

/// The prof's 2026 notes version: two segment trees INDEXED BY VALUE (prof's `SegmentTree`, l15::prof_blog),
/// P = counters of the prefix A[0..j-1], S = counters of the suffix A[j+1..n-1]; at every j:
/// S.add(A[j], -1); L_j = P.range_sum(0, A[j]-1), R_j = S.range_sum(A[j]+1, V-1) (skipped if empty);
/// result += L_j · R_j; P.add(A[j], 1). Values replaced by their ranks, so V <= n. Θ(n log n).
/// [extra] code mine, following the notes step by step.
pub fn count_triplets_st(a: &[i64]) -> u64 {
    use crate::prof_blog::SegmentTree;
    if a.is_empty() {
        return 0;
    }
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let rank: Vec<usize> = a.iter().map(|x| sorted.binary_search(x).unwrap()).collect();
    let v = sorted.len();
    let mut occ = vec![0i64; v];
    for &x in &rank {
        occ[x] += 1;
    }
    let mut p = SegmentTree::from_slice(&vec![0; v]);
    let mut s = SegmentTree::from_slice(&occ);
    let mut total = 0u64;
    for &x in &rank {
        s.add(x, -1);
        let l = if x > 0 { p.range_sum(0, x - 1) } else { 0 };
        let r = if x + 1 < v { s.range_sum(x + 1, v - 1) } else { 0 };
        total += (l * r) as u64;
        p.add(x, 1);
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
        // prof's 2026 notes: A = [5, 1, 3, 1, 4, 7, 2, 1] has 5 triplets (L_2·R_2 = 1·2, L_4·R_4 = 3·1);
        // ranks [4, 0, 2, 0, 3, 5, 1, 0], V = 6
        assert_eq!(count_triplets(&[5, 1, 3, 1, 4, 7, 2, 1]), 5);
        assert_eq!(count_triplets_st(&[5, 1, 3, 1, 4, 7, 2, 1]), 5);
        assert_eq!(count_triplets_st(&[]), 0);
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
            assert_eq!(count_triplets_st(&a), c);
        }
    }
}
