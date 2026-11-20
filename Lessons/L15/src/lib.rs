//! L15 — segment tree (end) and Mo's algorithm. Notes in L15.pdf.
//! Part I (#occs, successor, triplets, lazy): from the prof's 2026 notes "Segment Tree" (not in the recording;
//! probably the unrecorded lecture of 04/11). Part II (persistent segment tree, Mo's algorithm): from the
//! recording L_15.MP4 (recorded on 16/11/2026, file metadata) and the prof's notes.
//!
//! - `occs`: #occs(i, j, x) with a hash map in every node (2025 p.9)
//! - `successor`: Successor(i, j, x) with a sorted multiset (BST) in every node (2025 p.10-11)
//! - `triplets`: count i < j < k with A[i] < A[j] < A[k], Θ(n log n) with Fenwick trees (2025 p.12-13)
//! - `lazy`: RangeUpdate(i, j, v) + RangeSum with lazy propagation (2025 p.14-15)
//! - `persistent`: Add + RangeSum(i, j, t) on every past version, path copying (lecture + notes)
//! - `mo`: Mo's algorithm, the PROF's code from the blog "Mo's Algorithm" (2023), verbatim, with tests on the lecture
//! - `past`: "why query the past" — count(i, j, x) and k-th smallest with versions indexed by value (2026)
//! - `prof_blog`: the PROF's code from the 2026 notes, verbatim (SegmentTree, LazySegmentTree,
//!   PersistentSegmentTree), with tests on all the numbers of his examples
//!
//! [extra] Everything except `prof_blog` is mine; 0-based positions.

pub mod lazy;
pub mod mo;
pub mod occs;
pub mod past;
pub mod persistent;
pub mod prof_blog;
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
