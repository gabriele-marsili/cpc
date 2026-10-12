//! L6 — PROVVISORIO (lezione del 05/10 non ancora integrata): argomenti annunciati alla fine di L5,
//! dalle note 2025 del prof (Pearls_2025.pdf p.14-17): maggioranza con cancellazioni, Misra-Gries;
//! binary search (Binary_search_2025.pdf p.2-3, blog del prof): E24-E27.
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l6   (un esercizio: cargo test -p practice_l6 e22)

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E22 — Maggioranza con inserimenti e cancellazioni (note p.14): per ogni bit conta quanti
/// elementi hanno quel bit a 1; il candidato ha in ogni bit il valore più frequente.
/// O(log u) per operazione. Il candidato è significativo solo se una maggioranza esiste.
pub struct BitMajority {
    // scegli tu i campi
}

impl BitMajority {
    /// `bits` = numero di bit dei valori (u < 2^bits)
    pub fn new(bits: usize) -> Self {
        todo!()
    }
    pub fn insert(&mut self, x: u64) {
        todo!()
    }
    pub fn delete(&mut self, x: u64) {
        todo!()
    }
    pub fn candidate(&self) -> u64 {
        todo!()
    }
}

/// E23 — Heavy hitters: tutti gli elementi che compaiono almeno n/t + 1 volte (divisione intera),
/// in ordine crescente. Misra-Gries: al più t candidati con contatore; se diventano t+1,
/// decrementa tutti e togli gli zeri. Poi verifica i candidati con una seconda passata.
/// Obiettivo: Θ(n) (oltre al costo della mappa). Perché non O(t·n)?
/// Rust: `HashMap::retain`.
pub fn heavy_hitters(a: &[i64], t: usize) -> Vec<i64> {
    todo!()
}

// ------------------------- Binary search (note 2025 + blog del prof) -------------------------

/// E24 — Prima occorrenza con la ricerca binaria: posizione della prima occorrenza di `key` in `a`
/// ordinato, oppure None. Obiettivo: Θ(log n). Attenzione: `(low + high) / 2` può andare in overflow.
/// Rust: `match key.cmp(&a[mid])` con `std::cmp::Ordering`.
pub fn first_occurrence(a: &[i32], key: i32) -> Option<usize> {
    todo!()
}

/// E25 — Binary search for the answer: `pred` è monotono su [low, high) (prima tutti true, poi
/// tutti false). Restituisci l'ultimo valore per cui pred è true (None se nessuno).
/// Poi usala per la radice quadrata intera (E25b): il più grande x con x*x <= v.
/// Obiettivo: Θ(log(high - low)) valutazioni di pred.
/// Rust: parametro generico `F: Fn(u64) -> bool`; attenzione all'overflow di x*x (`checked_mul`).
pub fn last_true<F: Fn(u64) -> bool>(low: u64, high: u64, pred: F) -> Option<u64> {
    todo!()
}

/// E25b — Radice quadrata intera usando `last_true`.
pub fn isqrt(v: u64) -> u64 {
    todo!()
}

/// E26 — Social distancing: intervalli disgiunti [a, b] di interi (in ordine qualsiasi), c >= 2
/// persone da piazzare su punti interi degli intervalli: restituisci la massima distanza minima
/// possibile, None se i punti disponibili sono meno di c.
/// Esempio: [(0,2), (4,7), (9,9), (12,15)], c = 5 -> 3  (es. 0, 4, 7, 12, 15)
/// Obiettivo: Θ((n + c) log ℓ) + ordinamento. Idea: "riesco a piazzare c persone a distanza >= d?"
/// è monotono in d e si verifica greedy da sinistra.
/// Attenzione al limite superiore della ricerca: non è il numero di punti, è l'ampiezza.
pub fn social_distancing(intervals: &[(usize, usize)], c: usize) -> Option<usize> {
    todo!()
}

