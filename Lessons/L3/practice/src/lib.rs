//! L3 (23/09) — SWM in tempo lineare con la deque, Next Larger Element,
//! Trapping Rain Water, "un albero binario è un BST?" (note del prof).
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l3   (un esercizio: cargo test -p practice_l3 e07)

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;


/// E07 — SWM con la deque (stesso contratto di swm_brute in L2/practice).
/// Tieni in una deque le POSIZIONI dei right leaders della finestra: dalla testa togli chi è
/// uscito dalla finestra, dalla coda togli gli elementi <= del nuovo, la testa è il massimo.
/// Obiettivo: Θ(n) tempo, O(k) spazio. Sai dire perché è lineare anche se c'è un while dentro il for?
/// Rust: `std::collections::VecDeque` (`push_back`, `pop_front`, `back`, `front`).
pub fn swm_deque(a: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

/// E08 — Next Larger Element: per ogni posizione i, il primo elemento a destra
/// strettamente maggiore di a[i], oppure None.
/// Esempio: [2, 1, 4, 3, 3, 5] -> [Some(4), Some(4), Some(5), Some(5), Some(5), None]
/// Obiettivo: Θ(n) con uno stack (stessa idea della deque: gli elementi "dominati" escono).
pub fn next_larger(a: &[i32]) -> Vec<Option<i32>> {
    todo!()
}

/// E09 — Trapping Rain Water, soluzione del prof: acqua sopra i =
/// min(max a sinistra, max a destra) - h[i]. Precalcola i massimi (prefissi/suffissi).
/// Esempio: [6, 2, 0, 4, 0, 1, 0, 5, 0, 3] -> 26
/// Obiettivo: Θ(n) tempo, Θ(n) spazio extra.
/// LeetCode: https://leetcode.com/problems/trapping-rain-water/
pub fn trw_prefix(h: &[u64]) -> u64 {
    todo!()
}

/// E10 — Trapping Rain Water con due puntatori: Θ(n) tempo, O(1) spazio extra.
/// Idea: avanza sempre il lato il cui massimo è più basso. Perché in quel momento sai
/// già quanta acqua sta su quella posizione?
pub fn trw_two_pointers(h: &[u64]) -> u64 {
    todo!()
}

/// Albero binario per E11 (già pronto).
#[derive(Debug)]
pub struct Node {
    pub key: i64,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
        Some(Box::new(Node { key, left, right }))
    }
    pub fn leaf(key: i64) -> Option<Box<Node>> {
        Node::new(key, None, None)
    }
}

/// E11 — È un BST? Convenzione delle note: chiavi nel sottoalbero sinistro <= chiave < chiavi
/// nel sottoalbero destro. Controllare solo i figli diretti NON basta (controesempio: 10 con
/// figlio sinistro 5, che ha figlio destro 12).
/// Obiettivo: Θ(n), una visita. Idea delle note: una funzione ricorsiva che restituisce
/// (è BST?, minimo, massimo) del sottoalbero.
/// Rust: pattern matching su `Option<Box<Node>>`, `as_ref`/`as_deref`, tuple.
pub fn is_bst(root: &Option<Box<Node>>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    #[test]
    fn e07_swm_deque() {
        let a = [1, 3, 2, 1, 1, 2, 4, 3, 2]; // esempio della lezione
        assert_eq!(swm_deque(&a, 3), vec![3, 3, 2, 2, 4, 4, 4]);
        assert!(swm_deque(&a, 0).is_empty());
        assert!(swm_deque(&a, 10).is_empty());
        let mut rng = Rng(7);
        for _ in 0..1000 {
            let n = 1 + rng.below(50);
            let v = rng.vec_i32(n, -4, 5);
            let k = 1 + rng.below(n);
            let reference: Vec<i32> =
                (0..=n - k).map(|i| *v[i..i + k].iter().max().unwrap()).collect();
            assert_eq!(swm_deque(&v, k), reference, "a={v:?} k={k}");
        }
    }

    #[test]
    fn e08_next_larger() {
        assert_eq!(
            next_larger(&[2, 1, 4, 3, 3, 5]),
            vec![Some(4), Some(4), Some(5), Some(5), Some(5), None]
        );
        let mut rng = Rng(8);
        for _ in 0..500 {
            let n = rng.below(30);
            let v = rng.vec_i32(n, 0, 6);
            let reference: Vec<Option<i32>> =
                (0..n).map(|i| v[i + 1..].iter().copied().find(|&x| x > v[i])).collect();
            assert_eq!(next_larger(&v), reference, "{v:?}");
        }
    }

    fn trw_reference(h: &[u64]) -> u64 {
        (0..h.len())
            .map(|i| {
                let l = *h[..=i].iter().max().unwrap();
                let r = *h[i..].iter().max().unwrap();
                l.min(r) - h[i]
            })
            .sum()
    }

    fn trw_check(f: fn(&[u64]) -> u64) {
        assert_eq!(f(&[6, 2, 0, 4, 0, 1, 0, 5, 0, 3]), 26);
        assert_eq!(f(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6); // esempio LeetCode
        assert_eq!(f(&[]), 0);
        assert_eq!(f(&[5]), 0);
        let mut rng = Rng(9);
        for _ in 0..1000 {
            let n = rng.below(40);
            let h: Vec<u64> = (0..n).map(|_| rng.below(8) as u64).collect();
            assert_eq!(f(&h), trw_reference(&h), "{h:?}");
        }
    }

    #[test]
    fn e09_trw_prefix() {
        trw_check(trw_prefix);
    }

    #[test]
    fn e10_trw_two_pointers() {
        trw_check(trw_two_pointers);
    }

    #[test]
    fn e11_is_bst() {
        let bad = Node::new(10, Node::new(5, None, Node::leaf(12)), None);
        assert!(!is_bst(&bad));
        let good = Node::new(
            8,
            Node::new(5, Node::new(3, Node::leaf(1), None), Node::new(6, None, Node::leaf(7))),
            Node::new(10, Node::leaf(9), Node::leaf(12)),
        );
        assert!(is_bst(&good));
        assert!(is_bst(&None));
        assert!(is_bst(&Node::new(5, Node::leaf(5), None))); // uguale a sinistra: ok
        assert!(!is_bst(&Node::new(5, None, Node::leaf(5)))); // uguale a destra: no
        let deep = Node::new(10, None, Node::new(20, Node::leaf(9), None)); // 9 a destra di 10
        assert!(!is_bst(&deep));
    }
}
