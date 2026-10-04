//! SOLUZIONE (scritta da Claude, non dal prof). L4 (28/09) — 100 prigionieri (dimostrazione), elemento duplicato.
//! Nei problemi sul duplicato: `a` ha n+1 elementi con valori in {0, ..., n-1}
//! (quindi c'è almeno un duplicato); restituisci un valore qualsiasi che compare almeno 2 volte.
//!
//! SOLUZIONI: copia questo file su src/lib.rs per provarle (dopo aver salvato il tuo lavoro).

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;


/// E12 — Strategia "segui il ciclo". `drawers[d]` è il numero dentro il cassetto d
/// (una permutazione di 0..n, n pari). Il prigioniero p apre il cassetto p, poi quello col
/// numero trovato, ecc., al massimo n/2 cassetti. Restituisci true se TUTTI trovano il proprio numero.
/// Esempio delle note (in base 0): [6, 3, 5, 7, 1, 2, 4, 0] -> false (c'è un ciclo di lunghezza 6 > 4)
pub fn all_prisoners_win(drawers: &[usize]) -> bool {
    let n = drawers.len();
    (0..n).all(|p| {
        let mut d = p;
        for _ in 0..n / 2 {
            if drawers[d] == p {
                return true;
            }
            d = drawers[d];
        }
        false
    })
}

/// E13 — Lunghezza del ciclo più lungo di una permutazione. Θ(n) tempo.
/// (Verifica di E12: tutti vincono <=> il ciclo più lungo è <= n/2.)
/// Rust: un `Vec<bool>` di visitati.
pub fn longest_cycle(perm: &[usize]) -> usize {
    let mut seen = vec![false; perm.len()];
    let mut best = 0;
    for s in 0..perm.len() {
        let mut len = 0;
        let mut d = s;
        while !seen[d] {
            seen[d] = true;
            d = perm[d];
            len += 1;
        }
        best = best.max(len);
    }
    best
}

/// E14 — Probabilità esatta di vittoria con n prigionieri (n pari): 1 - (H_n - H_{n/2}).
/// Per n = 100 vale circa 0.3118. Poi confrontala con una simulazione (vedi il test).
pub fn win_probability(n: usize) -> f64 {
    1.0 - ((n / 2 + 1)..=n).map(|l| 1.0 / l as f64).sum::<f64>()
}

/// E15 — Duplicato con un hash set. Θ(n) atteso, Θ(n) spazio.
/// Rust: `HashSet::insert` restituisce false se l'elemento c'era già.
pub fn dup_hash_set(a: &[usize]) -> usize {
    let mut seen = std::collections::HashSet::new();
    *a.iter().find(|&&x| !seen.insert(x)).unwrap()
}

/// E16 — Duplicato con una tabella ad accesso diretto (vettore di bool / bit).
/// Θ(n) deterministico, Θ(n) spazio. Perché è meglio dell'hash set?
pub fn dup_direct_access(a: &[usize]) -> usize {
    let mut seen = vec![false; a.len()];
    *a.iter().find(|&&x| std::mem::replace(&mut seen[x], true)).unwrap()
}

/// E17 — Duplicato con O(1) spazio extra, `a` in sola lettura, solo scansioni sequenziali:
/// ricostruisci il duplicato bit per bit (o, più in generale, dimezzando l'intervallo di
/// valori che di sicuro contiene un duplicato). Θ(n log n) tempo.
/// Attenzione: contare 0 e 1 su TUTTO l'array per ogni bit è sbagliato
/// (controesempio di lezione: [3, 3, 5, 6, 5, 2, 0, 4, 7]).
/// Funziona per n qualsiasi, non solo potenze di 2.
pub fn dup_bit_by_bit(a: &[usize]) -> usize {
    // valori in [lo, hi): se in [lo, mid) cadono più di mid-lo elementi, lì c'è un duplicato
    let (mut lo, mut hi) = (0, a.len() - 1);
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        let left = a.iter().filter(|&&x| lo <= x && x < mid).count();
        if left > mid - lo { hi = mid; } else { lo = mid; }
    }
    lo
}

