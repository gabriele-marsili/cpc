//! L15 — E59-E62: gli esercizi alla fine delle note del prof "Segment Tree" (2026);
//! E63-E64: Mo's algorithm (E63 = compito a voce del prof alla fine della lezione registrata).
//! https://pages.di.unipi.it/rossano/blog/2026/segmenttree/ (sezione "Exercises")
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l15  (i controlli numerici sono quelli scritti dal prof nelle note)
//! Posizioni 0-based come nelle note. Niente crate esterni.

#![allow(unused_variables, unused_mut, dead_code)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E59 — Segment tree GENERICO, parametrizzato dal tipo dei valori, dalla funzione di combinazione
/// (associativa) e dall'elemento neutro. `set(i, x)`: A[i] = x; `query(i, j)`: combinazione di A[i..=j]
/// (da sinistra a destra: la combinazione può NON essere commutativa). Θ(log n) chiamate a `combine`.
/// Rust: generics con vincoli `T: Clone`, `F: Fn(T, T) -> T`; il neutro restituito sul "no overlap".
pub struct GenericSegTree<T, F> {
    // i tuoi campi
    _todo: std::marker::PhantomData<(T, F)>,
}

impl<T: Clone, F: Fn(T, T) -> T> GenericSegTree<T, F> {
    pub fn new(a: &[T], neutral: T, combine: F) -> Self {
        todo!()
    }
    pub fn set(&mut self, i: usize, x: T) {
        todo!()
    }
    pub fn query(&self, i: usize, j: usize) -> T {
        todo!()
    }
}

/// E60 — `search(p, s)`: la più piccola posizione i >= p con A[p] + ... + A[i] >= s (`None` se non esiste).
/// Valori NON negativi; `add(i, v)` con v >= 0. Obiettivo O(log n): visita a tre casi su [p, n-1]
/// che si ferma appena ha "raccolto" s (non una binary search for the answer, che darebbe log² n).
pub struct SearchFrom {
    // i tuoi campi
}

impl SearchFrom {
    pub fn new(a: &[u64]) -> Self {
        todo!()
    }
    pub fn add(&mut self, i: usize, v: u64) {
        todo!()
    }
    pub fn search(&self, p: usize, s: u64) -> Option<usize> {
        todo!()
    }
}

/// E61 — range add + range minimum con lazy propagation: `range_update(i, j, v)` aggiunge v a A[i..=j],
/// `range_min(i, j)`; entrambe O(log n). Cosa serve alla lazy propagation (note): applicare un'addizione
/// pendente al minimo di un nodo senza guardare i figli, e comporre due addizioni pendenti.
pub struct LazyMin {
    // i tuoi campi
}

impl LazyMin {
    pub fn new(a: &[i64]) -> Self {
        todo!()
    }
    pub fn range_update(&mut self, i: usize, j: usize, v: i64) {
        todo!()
    }
    pub fn range_min(&mut self, i: usize, j: usize) -> i64 {
        todo!()
    }
}

/// E62 — ONLINE: quanti valori di A[i..=j] sono <= x? Con un segment tree persistente.
/// Suggerimento del prof: inserisci le POSIZIONI di A in ordine crescente di valore.
/// Obiettivo: costruzione O(n log n), query O(log n), spazio O(n log n). Valori qualsiasi (anche negativi).
pub struct AtMost {
    // i tuoi campi
}

impl AtMost {
    pub fn new(a: &[i64]) -> Self {
        todo!()
    }
    pub fn count(&self, i: usize, j: usize, x: i64) -> usize {
        todo!()
    }
}

