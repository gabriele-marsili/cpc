//! L9 — Frogs and Mosquitoes (E39) e prefix sums statiche (E40-E42). E39 era l'ex E30 di L7.
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l9

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E39 — Frogs and Mosquitoes (Codeforces 609F, OBBLIGATORIO all'esame).
/// `frogs[i] = (x_i, t_i)` (x distinte), `mosquitoes[j] = (b_j, a_j)` nell'ordine di atterraggio.
/// La rana i mangia b se x_i <= b <= x_i + t_i; se possono più rane, la più a sinistra; dopo aver
/// mangiato t_i += a e può mangiare le zanzare atterrate prima e rimaste lì.
/// Restituisci per ogni rana (in ordine di input) (quante ne ha mangiate, lingua finale).
/// Obiettivo: Θ((n + m) log(n + m)). Prima scrivi la versione banale O(n·m), poi quella con:
/// BST delle rane per inizio dell'intervallo (predecessore di b), rimozione delle sovrapposizioni,
/// BST delle zanzare in attesa. Rust: `BTreeMap::range(..=b).next_back()`, `range(s..=e).next()`.
pub fn frogs_and_mosquitoes(frogs: &[(i64, i64)], mosquitoes: &[(i64, i64)]) -> Vec<(usize, i64)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rng(u64);
    impl Rng {
        fn below(&mut self, n: u64) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0 % n
        }
    }

    fn frogs_brute(frogs: &[(i64, i64)], mosq: &[(i64, i64)]) -> Vec<(usize, i64)> {
        let n = frogs.len();
        let mut eaten = vec![0; n];
        let mut t: Vec<i64> = frogs.iter().map(|f| f.1).collect();
        let mut waiting: Vec<(i64, i64)> = Vec::new();
        for &m in mosq {
            waiting.push(m);
            loop {
                // un qualsiasi zanzara mangiabile: la rana più a sinistra che la raggiunge la mangia
                let mut done = true;
                for w in 0..waiting.len() {
                    let (b, a) = waiting[w];
                    let best = (0..n)
                        .filter(|&i| frogs[i].0 <= b && b <= frogs[i].0 + t[i])
                        .min_by_key(|&i| frogs[i].0);
                    if let Some(i) = best {
                        eaten[i] += 1;
                        t[i] += a;
                        waiting.remove(w);
                        done = false;
                        break;
                    }
                }
                if done {
                    break;
                }
            }
        }
        (0..n).map(|i| (eaten[i], t[i])).collect()
    }

    #[test]
    fn e39_frogs_codeforces_examples() {
        let frogs = [(10, 2), (15, 0), (6, 1), (0, 1)];
        let mosq = [(110, 10), (1, 1), (6, 0), (15, 10), (14, 100), (12, 2)];
        assert_eq!(frogs_and_mosquitoes(&frogs, &mosq), vec![(3, 114), (1, 10), (1, 1), (1, 2)]);
        assert_eq!(frogs_and_mosquitoes(&[(10, 2)], &[(20, 2), (12, 1)]), vec![(1, 3)]);
    }

    #[test]
    fn e39_frogs_against_brute_force() {
        let mut rng = Rng(11);
        for _ in 0..3000 {
            let n = 1 + rng.below(5) as usize;
            let mut xs: Vec<i64> = (0..30).collect();
            let mut frogs = Vec::new();
            for _ in 0..n {
                let k = rng.below(xs.len() as u64) as usize;
                frogs.push((xs.remove(k), rng.below(5) as i64));
            }
            let m = rng.below(8) as usize;
            let mosq: Vec<(i64, i64)> =
                (0..m).map(|_| (rng.below(40) as i64, rng.below(6) as i64)).collect();
            assert_eq!(frogs_and_mosquitoes(&frogs, &mosq), frogs_brute(&frogs, &mosq), "{frogs:?} {mosq:?}");
        }
    }
}

// ===================== Aggiunti dopo la registrazione di L9 (14/10) =====================

/// E40 — Prefix sums statiche (L9, seconda parte). Indici 1-based come a lezione:
/// `sum(i)` = A[1] + ... + A[i] (con `sum(0) = 0`), `range_sum(i, j)` = A[i] + ... + A[j] (i <= j).
/// Obiettivo: costruzione Θ(n), query O(1); dopo la costruzione A non serve più (struttura implicita).
/// Rust: `iter().scan(...)` (il codice del prof) oppure un `Vec::with_capacity(n + 1)` con lo 0 iniziale.
#[allow(dead_code)] // togli questa riga quando usi `p`
pub struct PrefixSums {
    p: Vec<i64>,
}

impl PrefixSums {
    pub fn new(a: &[i64]) -> Self {
        todo!()
    }
    pub fn sum(&self, i: usize) -> i64 {
        todo!()
    }
    pub fn range_sum(&self, i: usize, j: usize) -> i64 {
        todo!()
    }
}

