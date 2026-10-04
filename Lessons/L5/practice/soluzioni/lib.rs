//! SOLUZIONE (scritta da Claude, non dal prof). L5 (30/09) — PROVVISORIA: ricostruita dalle note 2025 del prof (Pearls_2025.pdf p.8-17),
//! la registrazione non è ancora disponibile. Floyd, maggioranza, Boyer-Moore, Misra-Gries.
//!
//! SOLUZIONI: copia questo file su src/lib.rs per provarle (dopo aver salvato il tuo lavoro).

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;


/// E19 — Duplicato con `a` in SOLA LETTURA, Θ(n) tempo, O(1) spazio extra: algoritmo di Floyd.
/// Vedi `a` come lista i -> a[i], partendo da n (nessuno punta a n): si entra in un ciclo e
/// l'ingresso del ciclo è un duplicato. Fase 1: lento (1 passo) e veloce (2 passi) si incontrano
/// nel ciclo. Fase 2: uno riparte dall'inizio, entrambi a 1 passo: si incontrano all'ingresso.
/// Stesso contratto dei problemi sul duplicato di L4.
pub fn dup_floyd(a: &[usize]) -> usize {
    let n = a.len() - 1;
    let (mut slow, mut fast) = (a[n], a[a[n]]);
    while slow != fast {
        slow = a[slow];
        fast = a[a[fast]];
    }
    let mut s = n;
    while s != slow {
        s = a[s];
        slow = a[slow];
    }
    s
}

/// E20 — Elemento di maggioranza (compare più di n/2 volte), se esiste.
/// Boyer-Moore: candidato + contatore, poi una seconda passata di verifica.
/// Esempio delle note: [5, 3, 1, 1, 2, 3, 1, 1, 1] -> Some(1);  [1, 2, 3] -> None
/// Obiettivo: Θ(n) tempo, O(1) spazio.
pub fn majority(a: &[i64]) -> Option<i64> {
    let (&first, rest) = a.split_first()?;
    let (mut c, mut count) = (first, 1usize);
    for &x in rest {
        if x == c {
            count += 1;
        } else {
            count -= 1;
            if count == 0 {
                c = x;
                count = 1;
            }
        }
    }
    (a.iter().filter(|&&x| x == c).count() > a.len() / 2).then_some(c)
}

/// E21 — Maggioranza con inserimenti e cancellazioni (note p.14): per ogni bit conta quanti
/// elementi hanno quel bit a 1; il candidato ha in ogni bit il valore più frequente.
/// O(log u) per operazione. Il candidato è significativo solo se una maggioranza esiste.
pub struct BitMajority {
    ones: Vec<usize>,
    len: usize,
}

impl BitMajority {
    /// `bits` = numero di bit dei valori (u < 2^bits)
    pub fn new(bits: usize) -> Self {
        BitMajority { ones: vec![0; bits], len: 0 }
    }
    pub fn insert(&mut self, x: u64) {
        self.len += 1;
        for (b, c) in self.ones.iter_mut().enumerate() {
            *c += ((x >> b) & 1) as usize;
        }
    }
    pub fn delete(&mut self, x: u64) {
        self.len -= 1;
        for (b, c) in self.ones.iter_mut().enumerate() {
            *c -= ((x >> b) & 1) as usize;
        }
    }
    pub fn candidate(&self) -> u64 {
        self.ones.iter().enumerate()
            .filter(|&(_, &c)| c > self.len - c)
            .map(|(b, _)| 1u64 << b)
            .sum()
    }
}

/// E22 — Heavy hitters: tutti gli elementi che compaiono almeno n/t + 1 volte (divisione intera),
/// in ordine crescente. Misra-Gries: al più t candidati con contatore; se diventano t+1,
/// decrementa tutti e togli gli zeri. Poi verifica i candidati con una seconda passata.
/// Obiettivo: Θ(n) (oltre al costo della mappa). Perché non O(t·n)?
/// Rust: `HashMap::retain`.
pub fn heavy_hitters(a: &[i64], t: usize) -> Vec<i64> {
    use std::collections::HashMap;
    let mut k: HashMap<i64, usize> = HashMap::new();
    for &e in a {
        *k.entry(e).or_insert(0) += 1;
        if k.len() > t {
            k.retain(|_, c| { *c -= 1; *c > 0 });
        }
    }
    let threshold = a.len() / t + 1;
    let mut out: Vec<i64> = k
        .into_keys()
        .filter(|c| a.iter().filter(|&&x| x == *c).count() >= threshold)
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    #[test]
    fn e19_dup_floyd() {
        assert_eq!(dup_floyd(&[1, 3, 5, 6, 5, 2, 0, 4, 7]), 5);
        let mut rng = Rng(19);
        for _ in 0..2000 {
            let n = 1 + rng.below(40);
            let a: Vec<usize> = (0..=n).map(|_| rng.below(n)).collect();
            let d = dup_floyd(&a);
            assert!(a.iter().filter(|&&x| x == d).count() >= 2, "{a:?}");
        }
    }

    #[test]
    fn e20_majority() {
        assert_eq!(majority(&[5, 3, 1, 1, 2, 3, 1, 1, 1]), Some(1));
        assert_eq!(majority(&[1, 2, 3]), None);
        assert_eq!(majority(&[]), None);
        assert_eq!(majority(&[1, 1, 2, 2]), None); // esattamente n/2 non basta
        let mut rng = Rng(20);
        for _ in 0..2000 {
            let n = rng.below(30);
            let a: Vec<i64> = (0..n).map(|_| rng.below(3) as i64).collect();
            let brute = (0..3).find(|&v| a.iter().filter(|&&x| x == v).count() > n / 2);
            assert_eq!(majority(&a), brute, "{a:?}");
        }
    }

    #[test]
    fn e21_bit_majority() {
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
    fn e22_heavy_hitters() {
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
}
