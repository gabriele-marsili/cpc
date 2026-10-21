//! L10 — applicazioni delle prefix sums (Little Girl, Number of Ways) e Fenwick tree (E43-E46).
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l10
//! Indici 1-based come alla lavagna, salvo dove detto.

#![allow(unused_variables, unused_mut, dead_code)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E43 — Little Girl and Maximum Sum (Codeforces 276C).
/// `q[k] = (i, j)` 1-based, i <= j: la query vale A[i] + ... + A[j]. Permuta A per massimizzare la somma
/// dei valori di tutte le query e restituisci quel massimo.
/// Obiettivo: Θ(n log n + m): frequenze con il trucco "+1 in i, -1 in j+1, poi prefix sums" (Θ(n + m)),
/// poi i valori più grandi alle posizioni più frequenti.
/// Rust: `iter().scan`, `sort_unstable_by(|x, y| y.cmp(x))`, `zip`, `map(..).sum()`.
pub fn little_girl(a: &[i64], q: &[(usize, usize)]) -> i64 {
    todo!()
}

/// E44 — Le stesse frequenze di E43, ma con la prima soluzione della lezione: SWEEP LINE.
/// Restituisci F[0..n] dove F[p-1] = numero di query (i, j) con i <= p <= j (query 1-based).
/// Obiettivo: Θ(n + m log m): un evento +1 all'inizio e -1 dopo la fine di ogni intervallo, eventi
/// ordinati, una scansione delle posizioni 1..=n. NON usare l'array B di E43.
/// Rust: `Vec<(usize, i64)>` di eventi, `sort_unstable` su tuple (ordine lessicografico).
pub fn frequencies_sweep(n: usize, q: &[(usize, usize)]) -> Vec<i64> {
    todo!()
}

/// E45 — Number of Ways (Codeforces 466C).
/// Quanti modi ci sono di tagliare A in 3 parti contigue NON vuote con la stessa somma?
/// Obiettivo: Θ(n): T = S/3; "+" dove la prefix sum vale T, "punti rossi" dove la suffix sum vale T,
/// conta le coppie (+, rosso) con il rosso almeno 2 posizioni dopo il +.
/// Attenzione: n può essere 0, 1 o 2 (risposta 0, senza panic) e i valori possono essere negativi.
/// Rust: `u64::from(bool)`, cicli con `(0..n).rev()`.
pub fn number_of_ways(a: &[i64]) -> u64 {
    todo!()
}

/// E46 — Fenwick tree (BIT) da zero, 1-based come alla lavagna.
/// Il nodo i copre (i - lsb(i), i], dove lsb(i) è il bit meno significativo a 1 di i.
/// `sum(i)` = A[1] + ... + A[i] (sum(0) = 0), `add(i, v)`: A[i] += v, entrambi O(log n).
/// `node(i)` restituisce il valore memorizzato nel nodo i (serve ai test per controllare la struttura).
/// Rust: `i & i.wrapping_neg()` per lsb, oppure `1 << i.trailing_zeros()`.
pub struct Bit {
    t: Vec<i64>,
}

impl Bit {
    pub fn new(n: usize) -> Self {
        todo!()
    }
    pub fn add(&mut self, i: usize, v: i64) {
        todo!()
    }
    pub fn sum(&self, i: usize) -> i64 {
        todo!()
    }
    /// A[i] + ... + A[j], 1 <= i <= j <= n
    pub fn range_sum(&self, i: usize, j: usize) -> i64 {
        todo!()
    }
    pub fn node(&self, i: usize) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    fn value(a: &[i64], q: &[(usize, usize)]) -> i64 {
        q.iter().map(|&(i, j)| a[i - 1..j].iter().sum::<i64>()).sum()
    }
    fn little_brute(a: &[i64], q: &[(usize, usize)]) -> i64 {
        fn rec(a: &mut Vec<i64>, k: usize, q: &[(usize, usize)], best: &mut i64) {
            if k == a.len() {
                *best = (*best).max(value(a, q));
                return;
            }
            for i in k..a.len() {
                a.swap(k, i);
                rec(a, k + 1, q, best);
                a.swap(k, i);
            }
        }
        let mut best = i64::MIN;
        rec(&mut a.to_vec(), 0, q, &mut best);
        best
    }
    fn freq_brute(n: usize, q: &[(usize, usize)]) -> Vec<i64> {
        (1..=n).map(|p| q.iter().filter(|&&(i, j)| i <= p && p <= j).count() as i64).collect()
    }
    fn ways_brute(a: &[i64]) -> u64 {
        let n = a.len();
        let mut c = 0;
        for i in 1..n {
            for j in i + 1..n {
                let (x, y, z): (i64, i64, i64) = (a[..i].iter().sum(), a[i..j].iter().sum(), a[j..].iter().sum());
                if x == y && y == z {
                    c += 1;
                }
            }
        }
        c
    }
    fn rand_queries(rng: &mut Rng, n: usize, m: usize) -> Vec<(usize, usize)> {
        (0..m)
            .map(|_| {
                let i = 1 + rng.below(n);
                (i, i + rng.below(n - i + 1))
            })
            .collect()
    }

