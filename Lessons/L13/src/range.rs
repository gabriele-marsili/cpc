//! Range updates with the Fenwick tree. Prof's code from the blog (sections "Update the Array" and
//! "Dynamic Prefix-sums with Range Update"), copied verbatim: only comments and `pub` on the structs added.
//! All indices are 0-based. Brute force and tests are [extra].
//! Explained in the lecture of 28/10 (L13). `RangeUpdateLecture` [extra] is the board's formulation (1-based).

use crate::fenwick::FenwickTree;

/// Range update + point access (SPOJ UPDATEIT; mandatory in the 19/10/2026 row of the course page,
/// explained in the lecture of 28/10).
/// Same trick as Little Girl (L10): the array B with +v at l and -v at r+1, whose prefix sums are A,
/// but B is now kept in a Fenwick tree, so updates and accesses are both Θ(log n).
#[derive(Debug)]
pub struct UpdateArray {
    ft: FenwickTree,
}

impl UpdateArray {
    pub fn with_len(n: usize) -> Self {
        Self {
            ft: FenwickTree::with_len(n),
        }
    }

    pub fn len(&self) -> usize {
        self.ft.len()
    }

    // A[i] = prefix sum of B up to i
    pub fn access(&self, i: usize) -> i64 {
        self.ft.sum(i)
    }

    pub fn range_update(&mut self, l: usize, r: usize, v: i64) {
        assert!(l <= r);
        assert!(r < self.ft.len());

        self.ft.add(l, v);
        if r + 1 < self.ft.len() {
            self.ft.add(r + 1, -v);
        }
    }
}

/// Range update + prefix sums. sum(i) = a * i + b, where a = ft1.sum(i) and b = ft2.sum(i):
/// after range_update(l, r, v), for l <= i <= r the sum grows by v*(i - l + 1) = v*i - v*(l - 1);
/// for i > r it grows by v*(r - l + 1) = (v - v)*i + (-v*(l - 1) + v*r).
#[derive(Debug)]
pub struct RangeUpdate {
    ft1: FenwickTree,
    ft2: FenwickTree,
}

impl RangeUpdate {
    pub fn with_len(n: usize) -> Self {
        Self {
            ft1: FenwickTree::with_len(n),
            ft2: FenwickTree::with_len(n),
        }
    }

    pub fn len(&self) -> usize {
        self.ft1.len()
    }

    pub fn sum(&self, i: usize) -> i64 {
        self.ft1.sum(i) * i as i64 + self.ft2.sum(i)
    }

    pub fn access(&self, i: usize) -> i64 {
        self.sum(i) - if i == 0 { 0 } else { self.sum(i - 1) }
    }

    pub fn add(&mut self, i: usize, v: i64) {
        self.range_update(i, i, v)
    }

    pub fn range_update(&mut self, l: usize, r: usize, v: i64) {
        self.ft1.add(l, v);

        self.ft2.add(l, -v * (l as i64 - 1));

        if r + 1 < self.len() {
            self.ft1.add(r + 1, -v);
            self.ft2.add(r + 1, v * r as i64);
        }
    }
}

/// [extra] The formulation of the LECTURE of 28/10 (board 37:00-66:00), with two "range update + access"
/// structures, 1-based positions as on the board:
///   Sum(i) = i * FT1.Access(i) - Err.Access(i)
/// RangeUpdate(l, r, v) does three range updates:
/// - FT1.RangeUpdate(l, r, v): the values v, v', v'' covering i;
/// - Err.RangeUpdate(l, r, v * (l - 1)): inside, i*v counts l-1 cells too many;
/// - Err.RangeUpdate(r + 1, n, -v * (r - l + 1)): after, i*0 misses the whole range.
/// It is the prof's RangeUpdate (blog) with ft2 = -Err. Θ(log n) per operation, twice the space.
pub struct RangeUpdateLecture {
    ft1: UpdateArray,
    err: UpdateArray,
    n: usize,
}