/// E41 — Ilya and Queries (Codeforces 313B, L9). `s` fatta di 'a'/'b' (o qualsiasi byte);
/// per ogni query (l, r) 1-based con l < r restituisci il numero di k con l <= k < r e s[k] == s[k+1].
/// Esempio della lezione: s = "abbabba", (2, 6) -> 2. Obiettivo: Θ(n + m) in totale.
/// Rust: `as_bytes().windows(2)`, attenzione agli indici (il vettore binario è lungo n - 1).
pub fn ilya_queries(s: &str, queries: &[(usize, usize)]) -> Vec<usize> {
    todo!()
}

/// E42 — 1D point location statica (il caso semplificato di Frogs and Mosquitoes, L9).
/// `intervals` sono intervalli chiusi [start, end] DISGIUNTI e ordinati per start.
/// Per ogni punto restituisci `Some(indice)` dell'intervallo che lo contiene, `None` se nessuno.
/// Obiettivo: O(log n) per punto con il PREDECESSORE di p tra gli start (ricerca binaria).
/// Rust: `slice::partition_point` (niente BST: gli intervalli qui non cambiano).
pub fn point_location(intervals: &[(i64, i64)], points: &[i64]) -> Vec<Option<usize>> {
    todo!()
}

#[cfg(test)]
mod tests_l9 {
    use super::util::Rng;
    use super::*;

    #[test]
    fn e40_prefix_sums() {
        let a = [2, 4, 1, 7, 3, 0, 4, 2]; // esempio del blog del prof
        let ps = PrefixSums::new(&a);
        let expected = [0, 2, 6, 7, 14, 17, 17, 21, 23];
        for i in 0..=8 {
            assert_eq!(ps.sum(i), expected[i]);
        }
        assert_eq!(ps.range_sum(4, 5), 10);
        assert_eq!(ps.range_sum(1, 8), 23);
        let mut rng = Rng(40);
        for _ in 0..200 {
            let n = 1 + rng.below(30);
            let a: Vec<i64> = rng.vec_i32(n, -20, 20).into_iter().map(i64::from).collect();
            let ps = PrefixSums::new(&a);
            let i = 1 + rng.below(n);
            let j = i + rng.below(n - i + 1);
            assert_eq!(ps.range_sum(i, j), a[i - 1..j].iter().sum::<i64>());
        }
    }

    #[test]
    fn e41_ilya() {
        assert_eq!(ilya_queries("abbabba", &[(2, 6), (1, 7), (3, 5)]), vec![2, 2, 0]);
        // esempi di Codeforces 313B
        assert_eq!(ilya_queries("......", &[(3, 4), (2, 3), (1, 6), (2, 6)]), vec![1, 1, 5, 4]);
        assert_eq!(
            ilya_queries("#..###", &[(1, 3), (5, 6), (1, 5), (3, 6), (3, 4)]),
            vec![1, 1, 2, 2, 0]
        );
        let mut rng = Rng(41);
        for _ in 0..200 {
            let n = 2 + rng.below(30);
            let s: String = (0..n).map(|_| if rng.below(2) == 0 { 'a' } else { 'b' }).collect();
            let b = s.as_bytes();
            let qs: Vec<(usize, usize)> = (0..10)
                .map(|_| {
                    let l = 1 + rng.below(n - 1);
                    (l, l + 1 + rng.below(n - l))
                })
                .collect();
            let brute: Vec<usize> =
                qs.iter().map(|&(l, r)| (l..r).filter(|&k| b[k - 1] == b[k]).count()).collect();
            assert_eq!(ilya_queries(&s, &qs), brute, "{s} {qs:?}");
        }
    }

    #[test]
    fn e42_point_location() {
        let iv = [(1, 3), (5, 5), (8, 12)];
        assert_eq!(
            point_location(&iv, &[0, 1, 3, 4, 5, 6, 8, 12, 13]),
            vec![None, Some(0), Some(0), None, Some(1), None, Some(2), Some(2), None]
        );
        assert_eq!(point_location(&[], &[7]), vec![None]);
        let mut rng = Rng(42);
        for _ in 0..300 {
            let mut iv = Vec::new();
            let mut x = rng.below(5) as i64;
            for _ in 0..rng.below(8) {
                let len = rng.below(4) as i64;
                iv.push((x, x + len));
                x += len + 1 + rng.below(4) as i64;
            }
            let pts: Vec<i64> = (0..15).map(|_| rng.below(40) as i64 - 2).collect();
            let brute: Vec<Option<usize>> =
                pts.iter().map(|&p| iv.iter().position(|&(s, e)| s <= p && p <= e)).collect();
            assert_eq!(point_location(&iv, &pts), brute, "{iv:?} {pts:?}");
        }
    }
}
