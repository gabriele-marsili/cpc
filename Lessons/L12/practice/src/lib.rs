//! L12 — Fenwick tree: inversioni, remapping, Nested Segments (E50-E52).
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l12
//! Il Fenwick tree scrivilo tu (puoi riusare il tuo di E46): niente crate esterni.

#![allow(unused_variables, unused_mut, dead_code)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E50 — Numero di inversioni con un Fenwick tree sulle FREQUENZE dei valori (soluzione della lezione).
/// Valori 1 <= a[i] <= n (possono ripetersi). Scorri da sinistra: per ogni a[i] aggiungi al risultato quanti
/// valori > a[i] hai già visto (RangeSum(a[i]+1, max)), poi Add(a[i], 1).
/// Obiettivo: Θ(max(A) + n log max(A)) = Θ(n log n). Rust: struct Fenwick interna, `u64` per il risultato.
pub fn inversions_fenwick(a: &[u64]) -> u64 {
    todo!()
}

/// E51a — Remapping che preserva l'ordine: ogni valore diventa il suo rango tra i valori DISTINTI ordinati,
/// a partire da 1 (valori uguali -> stesso rango). [1, 150, 1e6, 155, 6, 8] -> [1, 4, 6, 5, 2, 3].
/// Obiettivo: Θ(n log n). NON assegnare gli id in ordine di apparizione (errore discusso a lezione).
/// Rust: `sort_unstable`, `dedup`, `binary_search` (o `partition_point`).
pub fn remap(a: &[u64]) -> Vec<u64> {
    todo!()
}

/// E51b — Inversioni con valori arbitrari (fino a 1e18): remap + E50. Obiettivo: Θ(n log n), spazio Θ(n).
pub fn inversions_any(a: &[u64]) -> u64 {
    todo!()
}

/// E52 — Nested Segments (Codeforces 652D, obbligatorio nella riga del 21/10/2026 della pagina del corso): per ogni segmento quanti segmenti contiene
/// (strettamente: gli estremi sono tutti distinti). Coordinate fino a ±1e9.
/// Obiettivo: Θ(n log n) con la soluzione della lezione: remap dei 2n estremi, UN Fenwick tree con un 1
/// sull'inizio di ogni segmento, segmenti per fine DECRESCENTE, RangeSum(l, r) - 1, poi togli il tuo 1.
/// Rust: `sort_unstable_by_key(|&i| Reverse(..))`, `flat_map` per raccogliere gli estremi.
pub fn nested_segments(segs: &[(i64, i64)]) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    fn inv_brute(a: &[u64]) -> u64 {
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
    fn nested_brute(s: &[(i64, i64)]) -> Vec<usize> {
        s.iter().map(|&(l, r)| s.iter().filter(|&&(l2, r2)| l < l2 && r2 < r).count()).collect()
    }

    #[test]
    fn e50_lecture_examples() {
        assert_eq!(inversions_fenwick(&[3, 2, 1, 6, 5, 4]), 6);
        // lavagna 26/10: 8 posizioni, valori fino a 6 (F è indicizzato per VALORE)
        assert_eq!(inversions_fenwick(&[3, 2, 1, 6, 5, 4, 3, 2]), 14);
        assert_eq!(inversions_fenwick(&[]), 0);
        assert_eq!(inversions_fenwick(&[1]), 0);
        let a: Vec<u64> = (1..=1000).rev().collect();
        assert_eq!(inversions_fenwick(&a), 499_500);
    }

    #[test]
    fn e50_against_brute_force() {
        let mut rng = Rng(50);
        for _ in 0..500 {
            let n = rng.below(40);
            let a: Vec<u64> = (0..n).map(|_| 1 + rng.below(n.max(1)) as u64).collect(); // 1 <= a[i] <= n, duplicati
            assert_eq!(inversions_fenwick(&a), inv_brute(&a), "{a:?}");
        }
    }

    #[test]
    fn e51_remap() {
        // lezione: [1, 150, 1_000_000, 155, 6, 8] -> ordinati 1 6 8 150 155 1e6 -> rango 1..6
        assert_eq!(remap(&[1, 150, 1_000_000, 155, 6, 8]), vec![1, 4, 6, 5, 2, 3]);
        // valori uguali -> stesso rango, ranghi densi 1..k
        assert_eq!(remap(&[50, 7, 50, 7, 1_000_000_000_000]), vec![2, 1, 2, 1, 3]);
        assert_eq!(remap(&[]), Vec::<u64>::new());
        let a = [1, 150, 1_000_000, 155, 6, 8];
        assert_eq!(inversions_any(&a), inv_brute(&a));
    }

    #[test]
    fn e51_against_brute_force() {
        let mut rng = Rng(51);
        for _ in 0..500 {
            let n = rng.below(40);
            let a: Vec<u64> = (0..n).map(|_| rng.next_u64() % 20 * 1_000_000_007).collect();
            let r = remap(&a);
            // l'ordine relativo è preservato e i ranghi sono densi
            for i in 0..n {
                for j in 0..n {
                    assert_eq!(a[i].cmp(&a[j]), r[i].cmp(&r[j]));
                }
            }
            let mut d = a.clone();
            d.sort_unstable();
            d.dedup();
            assert_eq!(r.iter().copied().max().unwrap_or(0), d.len() as u64);
            assert_eq!(inversions_any(&a), inv_brute(&a), "{a:?}");
        }
    }

    #[test]
    fn e52_lecture_example() {
        // lavagna 26/10 (coordinate già rimappate)
        let s = [(1, 5), (2, 10), (3, 4), (6, 8), (7, 11), (9, 12)];
        assert_eq!(nested_segments(&s), vec![1, 2, 0, 0, 0, 0]);
        // esempi di Codeforces 652D
        assert_eq!(nested_segments(&[(1, 8), (2, 3), (4, 7), (5, 6)]), vec![3, 0, 1, 0]);
        assert_eq!(nested_segments(&[(3, 4), (1, 5), (2, 6)]), vec![0, 1, 1]);
        assert_eq!(nested_segments(&[]), Vec::<usize>::new());
        // coordinate enormi e negative: serve il remapping
        assert_eq!(nested_segments(&[(-1_000_000_000, 1_000_000_000), (-5, 5)]), vec![1, 0]);
    }

    #[test]
    fn e52_against_brute_force() {
        let mut rng = Rng(52);
        for _ in 0..400 {
            let n = rng.below(15);
            let p = rng.permutation(2 * n);
            let s: Vec<(i64, i64)> = p
                .chunks(2)
                .map(|c| {
                    let (x, y) = (c[0] as i64 * 1_000_003 - 50_000_000, c[1] as i64 * 1_000_003 - 50_000_000);
                    (x.min(y), x.max(y))
                })
                .collect();
            assert_eq!(nested_segments(&s), nested_brute(&s), "{s:?}");
        }
    }
}
