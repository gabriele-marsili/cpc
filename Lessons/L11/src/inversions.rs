//! Counting inversions: pairs (i, j) with i < j and A[i] > A[j] (lecture of 21/10: definition, insertion sort,
//! mergesort variant). The Fenwick-tree solution was NOT done on 21/10: it is in `l12::inversions`.

/// The mergesort variant (lecture of 21/10, board at 84:40-90:20; code [extra]): Θ(n log n). When merging, every time an element of the
/// RIGHT half is moved to the output, it forms an inversion with every element still waiting in the
/// LEFT half (they are all larger), so add their number.
pub fn merge_sort_inversions(a: &[u64]) -> usize {
    fn rec(v: &mut Vec<u64>) -> usize {
        let n = v.len();
        if n <= 1 {
            return 0;
        }
        let mut right = v.split_off(n / 2);
        let mut count = rec(v) + rec(&mut right);
        let left = std::mem::take(v);
        let (mut i, mut j) = (0, 0);
        while i < left.len() || j < right.len() {
            if j == right.len() || (i < left.len() && left[i] <= right[j]) {
                v.push(left[i]);
                i += 1;
            } else {
                count += left.len() - i; // right[j] < all of left[i..]
                v.push(right[j]);
                j += 1;
            }
        }
        count
    }
    rec(&mut a.to_vec())
}

/// [extra] Insertion sort counting the swaps (first idea at the lecture): every swap fixes exactly one
/// inversion (two adjacent elements out of order), so #swaps = #inversions. Θ(n + #inversions), Θ(n^2) worst case.
pub fn insertion_sort_inversions(a: &[u64]) -> usize {
    let mut v = a.to_vec();
    let mut swaps = 0;
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            swaps += 1;
            j -= 1;
        }
    }
    swaps
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
        // your notes: [3,2,1] has 3 inversions; [3,2,1,6,5,4] has 6
        for f in [merge_sort_inversions, insertion_sort_inversions, brute_force] {
            assert_eq!(f(&[3, 2, 1]), 3);
            assert_eq!(f(&[3, 2, 1, 6, 5, 4]), 6);
            assert_eq!(f(&[]), 0);
            assert_eq!(f(&[7]), 0);
            assert_eq!(f(&[1, 2, 3, 4]), 0);
        }
        // board at 84:40: the mergesort tree on [3,2,6,5,4,3,7,8]
        assert_eq!(brute_force(&[3, 2, 6, 5, 4, 3, 7, 8]), merge_sort_inversions(&[3, 2, 6, 5, 4, 3, 7, 8]));
        // a decreasing array of length n has n(n-1)/2 inversions (the maximum)
        let a: Vec<u64> = (1..=10).rev().collect();
        assert_eq!(merge_sort_inversions(&a), 45);
    }

    #[test]
    fn random_vs_brute_force() {
        let mut rng = Rng(0x9E3779B97F4A7C15);
        for _ in 0..300 {
            let n = rng.below(40) as usize;
            let m = 1 + rng.below(30);
            let a: Vec<u64> = (0..n).map(|_| rng.below(m)).collect(); // duplicates and zeros too
            let b = brute_force(&a);
            assert_eq!(insertion_sort_inversions(&a), b, "{a:?}");
            assert_eq!(merge_sort_inversions(&a), b, "{a:?}");
        }
    }
}
