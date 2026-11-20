//! L16 (Wednesday 18/11/2026) — static Range Minimum Query (RMQ). Notes in L16.pdf.
//! From the recording L_16.MP4 (58 min) and the prof's board.
//!
//! The four solutions of the lecture (target: O(1) per query, Θ(n) space, A static):
//! - `full_table`  Solution #1: tabulate all the answers, O(1) query, Θ(n²) space, built row-wise in Θ(n²)
//! - `sparse`      Solution #2: sparse table, O(1) query, Θ(n log n) space — the PROF's code (prof/sparse_table.py)
//!                 translated to Rust line by line
//! - `blocks`      Solution #3: blocks of size log n + sparse table on the block minima, Θ(log n) query, Θ(n) space
//! - `msb`         position of the most significant bit (the "k" of the query): CPU instruction vs the 256-entry table
//!                 the prof described
//! - `bp`          the digression of the lecture: a tree shape in 2n bits with balanced parentheses (and the
//!                 level-order unary degree sequence, the prof's open question)
//! - `cartesian`   [extra, preview] Solution #4 was only announced ("Cartesian tree" is the last heading on the board):
//!                 the Cartesian tree, built with a stack in Θ(n); RMQ(i, j) = LCA(i, j)
//!
//! [extra] All Rust code is mine (Claude); `sparse` follows the prof's Python exactly. 0-based, both ends included.

pub mod blocks;
pub mod bp;
pub mod cartesian;
pub mod full_table;
pub mod msb;
pub mod sparse;

/// Brute force used by every test: position of the leftmost minimum of a[i..=j].
pub fn rmq_naive<T: Ord>(a: &[T], i: usize, j: usize) -> usize {
    let mut best = i;
    for p in i + 1..=j {
        if a[p] < a[best] {
            best = p;
        }
    }
    best
}

/// The array of the lecture's example.
pub const LECTURE_A: [i64; 8] = [5, 2, 7, 3, 8, 1, 6, 4];

#[cfg(test)]
pub(crate) mod rng {
    pub struct Rng(pub u64);
    impl Rng {
        pub fn next_u64(&mut self) -> u64 {
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            self.0.wrapping_mul(0x2545F4914F6CDD1D)
        }
        pub fn below(&mut self, n: usize) -> usize {
            (self.next_u64() % n as u64) as usize
        }
        pub fn vec(&mut self, len: usize, lo: i64, hi: i64) -> Vec<i64> {
            (0..len).map(|_| lo + self.below((hi - lo) as usize) as i64).collect()
        }
    }
}
