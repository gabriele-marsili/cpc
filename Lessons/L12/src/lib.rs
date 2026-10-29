//! L12 (Monday 26/10/2026), from the recording `L_12.MP4`: counting inversions with a Fenwick tree,
//! remapping the values (sort + rank), Nested Segments (solution derived together in class).
//! Update the Array and range updates were done on 28/10: they are in the crate `l13` (module `range`).
//! Notes in L12.pdf.
//!
//! Prof's code (copied verbatim from <https://pages.di.unipi.it/rossano/blog/2023/fenwick>, only comments
//! and `pub` added): `fenwick::FenwickTree`, `inversions::counting_inversions`.
//! [extra] (not from the prof): `inversions::remap` / `counting_inversions_any`, the three Nested Segments
//! implementations in `nested` (the lecture's one is `nested_segments_2026`), brute forces and tests.

pub mod fenwick;
pub mod inversions;
pub mod nested;

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
