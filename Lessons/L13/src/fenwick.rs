//! Fenwick tree (Binary Indexed Tree). `FenwickTree` is the prof's code, copied verbatim from
//! <https://pages.di.unipi.it/rossano/blog/2023/fenwick> (only comments added), same as `l10::fenwick`.
//! Copied here so that the crate `l13` is self-contained (same as `l11::fenwick`).

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lecture_example() {
        // A = [3,2,-1,5,7,-3,2,1] (L10/L11, used again in the 2025 notes). Sum(7) (1-based) = nodes 7, 6, 4 = 2 + 4 + 9 = 15 (your notes).
        let a = [3, 2, -1, 5, 7, -3, 2, 1];
        let mut ft = FenwickTree::with_len(a.len());
        for (i, &v) in a.iter().enumerate() {
            ft.add(i, v);
        }
        assert_eq!(ft.tree, vec![0, 3, 5, -1, 9, 7, 4, 2, 16]);
        assert_eq!(ft.sum(6), 15);
        // Add(5, 10) (1-based) touches nodes 5 = 0101, 6 = 0110, 8 = 1000 (your notes)
        ft.add(4, 10);
        assert_eq!(ft.tree, vec![0, 3, 5, -1, 9, 17, 14, 2, 26]);
    }

    #[test]
    fn parent_and_sibling() {
        // your notes: 7 -> 6 -> 4 -> 0 (clear the lowest set bit); 5 -> 6 -> 8 (add it)
        assert_eq!(FenwickTree::parent(7), 6);
        assert_eq!(FenwickTree::parent(6), 4);
        assert_eq!(FenwickTree::parent(4), 0);
        assert_eq!(FenwickTree::next_sibling(5), 6);
        assert_eq!(FenwickTree::next_sibling(6), 8);
        // i & -i in two's complement isolates the lowest set bit (your notes)
        for i in 1usize..1000 {
            assert_eq!(FenwickTree::isolate_trailing_one(i), i & i.wrapping_neg());
        }
    }
}
