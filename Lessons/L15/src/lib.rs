//! L15 (Wednesday 04/11/2026) — PROVISIONAL RECONSTRUCTION: no recording, no notes of Octech yet.
//! Contents guessed from where L14 stopped ("next lecture we conclude the segment tree presenting many
//! other problems, starting from #occs") and from the prof's 2025 notes `Lessons/L13/Segment_tree_2025.pdf`
//! p.9-19, in their order. Notes in L15.pdf. To be redone on the recording when it comes out.
//!
//! - `occs`: #occs(i, j, x) with a hash map in every node (2025 p.9)
//! - `successor`: Successor(i, j, x) with a sorted multiset (BST) in every node (2025 p.10-11)
//! - `triplets`: count i < j < k with A[i] < A[j] < A[k], Θ(n log n) with Fenwick trees (2025 p.12-13)
//! - `lazy`: RangeUpdate(i, j, v) + RangeSum with lazy propagation (2025 p.14-15)
//! - `persistent`: Add + RangeSum(i, j, t) on every past version, path copying (2025 p.16-19)
//!
//! [extra] All the code is mine (the prof has no code for these); 0-based positions as on the board.

pub mod lazy;
pub mod occs;
pub mod persistent;
pub mod successor;
pub mod triplets;

#[cfg(test)]
pub(crate) mod rng {
    pub struct Rng(pub u64);
    impl Rng {
        pub fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        pub fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
    }
}