/// E18 — Duplicato in Θ(n) tempo e O(1) spazio extra, potendo MODIFICARE `a`.
/// Compito di L4. Qualsiasi idea va bene (scambi per mettere ogni valore v nella cella v,
/// marcare le celle visitate seguendo i -> a[i] partendo da n, ...).
pub fn dup_destroy(a: &mut [usize]) -> usize {
    // segui i -> a[i] partendo da n, marcando con n le celle visitate
    let n = a.len() - 1;
    let mut cur = n;
    loop {
        let nxt = a[cur];
        a[cur] = n;
        if a[nxt] == n {
            return nxt;
        }
        cur = nxt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    fn is_dup(a: &[usize], x: usize) -> bool {
        a.iter().filter(|&&y| y == x).count() >= 2
    }

    fn random_instances(seed: u64, count: usize) -> Vec<Vec<usize>> {
        let mut rng = Rng(seed);
        (0..count)
            .map(|_| {
                let n = 1 + rng.below(40);
                (0..=n).map(|_| rng.below(n)).collect()
            })
            .collect()
    }

    const LESSON: [usize; 9] = [1, 3, 5, 6, 5, 2, 0, 4, 7];

    #[test]
    fn e12_all_prisoners_win() {
        assert!(!all_prisoners_win(&[6, 3, 5, 7, 1, 2, 4, 0]));
        assert!(all_prisoners_win(&[1, 0, 3, 2])); // due cicli di lunghezza 2
        assert!(!all_prisoners_win(&[1, 2, 3, 0])); // un ciclo di lunghezza 4 > 2
        assert!(all_prisoners_win(&[0, 1, 2, 3]));
    }

    #[test]
    fn e13_longest_cycle() {
        assert_eq!(longest_cycle(&[6, 3, 5, 7, 1, 2, 4, 0]), 6);
        assert_eq!(longest_cycle(&[0, 1, 2]), 1);
        let mut rng = Rng(13);
        for _ in 0..500 {
            let n = 2 * (1 + rng.below(20));
            let p = rng.permutation(n);
            assert_eq!(all_prisoners_win(&p), longest_cycle(&p) <= p.len() / 2, "{p:?}");
        }
    }

    #[test]
    fn e14_win_probability() {
        assert!((win_probability(100) - 0.3118).abs() < 1e-4);
        assert!((win_probability(2) - 0.5).abs() < 1e-12);
        // simulazione: deve avvicinarsi al valore esatto
        let mut rng = Rng(14);
        let trials = 20_000;
        let wins = (0..trials).filter(|_| all_prisoners_win(&rng.permutation(100))).count();
        assert!((wins as f64 / trials as f64 - win_probability(100)).abs() < 0.02);
    }

    #[test]
    fn e15_dup_hash_set() {
        assert_eq!(dup_hash_set(&LESSON), 5);
        for a in random_instances(15, 1000) {
            assert!(is_dup(&a, dup_hash_set(&a)), "{a:?}");
        }
    }

    #[test]
    fn e16_dup_direct_access() {
        assert_eq!(dup_direct_access(&LESSON), 5);
        for a in random_instances(16, 1000) {
            assert!(is_dup(&a, dup_direct_access(&a)), "{a:?}");
        }
    }

    #[test]
    fn e17_dup_bit_by_bit() {
        assert_eq!(dup_bit_by_bit(&LESSON), 5);
        assert!(is_dup(&[3, 3, 5, 6, 5, 2, 0, 4, 7], dup_bit_by_bit(&[3, 3, 5, 6, 5, 2, 0, 4, 7])));
        for a in random_instances(17, 2000) {
            assert!(is_dup(&a, dup_bit_by_bit(&a)), "{a:?}");
        }
    }

    #[test]
    fn e18_dup_destroy() {
        assert_eq!(dup_destroy(&mut LESSON.to_vec()), 5);
        for a in random_instances(18, 2000) {
            assert!(is_dup(&a, dup_destroy(&mut a.clone())), "{a:?}");
        }
        // input grande: deve essere lineare
        let n = 300_000;
        let mut big: Vec<usize> = (1..n).chain([0, n - 1]).collect();
        assert_eq!(dup_destroy(&mut big), n - 1);
    }
}
