// L4 (28/09/2026) — Duplicate element in an array (prima "pearl")
//
// File generato da Claude: NON è codice del prof (a lezione non ha scritto codice).
// Le soluzioni seguono quelle descritte a lezione (Floyd e il resto della L5 sono in Lessons/L5).
// La tua soluzione `my_sol` (lib.rs) non è toccata: qui ci sono solo test in più.
//
// Problema: A[0..=n] contiene n+1 valori in {0, ..., n-1}  =>  c'è almeno un duplicato.
// Trovarne uno qualsiasi.

use std::collections::HashSet;

/// Soluzione #1: hash set. Θ(n) tempo atteso (whp), Θ(n) spazio extra.
pub fn hash_set(a: &[usize]) -> Option<usize> {
    let mut h = HashSet::with_capacity(a.len());
    a.iter().copied().find(|&x| !h.insert(x))
}

/// Soluzione #2: tabella ad accesso diretto / bit vector. Θ(n) deterministico, più veloce
/// dell'hash set, ma ancora Θ(n) bit di spazio extra.
pub fn direct_access(a: &[usize]) -> Option<usize> {
    let mut seen = vec![false; a.len()];
    a.iter().copied().find(|&x| std::mem::replace(&mut seen[x], true))
}

/// Idea "ingenua" di lezione: per ogni bit, prende il valore più frequente su TUTTO l'array.
/// SBAGLIATA: controesempio in aula [3,3,5,6,5,2,0,4,7] -> restituisce 7 (non è un duplicato).
/// Assume n potenza di 2. Tenuta solo per i test.
pub fn bits_majority_naive(a: &[usize]) -> usize {
    let n = a.len() - 1;
    let bits = n.trailing_zeros();
    let mut e = 0;
    for b in (0..bits).rev() {
        let ones = a.iter().filter(|&&x| (x >> b) & 1 == 1).count();
        if ones > a.len() - ones {
            e |= 1 << b;
        }
    }
    e
}

/// Soluzione #3 (lezione): Θ(log n) passate, si ricostruisce e bit per bit dal più
/// significativo, contando solo gli elementi che iniziano col prefisso già trovato.
/// Θ(n log n) tempo, O(1) spazio extra, A in sola lettura e letto solo in sequenza.
///
/// Versione per n qualsiasi (a lezione n era una potenza di 2) [extra]: invece dei bit si
/// dimezza l'intervallo di valori [lo, hi) che contiene di sicuro un duplicato: se in
/// [lo, mid) cadono più di (mid - lo) elementi, lì c'è un duplicato (piccionaia), altrimenti
/// sta in [mid, hi). Con n potenza di 2 è esattamente "contare 0 e 1 sul bit successivo".
pub fn bit_by_bit(a: &[usize]) -> usize {
    let (mut lo, mut hi) = (0usize, a.len() - 1); // valori in [lo, hi)
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        let left = a.iter().filter(|&&x| lo <= x && x < mid).count();
        if left > mid - lo {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_sol;

    struct Rng(u64);
    impl Rng {
        fn below(&mut self, n: usize) -> usize {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 % n as u64) as usize
        }
    }

    fn is_dup(a: &[usize], x: usize) -> bool {
        a.iter().filter(|&&y| y == x).count() >= 2
    }

    const LESSON: [usize; 9] = [1, 3, 5, 6, 5, 2, 0, 4, 7];

    #[test]
    fn lesson_example_all_solutions() {
        assert_eq!(hash_set(&LESSON), Some(5));
        assert_eq!(direct_access(&LESSON), Some(5));
        assert_eq!(bit_by_bit(&LESSON), 5);
        assert_eq!(bits_majority_naive(&LESSON), 5); // qui funziona per caso...
    }

    #[test]
    fn naive_bits_counterexample_from_class() {
        // "mettiamo un 3 al primo posto": duplicati {3, 5}, ma l'idea ingenua dà 111 = 7
        let a = [3, 3, 5, 6, 5, 2, 0, 4, 7];
        assert_eq!(bits_majority_naive(&a), 7);
        assert!(!is_dup(&a, 7));
        assert!(is_dup(&a, bit_by_bit(&a))); // restringendosi al prefisso funziona (dà 5)
    }

    #[test]
    fn all_solutions_on_random_arrays() {
        let mut rng = Rng(0x9E3779B97F4A7C15);
        for _ in 0..3000 {
            let n = 1 + rng.below(40);
            let a: Vec<usize> = (0..=n).map(|_| rng.below(n)).collect();
            assert!(is_dup(&a, hash_set(&a).unwrap()), "{a:?}");
            assert!(is_dup(&a, direct_access(&a).unwrap()), "{a:?}");
            assert!(is_dup(&a, bit_by_bit(&a)), "{a:?}");
        }
    }

    /// Controllo della TUA soluzione (my_sol) contro la definizione, su input casuali
    #[test]
    fn my_sol_on_random_arrays() {
        let mut rng = Rng(12345);
        for _ in 0..3000 {
            let n = 1 + rng.below(40);
            let orig: Vec<usize> = (0..=n).map(|_| rng.below(n)).collect();
            let d = my_sol(&mut orig.clone()).expect("un duplicato esiste sempre");
            assert!(is_dup(&orig, d), "{orig:?} -> {d}");
        }
    }

    /// my_sol fa al più n scambi in tutto: ognuno mette definitivamente un valore al suo posto
    #[test]
    fn my_sol_is_linear_on_adversarial_input() {
        let n = 200_000;
        let mut a: Vec<usize> = (1..n).chain([0, n - 1]).collect(); // un solo grande ciclo + duplicato
        assert_eq!(a.len(), n + 1);
        assert_eq!(my_sol(&mut a), Some(n - 1));
    }
}