impl RangeUpdateLecture {
    /// positions 1..=n
    pub fn new(n: usize) -> Self {
        Self { ft1: UpdateArray::with_len(n + 1), err: UpdateArray::with_len(n + 1), n }
    }
    /// A[l..=r] += v, 1 <= l <= r <= n
    pub fn range_update(&mut self, l: usize, r: usize, v: i64) {
        self.ft1.range_update(l, r, v);
        self.err.range_update(l, r, v * (l as i64 - 1));
        if r < self.n {
            self.err.range_update(r + 1, self.n, -v * (r - l + 1) as i64);
        }
    }
    /// A[1] + ... + A[i], 0 <= i <= n
    pub fn sum(&self, i: usize) -> i64 {
        if i == 0 { 0 } else { i as i64 * self.ft1.access(i) - self.err.access(i) }
    }
    /// A[i] = Sum(i) - Sum(i - 1): Access reduces to Sum (the reduction shown in class)
    pub fn access(&self, i: usize) -> i64 {
        self.sum(i) - self.sum(i - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn small_example() {
        let mut ua = UpdateArray::with_len(5);
        ua.range_update(1, 3, 2); // [0,2,2,2,0]
        ua.range_update(0, 1, 5); // [5,7,2,2,0]
        assert_eq!((0..5).map(|i| ua.access(i)).collect::<Vec<_>>(), vec![5, 7, 2, 2, 0]);
        assert_eq!(ua.len(), 5);

        let mut ru = RangeUpdate::with_len(5);
        ru.range_update(1, 3, 2);
        ru.range_update(0, 1, 5);
        assert_eq!((0..5).map(|i| ru.access(i)).collect::<Vec<_>>(), vec![5, 7, 2, 2, 0]);
        assert_eq!(ru.sum(4), 16);
        ru.add(4, -1);
        assert_eq!(ru.sum(4), 15);
    }

    #[test]
    fn lecture_examples() {
        // Update the Array, board 0:00-3:40 (0-based there): Update(2,6,10), Update(0,4,5), Access(2) = 15
        let mut ua = UpdateArray::with_len(9);
        ua.range_update(2, 6, 10);
        ua.range_update(0, 4, 5);
        assert_eq!((0..9).map(|i| ua.access(i)).collect::<Vec<_>>(), vec![5, 5, 15, 15, 15, 10, 10, 0, 0]);
        // range update + sum, board 25:00 (1-based): RangeUpdate(4,6,10) -> Sum(6) = 30
        let mut r = RangeUpdateLecture::new(10);
        r.range_update(4, 6, 10);
        assert_eq!(r.sum(6), 30);
        // board 40:00-66:00 (1-based, n = 8): RangeUpdate(2,7,10)
        let mut r = RangeUpdateLecture::new(8);
        r.range_update(2, 7, 10);
        assert_eq!(r.ft1.access(6), 10);
        assert_eq!(r.err.access(6), 10); // error inside the range: v * (l - 1)
        assert_eq!(r.sum(6), 6 * 10 - 10); // 50
        assert_eq!(r.err.access(8), -60); // after the range: -(r - l + 1) * v
        assert_eq!(r.sum(8), 60);
        assert_eq!(r.sum(1), 0);
        // then RangeUpdate(3,4,5)
        r.range_update(3, 4, 5);
        assert_eq!(r.sum(6), 60); // 10*6 - (10 - 10)
        assert_eq!(r.sum(4), 40); // 15*4 - 20
        // the first, wrong attempt: Sum(i) = i * FT1.Access(i) only. With RangeUpdate(2,7,10), (4,7,5):
        // 6 * 15 = 90 instead of 65 (board 45:40)
        let mut w = RangeUpdateLecture::new(8);
        w.range_update(2, 7, 10);
        w.range_update(4, 7, 5);
        assert_eq!(6 * w.ft1.access(6), 90);
        assert_eq!(w.sum(6), 65);
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x2545F4914F6CDD1D);
        for _ in 0..200 {
            let n = 1 + rng.below(30) as usize;
            let mut naive = vec![0i64; n];
            let mut ua = UpdateArray::with_len(n);
            let mut ru = RangeUpdate::with_len(n);
            for _ in 0..100 {
                let l = rng.below(n as u64) as usize;
                let r = l + rng.below((n - l) as u64) as usize;
                match rng.below(4) {
                    0 => {
                        let v = rng.below(21) as i64 - 10;
                        ua.range_update(l, r, v);
                        ru.range_update(l, r, v);
                        for x in &mut naive[l..=r] {
                            *x += v;
                        }
                    }
                    1 => {
                        let v = rng.below(21) as i64 - 10;
                        // point add on both (UpdateArray: a range of length 1)
                        ua.range_update(l, l, v);
                        ru.add(l, v);
                        naive[l] += v;
                    }
                    2 => {
                        assert_eq!(ua.access(l), naive[l]);
                        assert_eq!(ru.access(l), naive[l]);
                    }
                    _ => assert_eq!(ru.sum(r), naive[..=r].iter().sum::<i64>()),
                }
            }
        }
    }

    #[test]
    fn lecture_version_vs_naive() {
        let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
        for _ in 0..200 {
            let n = 1 + rng.below(30) as usize;
            let mut naive = vec![0i64; n + 1];
            let mut r = RangeUpdateLecture::new(n);
            for _ in 0..100 {
                let l = 1 + rng.below(n as u64) as usize;
                let h = l + rng.below((n - l + 1) as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(21) as i64 - 10;
                    r.range_update(l, h, v);
                    for x in &mut naive[l..=h] {
                        *x += v;
                    }
                } else {
                    assert_eq!(r.sum(h), naive[1..=h].iter().sum::<i64>());
                    assert_eq!(r.access(l), naive[l]);
                }
            }
        }
    }
}
