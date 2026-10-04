//! SOLUZIONE (scritta da Claude, non dal prof). L1 (16/09) — Introduzione. Problemi dalla pagina del corso:
//! Leaders in array, Kadane's algorithm, Missing number in array.
//!
//! SOLUZIONI: copia questo file su src/lib.rs per provarle (dopo aver salvato il tuo lavoro).

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;


/// E01 — Leaders in array.
/// Un elemento è un *leader* se è strettamente maggiore di tutti gli elementi alla sua destra
/// (l'ultimo elemento è sempre un leader). Restituisci i leader nell'ordine in cui compaiono.
/// Esempio: [16, 17, 4, 3, 5, 2] -> [17, 5, 2]
/// Obiettivo: Θ(n) tempo, una sola scansione (da destra).
/// Rust: iteratori al contrario (`.iter().rev()`), `Vec::reverse`.
pub fn leaders(a: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    let mut max_right: Option<i32> = None;
    for &x in a.iter().rev() {
        if max_right.map_or(true, |m| x > m) {
            out.push(x);
            max_right = Some(x);
        }
    }
    out.reverse();
    out
}

/// E02 — Kadane: somma massima di un sottoarray contiguo NON vuoto.
/// Esempio: [-2, 1, -3, 4, -1, 2, 1, -5, 4] -> 6  (il sottoarray [4, -1, 2, 1])
/// Obiettivo: Θ(n) tempo, O(1) spazio. Attenzione al caso con tutti numeri negativi.
/// LeetCode: https://leetcode.com/problems/maximum-subarray/
pub fn max_subarray(a: &[i64]) -> i64 {
    let mut best = a[0];
    let mut cur = a[0];
    for &x in &a[1..] {
        cur = x.max(cur + x); // o estendo il sottoarray, o ricomincio da x
        best = best.max(cur);
    }
    best
}

/// E03 — Missing number. `a` contiene n numeri DISTINTI presi da {0, 1, ..., n}:
/// ne manca esattamente uno, trovalo.
/// Esempio: [3, 0, 1] -> 2
/// Obiettivo: Θ(n) tempo, O(1) spazio (somma attesa meno somma, oppure XOR).
/// Rust: attenzione all'overflow se usi la somma (usa u64 o lo XOR).
pub fn missing_number(a: &[u32]) -> u32 {
    let n = a.len() as u32;
    // XOR di 0..=n e di tutti gli elementi: le coppie si annullano, resta il mancante
    (0..=n).fold(0, |acc, x| acc ^ x) ^ a.iter().fold(0, |acc, &x| acc ^ x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    #[test]
    fn e01_leaders() {
        assert_eq!(leaders(&[16, 17, 4, 3, 5, 2]), vec![17, 5, 2]);
        assert_eq!(leaders(&[1, 2, 3]), vec![3]);
        assert_eq!(leaders(&[3, 3]), vec![3]); // "strettamente maggiore"
        assert_eq!(leaders(&[]), Vec::<i32>::new());
    }

    #[test]
    fn e02_max_subarray() {
        assert_eq!(max_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_subarray(&[-3, -1, -2]), -1);
        assert_eq!(max_subarray(&[5]), 5);
        let mut rng = Rng(1);
        for _ in 0..500 {
            let n = 1 + rng.below(30);
            let a: Vec<i64> = (0..n).map(|_| rng.below(21) as i64 - 10).collect();
            let brute = (0..n)
                .flat_map(|i| (i..n).map(move |j| (i, j)))
                .map(|(i, j)| a[i..=j].iter().sum::<i64>())
                .max()
                .unwrap();
            assert_eq!(max_subarray(&a), brute, "{a:?}");
        }
    }

    #[test]
    fn e03_missing_number() {
        assert_eq!(missing_number(&[3, 0, 1]), 2);
        assert_eq!(missing_number(&[0]), 1);
        assert_eq!(missing_number(&[1]), 0);
        let mut rng = Rng(2);
        for _ in 0..200 {
            let n = 1 + rng.below(50);
            let mut p: Vec<u32> = rng.permutation(n + 1).into_iter().map(|x| x as u32).collect();
            let missing = p.pop().unwrap();
            assert_eq!(missing_number(&p), missing);
        }
    }
}
