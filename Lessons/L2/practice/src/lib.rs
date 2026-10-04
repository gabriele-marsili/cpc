//! L2 (21/09) — Sliding Window Maxima: soluzione banale, heap con lazy delete, BST.
//! In tutte le funzioni SWM: restituisci il massimo di ogni finestra di k elementi
//! (n - k + 1 valori); se k == 0 o k > n restituisci un vettore vuoto.
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l2   (un esercizio: cargo test -p practice_l2 e07)

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;


/// E04 — SWM banale. Obiettivo: Θ(n·k).
/// Rust: `slice::windows(k)`.
pub fn swm_brute(a: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

/// E05 — SWM con max-heap e *lazy delete*: inserisci coppie (valore, posizione) e,
/// prima di leggere il massimo, estrai finché il massimo è fuori dalla finestra.
/// Obiettivo: Θ(n log n). Perché non Θ(n²)? (ogni elemento esce al più una volta)
/// Rust: `std::collections::BinaryHeap` (è già un max-heap), `peek`, `pop`, `while let`.
pub fn swm_heap(a: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

/// E06 — SWM con un BST usato come *multiset*: inserisci il nuovo elemento, togli quello
/// che esce, chiedi il massimo. Obiettivo: Θ(n log k).
/// Rust: `BTreeMap<i32, usize>` (valore -> quante volte compare nella finestra):
/// `entry(..).or_insert(0)`, `last_key_value()`, `remove` quando il contatore arriva a 0.
/// (Nella nota del prof si usa invece `BTreeSet<(valore, posizione)>`: prova anche quella.)
pub fn swm_btree(a: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    fn reference(a: &[i32], k: usize) -> Vec<i32> {
        if k == 0 || k > a.len() {
            return vec![];
        }
        (0..=a.len() - k).map(|i| *a[i..i + k].iter().max().unwrap()).collect()
    }

    const A: [i32; 9] = [1, 2, 3, 1, 4, 5, 2, 3, 1];

    fn check(f: fn(&[i32], usize) -> Vec<i32>) {
        assert_eq!(f(&A, 3), vec![3, 3, 4, 5, 5, 5, 3]);
        assert_eq!(f(&A, 1), A.to_vec());
        assert!(f(&A, 10).is_empty());
        let mut rng = Rng(3);
        for _ in 0..500 {
            let n = 1 + rng.below(40);
            let a = rng.vec_i32(n, -5, 6);
            let k = 1 + rng.below(n);
            assert_eq!(f(&a, k), reference(&a, k), "a={a:?} k={k}");
        }
    }

    #[test]
    fn e04_swm_brute() {
        check(swm_brute);
    }

    #[test]
    fn e05_swm_heap() {
        check(swm_heap);
    }

    #[test]
    fn e06_swm_btree() {
        check(swm_btree);
    }
}
