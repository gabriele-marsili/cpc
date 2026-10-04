//! SOLUZIONE (scritta da Claude, non dal prof). L2 (21/09) — Sliding Window Maxima: soluzione banale, heap con lazy delete, BST.
//! In tutte le funzioni SWM: restituisci il massimo di ogni finestra di k elementi
//! (n - k + 1 valori); se k == 0 o k > n restituisci un vettore vuoto.
//!
//! SOLUZIONI: copia questo file su src/lib.rs per provarle (dopo aver salvato il tuo lavoro).

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;


/// E04 — SWM banale. Obiettivo: Θ(n·k).
/// Rust: `slice::windows(k)`.
pub fn swm_brute(a: &[i32], k: usize) -> Vec<i32> {
    if k == 0 || k > a.len() {
        return vec![];
    }
    a.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

/// E05 — SWM con max-heap e *lazy delete*: inserisci coppie (valore, posizione) e,
/// prima di leggere il massimo, estrai finché il massimo è fuori dalla finestra.
/// Obiettivo: Θ(n log n). Perché non Θ(n²)? (ogni elemento esce al più una volta)
/// Rust: `std::collections::BinaryHeap` (è già un max-heap), `peek`, `pop`, `while let`.
pub fn swm_heap(a: &[i32], k: usize) -> Vec<i32> {
    use std::collections::BinaryHeap;
    if k == 0 || k > a.len() {
        return vec![];
    }
    let mut heap = BinaryHeap::new();
    let mut out = Vec::with_capacity(a.len() - k + 1);
    for (i, &x) in a.iter().enumerate() {
        heap.push((x, i));
        if i + 1 >= k {
            // lazy delete: tolgo il massimo finché è fuori dalla finestra [i+1-k, i]
            while let Some(&(_, p)) = heap.peek() {
                if p + k <= i { heap.pop(); } else { break; }
            }
            out.push(heap.peek().unwrap().0);
        }
    }
    out
}

/// E06 — SWM con un BST usato come *multiset*: inserisci il nuovo elemento, togli quello
/// che esce, chiedi il massimo. Obiettivo: Θ(n log k).
/// Rust: `BTreeMap<i32, usize>` (valore -> quante volte compare nella finestra):
/// `entry(..).or_insert(0)`, `last_key_value()`, `remove` quando il contatore arriva a 0.
/// (Nella nota del prof si usa invece `BTreeSet<(valore, posizione)>`: prova anche quella.)
pub fn swm_btree(a: &[i32], k: usize) -> Vec<i32> {
    use std::collections::BTreeMap;
    if k == 0 || k > a.len() {
        return vec![];
    }
    let mut window: BTreeMap<i32, usize> = BTreeMap::new();
    let mut out = Vec::with_capacity(a.len() - k + 1);
    for (i, &x) in a.iter().enumerate() {
        *window.entry(x).or_insert(0) += 1;
        if i >= k {
            let old = a[i - k];
            let c = window.get_mut(&old).unwrap();
            *c -= 1;
            if *c == 0 {
                window.remove(&old);
            }
        }
        if i + 1 >= k {
            out.push(*window.last_key_value().unwrap().0);
        }
    }
    out
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