/// E27 — Exponential search: la risposta t non ha un limite superiore noto. `pred` è monotono
/// (true per x <= t, false dopo) e pred(0) è true: trova t. Fase 1: raddoppia finché pred è
/// vero; fase 2: ricerca binaria nell'ultimo intervallo [2^i, 2^(i+1)).
/// Obiettivo: Θ(log t) valutazioni di pred (non Θ(log u) con u = u64::MAX).
pub fn exponential_search<F: Fn(u64) -> bool>(pred: F) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    #[test]
    fn e22_bit_majority() {
        let mut m = BitMajority::new(3);
        for x in [5, 3, 1, 1, 2, 3, 1, 1, 1] {
            m.insert(x);
        }
        assert_eq!(m.candidate(), 1);
        for _ in 0..5 {
            m.delete(1);
        }
        m.insert(3); // rimane 5 3 2 3 3
        assert_eq!(m.candidate(), 3);
    }

    #[test]
    fn e23_heavy_hitters() {
        assert_eq!(heavy_hitters(&[5, 3, 1, 1, 2, 3, 1, 1, 1], 2), vec![1]);
        assert_eq!(heavy_hitters(&[1, 1, 1, 2, 2, 2, 3, 4], 3), vec![1, 2]); // soglia 8/3 + 1 = 3
        let mut rng = Rng(22);
        for _ in 0..2000 {
            let n = 1 + rng.below(40);
            let t = 1 + rng.below(4);
            let a: Vec<i64> = (0..n).map(|_| rng.below(6) as i64).collect();
            let brute: Vec<i64> =
                (0..6).filter(|&v| a.iter().filter(|&&x| x == v).count() >= n / t + 1).collect();
            assert_eq!(heavy_hitters(&a, t), brute, "{a:?} t={t}");
        }
    }

    #[test]
    fn e24_first_occurrence() {
        let a = [1, 3, 3, 3, 5, 8, 8, 13];
        assert_eq!(first_occurrence(&a, 3), Some(1));
        assert_eq!(first_occurrence(&a, 13), Some(7));
        assert_eq!(first_occurrence(&a, 4), None);
        assert_eq!(first_occurrence(&[], 4), None);
        let mut rng = Rng(24);
        for _ in 0..1000 {
            let n = rng.below(30);
            let mut v = rng.vec_i32(n, 0, 10);
            v.sort();
            let k = rng.below(11) as i32;
            assert_eq!(first_occurrence(&v, k), v.iter().position(|&x| x == k), "{v:?} {k}");
        }
    }

    #[test]
    fn e25_last_true_and_isqrt() {
        assert_eq!(last_true(0, 100, |x| x < 37), Some(36));
        assert_eq!(last_true(5, 10, |_| false), None);
        assert_eq!(last_true(5, 10, |_| true), Some(9));
        for v in 0..3000u64 {
            let r = isqrt(v);
            assert!(r * r <= v && (r + 1) * (r + 1) > v, "v={v}");
        }
        assert_eq!(isqrt(u64::MAX), 4294967295);
    }

    fn social_brute(iv: &[(usize, usize)], c: usize) -> Option<usize> {
        let mut pts: Vec<usize> = iv.iter().flat_map(|&(a, b)| a..=b).collect();
        pts.sort();
        if pts.len() < c {
            return None;
        }
        (1..=pts[pts.len() - 1] - pts[0] + 1)
            .filter(|&d| {
                let (mut last, mut cnt) = (pts[0], 1);
                for &p in &pts[1..] {
                    if p >= last + d {
                        last = p;
                        cnt += 1;
                    }
                }
                cnt >= c
            })
            .max()
    }

    #[test]
    fn e26_social_distancing() {
        assert_eq!(social_distancing(&[(0, 2), (4, 7), (9, 9), (12, 15)], 5), Some(3));
        assert_eq!(social_distancing(&[(1, 1), (4, 5), (9, 12), (14, 16)], 2), Some(15));
        assert_eq!(social_distancing(&[(0, 1)], 3), None);
        let mut rng = Rng(26);
        for _ in 0..1000 {
            let mut iv = Vec::new();
            let mut x = rng.below(3);
            for _ in 0..1 + rng.below(5) {
                let len = rng.below(4);
                iv.push((x, x + len));
                x += len + 1 + rng.below(4);
            }
            let p = rng.permutation(iv.len());
            let shuffled: Vec<(usize, usize)> = p.iter().map(|&i| iv[i]).collect();
            let c = 2 + rng.below(7);
            assert_eq!(social_distancing(&shuffled, c), social_brute(&iv, c), "{iv:?} c={c}");
        }
    }

    #[test]
    fn e27_exponential_search() {
        use std::cell::Cell;
        for t in [0u64, 1, 2, 3, 7, 8, 1000, 123_456_789] {
            let calls = Cell::new(0u32);
            let r = exponential_search(|x| {
                calls.set(calls.get() + 1);
                x <= t
            });
            assert_eq!(r, t);
            let bound = 2 * (64 - (t + 1).leading_zeros()) + 2;
            assert!(calls.get() <= bound, "t={t}: {} chiamate", calls.get());
        }
    }
}
