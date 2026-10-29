//! L13 (Wednesday 28/10/2026), from the recording `L_13.MP4`: Update the Array (range update + access),
//! dynamic prefix sums with range update (two Fenwick trees), introduction to the segment tree.
//! Notes in L13.pdf.
//!
//! Prof's code (copied verbatim from <https://pages.di.unipi.it/rossano/blog/2023/fenwick>, only comments
//! and `pub` added): `fenwick::FenwickTree`, `range::UpdateArray`, `range::RangeUpdate`.
//! [extra] (not from the prof): `range::RangeUpdateLecture` (the board's formulation, 1-based),
//! `segment_tree::SegmentTree` (no code from the prof), brute forces and tests.

pub mod fenwick;
pub mod range;
pub mod segment_tree;

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
