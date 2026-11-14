//! L14 — segment tree: RMQ dinamico, minimo con occorrenze, Search(s) (E56-E58).
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l14
//! Posizioni 0-based come alla lavagna. Regola d'oro della lezione: ogni nodo memorizza la RISPOSTA
//! della query per il suo range; la query è sempre la stessa visita a tre casi (no / total / partial overlap).
//! Scrivi tu i tuoi alberi (puoi partire dal tuo E55): niente crate esterni.

#![allow(unused_variables, unused_mut, dead_code)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E56 — Dynamic Range Minimum Queries. `add(i, v)`: A[i] += v; `rmq(l, r)`: la POSIZIONE del minimo di
/// A[l..=r] (argmin); a parità la posizione più a sinistra. Entrambe Θ(log n).
/// Rust: nel nodo tieni solo la posizione (`Vec<usize>`) e leggi il valore in una copia di A,
/// come suggerito a lezione; un metodo privato `better(p, q)` che sceglie tra due posizioni.
pub struct MinTree {
    // i tuoi campi
}

impl MinTree {
    pub fn new(a: &[i64]) -> Self {
        todo!()
    }
    pub fn add(&mut self, i: usize, v: i64) {
        todo!()
    }
    pub fn rmq(&self, l: usize, r: usize) -> usize {
        todo!()
    }
}

/// E57 — RMQ con occorrenze. `query(l, r)` = (minimo di A[l..=r], numero di occorrenze del minimo in A[l..=r]).
/// Θ(log n) per `add` e `query`. Il cuore è la funzione che combina le risposte di due range adiacenti.
/// Rust: `Vec<(i64, usize)>`, una `fn combine(x, y) -> (i64, usize)` libera, `Option` per il caso "no overlap".
pub struct MinCount {
    // i tuoi campi
}

impl MinCount {
    pub fn new(a: &[i64]) -> Self {
        todo!()
    }
    pub fn add(&mut self, i: usize, v: i64) {
        todo!()
    }
    pub fn query(&self, l: usize, r: usize) -> (i64, usize) {
        todo!()
    }
}

/// E58 — Search(s) su valori NON negativi: la più piccola posizione i con A[0] + ... + A[i] >= s
/// (`None` se la somma totale è < s). `add(i, v)` aggiunge v >= 0.
/// - `search`: discesa dalla radice in Θ(log n) (la soluzione specializzata della lezione);
/// - `search_bs`: binary search for the answer con una range sum Θ(log n) a ogni passo, Θ(log² n)
///   (la soluzione "dalla checklist": la scrivi per confrontarla con l'altra).
/// Rust: `u64`, aritmetica senza underflow (sottrai solo quando sai che non va sotto zero).
pub struct PrefixSearch {
    // i tuoi campi
}

impl PrefixSearch {
    pub fn new(a: &[u64]) -> Self {
        todo!()
    }
    pub fn add(&mut self, i: usize, v: u64) {
        todo!()
    }
    pub fn search(&self, s: u64) -> Option<usize> {
        todo!()
    }
    pub fn search_bs(&self, s: u64) -> Option<usize> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    #[test]
    fn e56_board_example() {
        // lavagna 52:00-60:00
        let mut t = MinTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        assert_eq!(t.rmq(2, 5), 2);
        assert_eq!(t.rmq(0, 7), 2);
        assert_eq!(t.rmq(4, 7), 7);
        t.add(2, 7); // -2 -> 5
        assert_eq!(t.rmq(2, 5), 3);
        assert_eq!(t.rmq(0, 7), 7);
        assert_eq!(t.rmq(0, 3), 1);
        assert_eq!(MinTree::new(&[5]).rmq(0, 0), 0);
        // parità: la posizione più a sinistra
        assert_eq!(MinTree::new(&[2, 1, 1, 1]).rmq(0, 3), 1);
    }

    #[test]
    fn e56_against_naive() {
        let mut rng = Rng(56);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<i64> = rng.vec_i32(n, -5, 6).into_iter().map(i64::from).collect();
            let mut t = MinTree::new(&a);
            for _ in 0..80 {
                let l = rng.below(n);
                let r = l + rng.below(n - l);
                if rng.below(2) == 0 {
                    let v = rng.below(11) as i64 - 5;
                    t.add(l, v);
                    a[l] += v;
                } else {
                    let best = (l..=r).fold(l, |b, i| if a[i] < a[b] { i } else { b });
                    assert_eq!(t.rmq(l, r), best, "a={a:?} l={l} r={r}");
                }
            }
        }
    }

    #[test]
    fn e57_board_example() {
        // lavagna 62:00-68:00
        let mut t = MinCount::new(&[3, 0, 0, 4, 0, 13, 2, 0]);
        assert_eq!(t.query(0, 7), (0, 4));
        assert_eq!(t.query(2, 5), (0, 2));
        t.add(4, 13); // 0 -> 13
        assert_eq!(t.query(4, 5), (13, 2));
        assert_eq!(t.query(2, 5), (0, 1));
        assert_eq!(t.query(0, 7), (0, 3));
    }

    #[test]
    fn e57_against_naive() {
        let mut rng = Rng(57);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<i64> = rng.vec_i32(n, 0, 4).into_iter().map(i64::from).collect();
            let mut t = MinCount::new(&a);
            for _ in 0..80 {
                let l = rng.below(n);
                let r = l + rng.below(n - l);
                if rng.below(2) == 0 {
                    let v = rng.below(5) as i64 - 2;
                    t.add(l, v);
                    a[l] += v;
                } else {
                    let m = *a[l..=r].iter().min().unwrap();
                    let c = a[l..=r].iter().filter(|&&x| x == m).count();
                    assert_eq!(t.query(l, r), (m, c));
                }
            }
        }
    }

    fn naive_search(a: &[u64], s: u64) -> Option<usize> {
        let mut p = 0;
        for (i, &x) in a.iter().enumerate() {
            p += x;
            if p >= s {
                return Some(i);
            }
        }
        None
    }

    #[test]
    fn e58_board_example() {
        // lavagna 74:40 e 86:00
        let t = PrefixSearch::new(&[3, 1, 0, 4, 6, 13, 2, 0]);
        assert_eq!(t.search(22), Some(5));
        assert_eq!(t.search_bs(22), Some(5));
        assert_eq!(t.search(8), Some(3));
        assert_eq!(t.search(4), Some(1)); // il primo prefisso che arriva a 4
        assert_eq!(t.search(29), Some(6));
        assert_eq!(t.search(30), None);
        assert_eq!(t.search_bs(30), None);
        assert_eq!(t.search(0), Some(0));
    }

    #[test]
    fn e58_against_naive() {
        let mut rng = Rng(58);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<u64> = (0..n).map(|_| rng.below(5) as u64).collect();
            let mut t = PrefixSearch::new(&a);
            for _ in 0..80 {
                if rng.below(3) == 0 {
                    let i = rng.below(n);
                    let v = rng.below(4) as u64;
                    t.add(i, v);
                    a[i] += v;
                } else {
                    let s = rng.below(5 * n + 2) as u64;
                    let want = naive_search(&a, s);
                    assert_eq!(t.search(s), want, "a={a:?} s={s}");
                    assert_eq!(t.search_bs(s), want, "a={a:?} s={s}");
                }
            }
        }
    }
}
