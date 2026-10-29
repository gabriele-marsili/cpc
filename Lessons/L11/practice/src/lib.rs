//! L11 — Fenwick tree (navigazione con i bit) e conteggio delle inversioni (E47-E49).
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l11
//! Posizioni 1-based come alla lavagna.

#![allow(unused_variables, unused_mut, dead_code)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E47a — Il bit a 1 meno significativo di i (lsb(7) = 1, lsb(6) = 2, lsb(0) = 0).
/// Obiettivo: O(1), senza cicli. Rust: `i & i.wrapping_neg()` (complemento a 2, come alla lavagna)
/// oppure `1 << i.trailing_zeros()` (attento a i = 0).
pub fn lsb(i: usize) -> usize {
    todo!()
}

/// E47b — Padre del nodo i nel Fenwick tree: si azzera il bit meno significativo. parent(7) = 6.
pub fn parent(i: usize) -> usize {
    todo!()
}

/// E47c — Livello del nodo i (la radice finta 0 è al livello 0): numero di bit a 1.
/// Rust: `count_ones()` restituisce u32.
pub fn level(i: usize) -> usize {
    todo!()
}

/// E47d — I nodi visitati da Sum(i), i >= 1, nell'ordine, SENZA la radice 0. sum_path(7) = [7, 6, 4].
/// Obiettivo: Θ(log n). Rust: `Vec::with_capacity`, `while`.
pub fn sum_path(i: usize) -> Vec<usize> {
    todo!()
}

/// E47e — I nodi visitati da Add(i, v) in un Fenwick tree di n posizioni (1 <= i <= n), nell'ordine.
/// add_path(5, 8) = [5, 6, 8]. Obiettivo: Θ(log n).
pub fn add_path(i: usize, n: usize) -> Vec<usize> {
    todo!()
}

/// E48 — Numero di inversioni (coppie i < j con a[i] > a[j]) con la variante del MERGESORT.
/// Obiettivo: Θ(n log n). Il risultato può superare 2^32: restituisci u64.
/// Rust: ricorsione su slice `&mut [i64]` con `split_at_mut`, buffer ausiliario `Vec<i64>`.
pub fn merge_sort_inversions(a: &[i64]) -> u64 {
    todo!()
}

/// E49 — Numero di inversioni contando gli scambi dell'INSERTION SORT (prima idea vista a lezione).
/// Obiettivo: Θ(n + #inversioni). Rust: `slice::swap`.
pub fn insertion_sort_inversions(a: &[i64]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    fn inv_brute(a: &[i64]) -> u64 {
        let mut c = 0;
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                if a[i] > a[j] {
                    c += 1;
                }
            }
        }
        c
    }

    #[test]
    fn e47_lecture_examples() {
        // lavagna: 7 = 0111 -> 0001; Sum(7) visita 7, 6, 4 (poi la radice 0); Add(5, ·) visita 5, 6, 8
        assert_eq!(lsb(7), 1);
        assert_eq!(lsb(6), 2);
        assert_eq!(lsb(0b1011000), 0b1000);
        assert_eq!(parent(7), 6);
        assert_eq!(parent(6), 4);
        assert_eq!(parent(4), 0);
        assert_eq!(sum_path(7), vec![7, 6, 4]);
        assert_eq!(add_path(5, 8), vec![5, 6, 8]);
        assert_eq!(add_path(8, 8), vec![8]);
        assert_eq!(sum_path(8), vec![8]);
        // livello = numero di bit a 1: 1, 2, 4, 8 al livello 1; 3, 5, 6 al livello 2; 7 al livello 3
        assert_eq!(level(8), 1);
        assert_eq!(level(6), 2);
        assert_eq!(level(7), 3);
        assert_eq!(level(0), 0);
    }

    #[test]
    fn e47_against_definition() {
        for i in 1usize..2000 {
            let l = 1usize << i.trailing_zeros();
            assert_eq!(lsb(i), l);
            assert_eq!(level(i), i.count_ones() as usize);
            // il nodo i copre (i - lsb(i), i]: i nodi di sum_path(i) partizionano [1, i]
            let p = sum_path(i);
            let mut covered = 0;
            let mut hi = i;
            for &x in &p {
                assert_eq!(x, hi, "i={i}");
                covered += lsb(x);
                hi = x - lsb(x);
            }
            assert_eq!(covered, i);
            assert!(p.len() <= 11);
            // add_path(i, n): esattamente i nodi x <= n il cui intervallo contiene i
            let n = 2000;
            let expected: Vec<usize> = (i..=n).filter(|&x| x - lsb(x) < i).collect();
            assert_eq!(add_path(i, n), expected, "i={i}");
        }
    }

    #[test]
    fn e48_e49_examples() {
        // lavagna: A = 3 2 1 6 5 4 ha 6 inversioni
        for a in [vec![3, 2, 1, 6, 5, 4], vec![3, 2, 1], vec![], vec![7], vec![1, 2, 3]] {
            let b = inv_brute(&a);
            assert_eq!(insertion_sort_inversions(&a), b, "{a:?}");
            assert_eq!(merge_sort_inversions(&a), b, "{a:?}");
        }
        assert_eq!(merge_sort_inversions(&[3, 2, 1, 6, 5, 4]), 6);
        // lavagna 84:40: l'albero del mergesort su 3 2 6 5 4 3 7 8
        assert_eq!(merge_sort_inversions(&[3, 2, 6, 5, 4, 3, 7, 8]), 7); // 3>2; 6>5,4,3; 5>4,3; 4>3
        // decrescente: n(n-1)/2
        let a: Vec<i64> = (1..=100).rev().collect();
        assert_eq!(merge_sort_inversions(&a), 4950);
        assert_eq!(insertion_sort_inversions(&a), 4950);
    }

    #[test]
    fn e48_e49_against_brute_force() {
        let mut rng = Rng(47);
        for _ in 0..500 {
            let n = rng.below(40);
            let a: Vec<i64> = rng.vec_i32(n, -10, 10).into_iter().map(i64::from).collect(); // duplicati e negativi
            let b = inv_brute(&a);
            assert_eq!(insertion_sort_inversions(&a), b, "{a:?}");
            assert_eq!(merge_sort_inversions(&a), b, "{a:?}");
        }
    }

    #[test]
    fn e48_large() {
        // 2e5 elementi: deve essere veloce (Θ(n log n)) e il risultato non sta in u32
        let n = 200_000;
        let a: Vec<i64> = (0..n).rev().collect();
        assert_eq!(merge_sort_inversions(&a), (n as u64) * (n as u64 - 1) / 2);
    }
}
