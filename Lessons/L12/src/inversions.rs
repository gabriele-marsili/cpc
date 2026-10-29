//! Counting inversions with a Fenwick tree (lecture of 26/10, first 40 min; blog; 2025 notes p.7-8).
//! The mergesort solution is in `l11::inversions`.

use crate::fenwick::FenwickTree;

/// Prof's code (blog, "Counting Inversions in an Array"), verbatim.
/// Scan A left to right; the Fenwick tree has a 1 in position v for every value v already seen.
/// Before inserting e, the elements seen so far that are larger than e are `range_sum(e + 1, max)`.
/// Θ(n log M) with M = max value; if M is not O(n), replace the values by their ranks first.
pub fn counting_inversions(a: &[u64]) -> usize {
    if a.is_empty() {
        return 0;
    }

    let max = *a.iter().max().unwrap() as usize;
    let mut ft = FenwickTree::with_len(max + 1);

    let mut count: usize = 0;
    for &e in a {
        // [extra] when e == max this is range_sum(max + 1, max) = sum(max) - sum(max) = 0: correct
        count += ft.range_sum((e + 1) as usize, max) as usize;
        ft.add(e as usize, 1);
    }

    count
}

/// Remapping (2025 notes p.8, "if max value is too big, remap"; code [extra]): sort the pairs
/// (value, position) in increasing order, Θ(n log n), and replace each value with its rank.
/// As in the notes, ties are broken by position, so the result is a permutation of 1..=n:
/// [1, 50, 1_000_000_000, 155, 6, 6] -> [1, 4, 6, 5, 2, 3].
/// Equal values get increasing ranks from left to right, so they still do NOT form an inversion,
/// and `a[i] > a[j]` iff `rank[i] > rank[j]`: the number of inversions is preserved, and max = n.
pub fn remap(a: &[u64]) -> Vec<u64> {
    let mut order: Vec<(u64, usize)> = a.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    order.sort_unstable();
    let mut rank = vec![0; a.len()];
    for (r, &(_, pos)) in order.iter().enumerate() {
        rank[pos] = r as u64 + 1;
    }
    rank
}

/// [extra] Counting inversions for arbitrary values (up to 1e18): remap, then the prof's code.
/// Θ(n log n) time and Θ(n) space, independently of the maximum value.
pub fn counting_inversions_any(a: &[u64]) -> usize {
    counting_inversions(&remap(a))
}

/// [extra] Brute force Θ(n^2).
pub fn brute_force(a: &[u64]) -> usize {
    let mut c = 0;
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[i] > a[j] {
                c += 1;
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn lecture_examples() {
        // 2025 notes p.7: A = [3,2,1,6,5,4] has 6 inversions; when e = A[3] = 1 is processed,
        // RangeSum(1 + 1, 6) = 2 (the 3 and the 2 already seen)
        for f in [counting_inversions, counting_inversions_any, brute_force] {
            assert_eq!(f(&[3, 2, 1, 6, 5, 4]), 6);
            assert_eq!(f(&[3, 2, 1]), 3);
            assert_eq!(f(&[]), 0);
            assert_eq!(f(&[7]), 0);
        }
        let a: Vec<u64> = (1..=10).rev().collect();
        assert_eq!(counting_inversions(&a), 45);
    }

    #[test]
    fn lecture_examples_2026() {
        // board 26/10: A = [3,2,1,6,5,4,3,2] (values up to 6 but 8 positions: F is indexed by VALUE)
        let a = [3, 2, 1, 6, 5, 4, 3, 2];
        assert_eq!(counting_inversions(&a), brute_force(&a));
        assert_eq!(counting_inversions(&a), 14);
        // remap example of the lecture: [1, 150, 1_000_000, 155, 6, 8]; sorted 1, 6, 8, 150, 155, 1e6 -> ranks 1..6
        let a = [1, 150, 1_000_000, 155, 6, 8];
        assert_eq!(remap(&a), vec![1, 4, 6, 5, 2, 3]);
        assert_eq!(counting_inversions_any(&a), brute_force(&a));
        // the WRONG remap discussed in the lecture: ids in order of first appearance (hash map + counter)
        // gives [1,2,3,4,5,6], which has 0 inversions: the relative order is not preserved
        assert_eq!(counting_inversions(&[1, 2, 3, 4, 5, 6]), 0);
        assert_ne!(counting_inversions_any(&a), 0);
    }

    #[test]
    fn remap_example_2025() {
        // 2025 notes p.8: A = [1, 50, 1_000_000_000, 155, 6, 6]
        let a = [1, 50, 1_000_000_000, 155, 6, 6];
        // the ordering table on the board: (1,1) (6,5) (6,6) (50,2) (155,4) (1e9,3) -> ranks 1..6
        assert_eq!(remap(&a), vec![1, 4, 6, 5, 2, 3]);
        assert_eq!(counting_inversions_any(&a), brute_force(&a));
        assert_eq!(counting_inversions_any(&a), 7); // 50>6,6; 1e9>155,6,6; 155>6,6
    }

    #[test]
    fn random_vs_brute_force() {
        let mut rng = Rng(0x9E3779B97F4A7C15);
        for _ in 0..300 {
            let n = rng.below(40) as usize;
            let m = 1 + rng.below(30);
            let a: Vec<u64> = (0..n).map(|_| rng.below(m)).collect();
            assert_eq!(counting_inversions(&a), brute_force(&a), "{a:?}");
            let big: Vec<u64> = a.iter().map(|x| x * 1_000_000_007 + 3).collect();
            assert_eq!(counting_inversions_any(&big), brute_force(&a), "{big:?}");
        }
    }
}
