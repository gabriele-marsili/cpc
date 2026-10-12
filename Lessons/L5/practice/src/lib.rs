//! L5 (30/09) — Duplicato con accesso casuale (spostare in cella, Floyd), maggioranza (Boyer-Moore).
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l5   (un esercizio: cargo test -p practice_l5 e19)

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;


/// E19 — Soluzione del prof in L5 (distrugge A): parti dall'ultima posizione n, prendi l'elemento
/// e mettilo nella cella col suo indice, togliendo quello che c'era; continua con l'elemento tolto.
/// Quando la cella di destinazione contiene già quel valore, hai trovato il duplicato.
/// Esempio: [1, 3, 5, 6, 5, 2, 0, 4, 7]: 7 va in 7 (esce 4), 4 in 4 (esce 5), 5 in 5 (esce 2),
/// 2 in 2 (esce 5), 5 in 5: c'è già -> 5.
/// Obiettivo: Θ(n) tempo, O(1) spazio extra. Stesso contratto dei problemi sul duplicato di L4.
/// Rust: `std::mem::replace` per "metti x qui e dammi quello che c'era".
pub fn dup_place_from_n(a: &mut [usize]) -> usize {
    todo!()
}

/// E20 — Duplicato con `a` in SOLA LETTURA, Θ(n) tempo, O(1) spazio extra: algoritmo di Floyd.
/// Vedi `a` come lista i -> a[i], partendo da n (nessuno punta a n): si entra in un ciclo e
/// l'ingresso del ciclo è un duplicato. Fase 1: lento (1 passo) e veloce (2 passi) si incontrano
/// nel ciclo. Fase 2: uno riparte dall'inizio, entrambi a 1 passo: si incontrano all'ingresso.
/// Stesso contratto dei problemi sul duplicato di L4.
pub fn dup_floyd(a: &[usize]) -> usize {
    todo!()
}

/// E21 — Elemento di maggioranza (compare più di n/2 volte), se esiste.
/// Boyer-Moore: candidato + contatore, poi una seconda passata di verifica.
/// Esempio delle note: [5, 3, 1, 1, 2, 3, 1, 1, 1] -> Some(1);  [1, 2, 3] -> None
/// Obiettivo: Θ(n) tempo, O(1) spazio.
pub fn majority(a: &[i64]) -> Option<i64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    #[test]
    fn e19_dup_place_from_n() {
        assert_eq!(dup_place_from_n(&mut vec![1, 3, 5, 6, 5, 2, 0, 4, 7]), 5);
        let mut rng = Rng(190);
        for _ in 0..2000 {
            let n = 1 + rng.below(40);
            let a: Vec<usize> = (0..=n).map(|_| rng.below(n)).collect();
            let d = dup_place_from_n(&mut a.clone());
            assert!(a.iter().filter(|&&x| x == d).count() >= 2, "{a:?}");
        }
    }

    #[test]
    fn e20_dup_floyd() {
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
    fn e21_majority() {
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
}
