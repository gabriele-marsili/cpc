//! L11 (Wednesday 21/10/2026), redone on the recording `L_11.MP4`: Fenwick tree (levels, `Sum` and the
//! parent i - (i & -i), `Add` and the right sibling i + (i & -i), Θ(log n)) and counting inversions
//! (definition, insertion sort, mergesort variant). The lecture ends there; the Fenwick-tree solution of
//! counting inversions, Nested Segments and range updates are in the crate `l12`.
//! Notes in L11.pdf.
//!
//! Prof's code (copied verbatim from <https://pages.di.unipi.it/rossano/blog/2023/fenwick>, only comments
//! added): `fenwick::FenwickTree`.
//! [extra] (not from the prof): `inversions::merge_sort_inversions`, `inversions::insertion_sort_inversions`,
//! brute force and tests.

pub mod fenwick;
pub mod inversions;

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
