#![allow(dead_code)] // non tutti i test usano tutti gli strumenti
//! Strumenti già pronti per i test (non sono esercizi).

/// Generatore pseudo-casuale minimale (xorshift64*), deterministico: stesso seme, stessi numeri.
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545F4914F6CDD1D)
    }
    /// Numero in [0, n)
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
    /// Vettore di `len` interi in [lo, hi)
    pub fn vec_i64(&mut self, len: usize, lo: i64, hi: i64) -> Vec<i64> {
        (0..len).map(|_| lo + self.below((hi - lo) as usize) as i64).collect()
    }
}