/// E63 — Powerful Array (il compito a voce del prof alla fine della lezione registrata: "provate a risolverlo
/// con la nuova tecnica"). Query OFFLINE power(l, r) = somma, su ogni valore distinto e in A[l..=r],
/// di e · K_e², con K_e = occorrenze di e in A[l..=r]. Valori positivi < `max_val`.
/// Obiettivo: Θ((n + q)·√n) con Mo; restituisci le risposte nell'ordine ORIGINALE delle query.
/// Rust: closure o funzioni add/remove che aggiornano contatori e risposta in O(1), `sort_by_key` con tupla.
pub fn powerful_array(a: &[u64], max_val: usize, queries: &[(usize, usize)]) -> Vec<u64> {
    todo!()
}

/// E64 — Distinct: numero di valori distinti in A[l..=r] (valori < n), query offline, con Mo.
/// Stesso schema di E63: cambia solo cosa fanno add e remove.
pub fn distinct(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    const A: [i64; 8] = [5, 1, 3, 1, 4, 7, 2, 1]; // esempio delle note del prof (2026)

    #[test]
    fn e59_checks_of_the_notes() {
        let sum = GenericSegTree::new(&A, 0i64, |x, y| x + y);
        assert_eq!(sum.query(2, 6), 17);
        // RMQ: (valore, posizione), il min di tuple sceglie a parità la posizione più a sinistra
        let pairs: Vec<(i64, usize)> = A.iter().enumerate().map(|(p, &v)| (v, p)).collect();
        let mut rmq = GenericSegTree::new(&pairs, (i64::MAX, usize::MAX), |x, y| x.min(y));
        assert_eq!(rmq.query(2, 7).1, 3);
        rmq.set(1, (5, 1)); // add(1, 4) delle note
        assert_eq!(rmq.query(0, 7).1, 3);
        // minimo con occorrenze
        let occ: Vec<(i64, u32)> = A.iter().map(|&v| (v, 1)).collect();
        let comb = |x: (i64, u32), y: (i64, u32)| {
            if x.0 == y.0 { (x.0, x.1 + y.1) } else if x.0 < y.0 { x } else { y }
        };
        let mut mo = GenericSegTree::new(&occ, (i64::MAX, 0), comb);
        assert_eq!(mo.query(0, 7), (1, 3));
        mo.set(1, (0, 1)); // add(1, -1)
        assert_eq!(mo.query(0, 7), (0, 1));
    }

    #[test]
    fn e59_non_commutative() {
        // concatenazione di stringhe: associativa ma NON commutativa (sinistra prima, destra dopo)
        let s: Vec<String> = "abcdefg".chars().map(|c| c.to_string()).collect();
        let mut t = GenericSegTree::new(&s, String::new(), |x, y| x + &y);
        assert_eq!(t.query(1, 5), "bcdef");
        t.set(3, "X".to_string());
        assert_eq!(t.query(0, 6), "abcXefg");
    }

    #[test]
    fn e60_checks_and_random() {
        let mut t = SearchFrom::new(&[5, 1, 3, 1, 4, 7, 2, 1]);
        assert_eq!(t.search(2, 8), Some(4)); // 3 + 1 + 4 >= 8
        assert_eq!(t.search(0, 20), Some(5)); // = search(20) delle note
        assert_eq!(t.search(7, 2), None);
        assert_eq!(t.search(5, 0), Some(5));
        let mut rng = Rng(60);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<u64> = (0..n).map(|_| rng.below(5) as u64).collect();
            t = SearchFrom::new(&a);
            for _ in 0..60 {
                let p = rng.below(n);
                if rng.below(3) == 0 {
                    let v = rng.below(4) as u64;
                    t.add(p, v);
                    a[p] += v;
                } else {
                    let s = rng.below(5 * n + 2) as u64;
                    let mut acc = 0;
                    let want = (p..n).find(|&i| {
                        acc += a[i];
                        acc >= s
                    });
                    assert_eq!(t.search(p, s), want, "a={a:?} p={p} s={s}");
                }
            }
        }
    }

    #[test]
    fn e61_checks_and_random() {
        let mut t = LazyMin::new(&A);
        t.range_update(0, 4, 20);
        assert_eq!(t.range_min(0, 7), 1);
        assert_eq!(t.range_min(0, 3), 21);
        let mut rng = Rng(61);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<i64> = rng.vec_i32(n, -20, 20).into_iter().map(i64::from).collect();
            let mut t = LazyMin::new(&a);
            for _ in 0..80 {
                let i = rng.below(n);
                let j = i + rng.below(n - i);
                if rng.below(2) == 0 {
                    let v = rng.below(21) as i64 - 10;
                    t.range_update(i, j, v);
                    for x in &mut a[i..=j] {
                        *x += v;
                    }
                } else {
                    assert_eq!(t.range_min(i, j), *a[i..=j].iter().min().unwrap());
                }
            }
        }
    }

    #[test]
    fn e62_checks_and_random() {
        let t = AtMost::new(&A);
        assert_eq!(t.count(1, 6, 2), 3); // 1, 1, 2
        assert_eq!(t.count(0, 7, 0), 0);
        assert_eq!(t.count(0, 7, 100), 8);
        let mut rng = Rng(62);
        for _ in 0..200 {
            let n = 1 + rng.below(40);
            let a: Vec<i64> = rng.vec_i32(n, -1000, 1000).into_iter().map(|x| i64::from(x) * 3).collect();
            let t = AtMost::new(&a);
            for _ in 0..60 {
                let i = rng.below(n);
                let j = i + rng.below(n - i);
                let x = rng.below(6001) as i64 - 3000;
                assert_eq!(t.count(i, j, x), a[i..=j].iter().filter(|&&y| y <= x).count());
            }
        }
    }

    #[test]
    fn e63_lecture_and_random() {
        // slide "Powerful Array": A = [2, 1, 3, 2, 2, 2, 1, 4], power(1, 6) = 1·2² + 3·1² + 2·3² = 25
        let a = [2u64, 1, 3, 2, 2, 2, 1, 4];
        assert_eq!(powerful_array(&a, 5, &[(1, 6), (0, 0), (0, 7)]), vec![25, 2, 1 * 4 + 2 * 16 + 3 + 4]);
        let mut rng = Rng(63);
        for _ in 0..200 {
            let n = 1 + rng.below(50);
            let a: Vec<u64> = (0..n).map(|_| 1 + rng.below(6) as u64).collect();
            let q: Vec<(usize, usize)> = (0..1 + rng.below(40))
                .map(|_| {
                    let l = rng.below(n);
                    (l, l + rng.below(n - l))
                })
                .collect();
            let got = powerful_array(&a, 7, &q);
            for (k, &(l, r)) in q.iter().enumerate() {
                let mut c = [0u64; 7];
                for &x in &a[l..=r] {
                    c[x as usize] += 1;
                }
                let want: u64 = (0..7).map(|e| e as u64 * c[e] * c[e]).sum();
                assert_eq!(got[k], want, "a={a:?} q={:?}", (l, r));
            }
        }
    }

    #[test]
    fn e64_random() {
        assert_eq!(distinct(&[1, 3, 1, 2, 1, 3, 3, 0, 2, 1, 2, 2, 3, 1, 0, 3], &[(0, 6), (2, 10), (7, 7)]), vec![3, 4, 1]);
        let mut rng = Rng(64);
        for _ in 0..200 {
            let n = 1 + rng.below(50);
            let a: Vec<usize> = (0..n).map(|_| rng.below(n.min(6))).collect();
            let q: Vec<(usize, usize)> = (0..1 + rng.below(40))
                .map(|_| {
                    let l = rng.below(n);
                    (l, l + rng.below(n - l))
                })
                .collect();
            let got = distinct(&a, &q);
            for (k, &(l, r)) in q.iter().enumerate() {
                let mut v = a[l..=r].to_vec();
                v.sort_unstable();
                v.dedup();
                assert_eq!(got[k], v.len());
            }
        }
    }
}
