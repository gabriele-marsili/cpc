//! [extra] Nested Segments (Codeforces 652D, mandatory problem of 21/10 on the course page).
//! n segments [l_i, r_i] with all 2n endpoints distinct: for each segment, how many segments it contains.
//! Explained in the lecture of 26/10 (second hour). Three equivalent Θ(n log n) solutions, all coded by me (no code
//! from the prof): `nested_segments_2026` (THE ONE OF THE LECTURE: Fenwick tree on starts, decreasing ends, remove),
//! `nested_segments_2025` (2025 notes: increasing ends, insert starts) and `nested_segments` (blog: increasing starts,
//! Fenwick tree on ends).
//!
//! Idea (blog): sweep the segments by increasing left endpoint, with a Fenwick tree over the right
//! endpoints (ranks) of the segments not yet swept. Initially +1 at every right endpoint. When we reach
//! segment i, all the segments still in the tree start after l_i; those contained in i are exactly the
//! ones with r < r_i, i.e. `sum(rank(r_i) - 1)`. Then remove i: `add(rank(r_i), -1)`. Θ(n log n).

use crate::fenwick::FenwickTree;

pub fn nested_segments(segs: &[(i64, i64)]) -> Vec<usize> {
    let n = segs.len();
    // ranks of the right endpoints (coordinates can be up to 1e9)
    let mut rights: Vec<i64> = segs.iter().map(|&(_, r)| r).collect();
    rights.sort_unstable();
    let rank = |r: i64| rights.binary_search(&r).unwrap();

    let mut ft = FenwickTree::with_len(n.max(1));
    for &(_, r) in segs {
        ft.add(rank(r), 1);
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by_key(|&i| segs[i].0);

    let mut ans = vec![0; n];
    for i in order {
        let k = rank(segs[i].1);
        ans[i] = if k == 0 { 0 } else { ft.sum(k - 1) as usize };
        ft.add(k, -1);
    }
    ans
}

/// The solution derived in the lecture of 26/10 (board 76:00-83:00; code [extra]).
/// ONE Fenwick tree on the (remapped) coordinates with a 1 at the START of every segment. Process the segments
/// by DECREASING end. For segment [l, r]: RangeSum(l, r) counts itself plus the segments starting inside it
/// that are still in the tree; those ending after r have already been processed and have removed their 1,
/// so they are not counted ("Santa Claus"). Answer = RangeSum(l, r) - 1, then remove our own 1. Θ(n log n).
/// Board example: [9,12] -> 1-1 = 0; [7,11] -> 1-1 = 0 (it would be 2 without removing 9); [2,10] -> 3-1 = 2.
pub fn nested_segments_2026(segs: &[(i64, i64)]) -> Vec<usize> {
    let n = segs.len();
    // "remap the segments' extremes, if needed": the 2n endpoints become 0..2n
    let mut coords: Vec<i64> = segs.iter().flat_map(|&(l, r)| [l, r]).collect();
    coords.sort_unstable();
    let pos = |x: i64| coords.binary_search(&x).unwrap();

    let mut ft = FenwickTree::with_len((2 * n).max(1));
    for &(l, _) in segs {
        ft.add(pos(l), 1);
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by_key(|&i| std::cmp::Reverse(segs[i].1));

    let mut ans = vec![0; n];
    for i in order {
        let (l, r) = (pos(segs[i].0), pos(segs[i].1));
        ans[i] = (ft.range_sum(l, r) - 1) as usize; // -1: our own start
        ft.add(l, -1); // before leaving, remove our 1
    }
    ans
}

/// [extra] The version of the 2025 notes (p.9-10), in code. Fenwick tree indexed by (remapped) coordinates,
/// with a 1 at the LEFT endpoint of every segment already processed; segments processed by increasing RIGHT
/// endpoint. When segment i = [l, r] is processed, the segments in the tree end before r, so those contained in
/// i are exactly the ones starting inside it: `RangeSum(l + 1, r - 1)`. Then add 1 at l. Θ(n log n).
/// (Board 2025: with ALL left endpoints in the tree, RangeSum(3, 9) = 4 for [2, 10] also counts [7, 12] and
/// [9, 11], which end after 10; inserting a segment only after it is processed fixes it: RangeSum(3, 9) = 2.)
pub fn nested_segments_2025(segs: &[(i64, i64)]) -> Vec<usize> {
    let n = segs.len();
    // remap all the 2n endpoints to 0..2n ("remap the extremes if needed")
    let mut coords: Vec<i64> = segs.iter().flat_map(|&(l, r)| [l, r]).collect();
    coords.sort_unstable();
    let pos = |x: i64| coords.binary_search(&x).unwrap();

    let mut ft = FenwickTree::with_len((2 * n).max(1));
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by_key(|&i| segs[i].1);

    let mut ans = vec![0; n];
    for i in order {
        let (l, r) = (pos(segs[i].0), pos(segs[i].1));
        ans[i] = ft.range_sum(l + 1, r - 1) as usize; // r > l, endpoints distinct: l + 1 <= r - 1 + 1
        ft.add(l, 1);
    }
    ans
}

/// [extra] Brute force Θ(n^2).
pub fn brute_force(segs: &[(i64, i64)]) -> Vec<usize> {
    segs.iter()
        .map(|&(l, r)| {
            segs.iter()
                .filter(|&&(l2, r2)| l < l2 && r2 < r)
                .count()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn codeforces_examples() {
        // examples of CF 652D
        let s = [(1, 8), (2, 3), (4, 7), (5, 6)];
        assert_eq!(nested_segments(&s), vec![3, 0, 1, 0]);
        assert_eq!(nested_segments_2025(&s), vec![3, 0, 1, 0]);
        assert_eq!(nested_segments_2026(&s), vec![3, 0, 1, 0]);
        let s = [(3, 4), (1, 5), (2, 6)];
        assert_eq!(nested_segments(&s), vec![0, 1, 1]);
        assert_eq!(nested_segments_2025(&s), vec![0, 1, 1]);
        assert_eq!(nested_segments(&[]), Vec::<usize>::new());
        assert_eq!(nested_segments_2025(&[]), Vec::<usize>::new());
        assert_eq!(nested_segments_2026(&s), vec![0, 1, 1]);
        assert_eq!(nested_segments_2026(&[]), Vec::<usize>::new());
    }

    #[test]
    fn example_lecture_2026() {
        // board of 26/10 (coordinates already remapped to 1..12):
        // [1,5] [2,10] [3,4] [6,8] [7,11] [9,12]; [2,10] contains [3,4] and [6,8]; [1,5] contains [3,4]
        let s = [(1, 5), (2, 10), (3, 4), (6, 8), (7, 11), (9, 12)];
        let expected = vec![1, 2, 0, 0, 0, 0];
        assert_eq!(brute_force(&s), expected);
        assert_eq!(nested_segments_2026(&s), expected);
        assert_eq!(nested_segments_2025(&s), expected);
        assert_eq!(nested_segments(&s), expected);
    }

    #[test]
    fn example_2025_notes() {
        // 2025 notes p.10: [2, 10] contains [3, 4] and [6, 8] only
        let s = [(1, 5), (7, 12), (2, 10), (3, 4), (6, 8), (9, 11)];
        let expected = vec![1, 1, 2, 0, 0, 0];
        assert_eq!(brute_force(&s), expected);
        assert_eq!(nested_segments(&s), expected);
        assert_eq!(nested_segments_2025(&s), expected);
    }

    #[test]
    fn random_vs_brute_force() {
        let mut rng = Rng(0xD1B54A32D192ED03);
        for _ in 0..300 {
            let n = rng.below(15) as usize;
            // 2n distinct endpoints: a random permutation of 0..2n, paired up
            let mut p: Vec<i64> = (0..2 * n as i64).map(|x| x * 1_000_000 - 7).collect();
            for i in (1..p.len()).rev() {
                let j = rng.below(i as u64 + 1) as usize;
                p.swap(i, j);
            }
            let segs: Vec<(i64, i64)> = p
                .chunks(2)
                .map(|c| (c[0].min(c[1]), c[0].max(c[1])))
                .collect();
            assert_eq!(nested_segments(&segs), brute_force(&segs), "{segs:?}");
            assert_eq!(nested_segments_2025(&segs), brute_force(&segs), "{segs:?}");
            assert_eq!(nested_segments_2026(&segs), brute_force(&segs), "{segs:?}");
        }
    }
}
