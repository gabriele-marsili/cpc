//! L14 (Monday 02/11/2026) — from the recording (`Lessons/L14/L_14.MP4`, 90 min, transcript
//! `L14_transcript.txt`), Octech's notes (`Lessons/L14/notes.md`) and the prof's 2025 notes
//! `Lessons/L13/Segment_tree_2025.pdf` (p.3-9). Notes in L14.pdf.
//!
//! Topics (segment tree, continued): cost of RangeSum (O(log n) nodes: at most one "splitting" partial
//! overlap, then two paths of easy cases), dynamic RMQ (argmin), RMQ with number of occurrences,
//! Search(s) = smallest i with prefix sum >= s (non-negative values; Θ(log n) vs binary search for the
//! answer Θ(log² n)), and only the STATEMENT of #occs(i, j, x) (solution: next lecture).
//!
//! [extra] All the code is mine (the prof has no blog post on segment trees): the trees follow the
//! board's "golden rule" — every node stores the answer of the query for its range — with the three
//! cases of L13 (no / total / partial overlap) for queries. `occs` implements the 2025 notes' solution
//! of #occs, NOT yet given in class (2026).
//! `prof_blog`: the prof's code from his 2026 notes (blog "Segment Tree", 14/11/2026), copied verbatim, with tests.
//! Layout of every tree (mine): node 1 = root, children 2k and 2k+1, array of 4n nodes, positions 0-based.

pub mod occs;
pub mod prof_blog;
pub mod range_sum;
pub mod rmq;
pub mod rmq_occ;
pub mod search;

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