    #[test]
    fn e43_lecture_example() {
        // lezione: A = [5,2,4,1,3], query (1,5), (2,3), (2,3): 27 così com'è, 33 il massimo
        let q = [(1, 5), (2, 3), (2, 3)];
        assert_eq!(little_girl(&[5, 2, 4, 1, 3], &q), 33);
        // Codeforces 276C, primo esempio
        assert_eq!(little_girl(&[5, 3, 2], &[(1, 2), (2, 3), (1, 3)]), 25);
    }

    #[test]
    fn e43_against_brute_force() {
        let mut rng = Rng(43);
        for _ in 0..400 {
            let n = 1 + rng.below(6);
            let a: Vec<i64> = rng.vec_i32(n, -5, 10).into_iter().map(i64::from).collect();
            let m = rng.below(6);
            let q = rand_queries(&mut rng, n, m);
            assert_eq!(little_girl(&a, &q), little_brute(&a, &q), "a={a:?} q={q:?}");
        }
    }

    #[test]
    fn e44_lecture_example() {
        // lezione: Q = (2,4), (1,3), (2,5) su n = 5 -> F = [1,3,3,2,1]
        assert_eq!(frequencies_sweep(5, &[(2, 4), (1, 3), (2, 5)]), vec![1, 3, 3, 2, 1]);
        assert_eq!(frequencies_sweep(3, &[]), vec![0, 0, 0]);
    }

    #[test]
    fn e44_against_brute_force() {
        let mut rng = Rng(44);
        for _ in 0..500 {
            let n = 1 + rng.below(20);
            let m = rng.below(15);
            let q = rand_queries(&mut rng, n, m);
            assert_eq!(frequencies_sweep(n, &q), freq_brute(n, &q), "n={n} q={q:?}");
        }
    }

    #[test]
    fn e45_examples() {
        assert_eq!(number_of_ways(&[1, 2, 3, 0, 3]), 2); // lezione
        assert_eq!(number_of_ways(&[0, 1, -1, 0]), 1); // Codeforces
        assert_eq!(number_of_ways(&[4, 1]), 0); // Codeforces
        assert_eq!(number_of_ways(&[0]), 0); // n = 1: il codice del blog qui va in panic
        assert_eq!(number_of_ways(&[0, 0, 0, 0]), 3);
    }

    #[test]
    fn e45_against_brute_force() {
        let mut rng = Rng(45);
        for _ in 0..800 {
            let n = rng.below(14);
            let a: Vec<i64> = rng.vec_i32(n, -2, 3).into_iter().map(i64::from).collect();
            assert_eq!(number_of_ways(&a), ways_brute(&a), "a={a:?}");
        }
    }

    #[test]
    fn e46_lecture_example() {
        // lezione: A = [3,2,-1,5,7,-3,2,1]; nodi 1,2,4,8 = 3,5,9,16 (prefissi), 6 = [5,6] = 4
        let a = [3, 2, -1, 5, 7, -3, 2, 1];
        let mut b = Bit::new(8);
        for (k, &v) in a.iter().enumerate() {
            b.add(k + 1, v);
        }
        let nodes: Vec<i64> = (1..=8).map(|i| b.node(i)).collect();
        assert_eq!(nodes, vec![3, 5, -1, 9, 7, 4, 2, 16]);
        assert_eq!(b.sum(7), 15);
        assert_eq!(b.range_sum(3, 5), 11);
        b.add(3, 10);
        assert_eq!(b.sum(8), 26);
        assert_eq!(b.node(4), 19);
    }

    #[test]
    fn e46_against_naive() {
        let mut rng = Rng(46);
        for _ in 0..100 {
            let n = 1 + rng.below(40);
            let mut b = Bit::new(n);
            let mut a = vec![0i64; n + 1];
            for _ in 0..300 {
                let i = 1 + rng.below(n);
                if rng.below(2) == 0 {
                    let v = rng.below(201) as i64 - 100;
                    b.add(i, v);
                    a[i] += v;
                } else {
                    let j = i + rng.below(n - i + 1);
                    assert_eq!(b.sum(i), a[1..=i].iter().sum::<i64>());
                    assert_eq!(b.range_sum(i, j), a[i..=j].iter().sum::<i64>());
                }
            }
        }
    }
}
