//! SOLUZIONE (scritta da Claude, non dal prof). L9 — Frogs and Mosquitoes (E39) e prefix sums statiche (E40-E42).
//!
//! SOLUZIONI: copia questo file su src/lib.rs per provarle (dopo aver salvato il tuo lavoro).
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
    use std::collections::BTreeMap;
    let n = frogs.len();
    let mut eaten = vec![0usize; n];
    let mut tongue: Vec<i64> = frogs.iter().map(|f| f.1).collect();

    // BST: inizio effettivo dell'intervallo -> indice della rana
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| frogs[i].0);
    let mut bst: BTreeMap<i64, usize> = BTreeMap::new();
    let mut covered_until = i64::MIN; // fine dell'ultimo intervallo inserito
    for &i in &order {
        let (x, t) = frogs[i];
        if x + t <= covered_until {
            continue; // coperta del tutto da una rana più a sinistra: non mangerà mai
        }
        bst.insert(x.max(covered_until.saturating_add(1)), i);
        covered_until = covered_until.max(x + t);
    }

    // zanzare non mangiate: posizione -> valori (multiset)
    let mut waiting: BTreeMap<i64, Vec<i64>> = BTreeMap::new();

    for &(b, a) in mosquitoes {
        // predecessore di b
        let found = bst.range(..=b).next_back().map(|(&s, &i)| (s, i));
        match found {
            Some((_, i)) if frogs[i].0 + tongue[i] >= b => {
                eaten[i] += 1;
                tongue[i] += a;
                // la rana cresce: mangia le zanzare in attesa ora raggiungibili
                loop {
                    let reach = frogs[i].0 + tongue[i];
                    // inizio effettivo della rana i nel BST: cerco chiave con valore i <= b
                    let start = bst.range(..=b).next_back().map(|(&s, _)| s).unwrap();
                    let next = waiting.range(start..=reach).next().map(|(&p, _)| p);
                    let Some(p) = next else { break };
                    let vals = waiting.get_mut(&p).unwrap();
                    let v = vals.pop().unwrap();
                    if vals.is_empty() {
                        waiting.remove(&p);
                    }
                    eaten[i] += 1;
                    tongue[i] += v;
                }
                // issue #2: togli / accorcia le rane successive coperte dalla lingua cresciuta
                let reach = frogs[i].0 + tongue[i];
                let start = bst.range(..=b).next_back().map(|(&s, _)| s).unwrap();
                while let Some((&s, &j)) = bst.range(start + 1..).next() {
                    if s > reach {
                        break;
                    }
                    bst.remove(&s);
                    if frogs[j].0 + tongue[j] > reach {
                        bst.insert(reach + 1, j); // coperta in parte: nuovo inizio
                        break;
                    }
                }
            }
            _ => waiting.entry(b).or_default().push(a),
        }
    }
    (0..n).map(|i| (eaten[i], tongue[i])).collect()
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
pub struct PrefixSums {
    p: Vec<i64>,
}

impl PrefixSums {
    pub fn new(a: &[i64]) -> Self {
        // p[0] = 0, p[i] = A[1] + ... + A[i]: niente casi speciali per i = 1
        let mut p = Vec::with_capacity(a.len() + 1);
        p.push(0);
        for &x in a {
            p.push(p[p.len() - 1] + x);
        }
        Self { p }
    }
    pub fn sum(&self, i: usize) -> i64 {
        self.p[i]
    }
    pub fn range_sum(&self, i: usize, j: usize) -> i64 {
        // RangeSum(i, j) = Sum(j) - Sum(i - 1)
        self.p[j] - self.p[i - 1]
    }
}

/// E41 — Ilya and Queries (Codeforces 313B, L9). `s` fatta di 'a'/'b' (o qualsiasi byte);
/// per ogni query (l, r) 1-based con l < r restituisci il numero di k con l <= k < r e s[k] == s[k+1].
/// Esempio della lezione: s = "abbabba", (2, 6) -> 2. Obiettivo: Θ(n + m) in totale.
/// Rust: `as_bytes().windows(2)`, attenzione agli indici (il vettore binario è lungo n - 1).
pub fn ilya_queries(s: &str, queries: &[(usize, usize)]) -> Vec<usize> {
    // p[k] = numero di posizioni 1..=k (1-based) con s[k] == s[k+1]; p[0] = 0
    let mut p = vec![0usize];
    for w in s.as_bytes().windows(2) {
        p.push(p[p.len() - 1] + usize::from(w[0] == w[1]));
    }
    // posizioni k in [l, r): p[r - 1] - p[l - 1]
    queries.iter().map(|&(l, r)| p[r - 1] - p[l - 1]).collect()
}

/// E42 — 1D point location statica (il caso semplificato di Frogs and Mosquitoes, L9).
/// `intervals` sono intervalli chiusi [start, end] DISGIUNTI e ordinati per start.
/// Per ogni punto restituisci `Some(indice)` dell'intervallo che lo contiene, `None` se nessuno.
/// Obiettivo: O(log n) per punto con il PREDECESSORE di p tra gli start (ricerca binaria).
/// Rust: `slice::partition_point` (niente BST: gli intervalli qui non cambiano).
pub fn point_location(intervals: &[(i64, i64)], points: &[i64]) -> Vec<Option<usize>> {
    points
        .iter()
        .map(|&p| {
            // numero di intervalli con start <= p: il predecessore è l'ultimo di questi
            let k = intervals.partition_point(|&(s, _)| s <= p);
            if k > 0 && p <= intervals[k - 1].1 { Some(k - 1) } else { None }
        })
        .collect()
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
