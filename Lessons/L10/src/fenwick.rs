//! Fenwick tree (Binary Indexed Tree). `FenwickTree` is the prof's code, copied verbatim from
//! <https://pages.di.unipi.it/rossano/blog/2023/fenwick> (only comments added).
//! In the lecture of 19/10 the Fenwick tree was only started (trivial solutions, relaxed version,
//! fake root; the recording stops there, the rest is in your notes). Tests and `NaiveArray` are [extra].

#[derive(Debug)]
pub struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len() - 1
    }

    /// Indexing is 0-based, even if internally we use 1-based indexing
    // add(i, v): update every node whose range contains i: i, then i + (lowest set bit), ...
    pub fn add(&mut self, i: usize, delta: i64) {
        let mut i = i + 1;
        assert!(i < self.tree.len());

        while i < self.tree.len() {
            self.tree[i] += delta;
            i = Self::next_sibling(i);
        }
    }

    /// Indexing is 0-based, even if internally we use 1-based indexing
    // sum(i): from the node whose range ends at i up to the fake root 0, clearing the lowest set bit.
    pub fn sum(&self, i: usize) -> i64 {
        let mut i = i + 1;

        assert!(i < self.tree.len());
        let mut sum = 0;
        while i != 0 {
            sum += self.tree[i];
            i = Self::parent(i);
        }

        sum
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - if l == 0 { 0 } else { self.sum(l - 1) }
    }

    fn isolate_trailing_one(i: usize) -> usize {
        if i == 0 {
            0
        } else {
            1 << i.trailing_zeros()
        }
    }

    fn parent(i: usize) -> usize {
        i - Self::isolate_trailing_one(i)
    }

    fn next_sibling(i: usize) -> usize {
        i + Self::isolate_trailing_one(i)
    }
}

/// [extra] The two trivial solutions of the lecture (your notes), used as brute force.
pub struct NaiveArray(pub Vec<i64>);
impl NaiveArray {
    /// Theta(i)
    pub fn sum(&self, i: usize) -> i64 {
        self.0[..=i].iter().sum()
    }
    /// O(1)
    pub fn add(&mut self, i: usize, v: i64) {
        self.0[i] += v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn notes_example() {
        // your notes: A = [3,2,-1,5,7,-3,2,1]; nodes at powers of two hold 3, 5, 9, 16;
        // node 6 (0110) covers [5,6] = 7 + (-3) = 4, node 5 (0101) covers [5,5] = 7, node 3 covers [3,3] = -1
        let a = [3, 2, -1, 5, 7, -3, 2, 1];
        let mut ft = FenwickTree::with_len(a.len());
        for (i, &v) in a.iter().enumerate() {
            ft.add(i, v);
        }
        assert_eq!(ft.tree, vec![0, 3, 5, -1, 9, 7, 4, 2, 16]);
        assert_eq!(ft.sum(6), 3 + 2 - 1 + 5 + 7 - 3 + 2); // sum(7) 1-based = nodes 7, 6, 4
        assert_eq!(ft.range_sum(2, 4), -1 + 5 + 7);
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x2545F4914F6CDD1D);
        for _ in 0..100 {
            let n = 1 + rng.below(50) as usize;
            let mut ft = FenwickTree::with_len(n);
            let mut na = NaiveArray(vec![0; n]);
            assert_eq!(ft.len(), n);
            for _ in 0..200 {
                let i = rng.below(n as u64) as usize;
                if rng.below(2) == 0 {
                    let v = rng.below(201) as i64 - 100;
                    ft.add(i, v);
                    na.add(i, v);
                } else {
                    assert_eq!(ft.sum(i), na.sum(i));
                }
            }
        }
    }
}
