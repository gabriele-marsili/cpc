//! L7 (07/10) — Two pointers trick, binary search for the answer, sweep line (E31-E34 dalla registrazione; E28-E29 dalla versione provvisoria; Frogs and Mosquitoes spostato in L9).
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l7   (un esercizio: cargo test -p practice_l7 e29)

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E28 — Two pointers su array ORDINATO: trova i < j con a[i] + a[j] == target (una coppia
/// qualsiasi), None se non esiste. Obiettivo: Θ(n) tempo, O(1) spazio (niente hash set).
/// Perché è corretto scartare a[i] quando a[i] + a[j] < target?
pub fn pair_with_sum(a: &[i64], target: i64) -> Option<(usize, usize)> {
    todo!()
}

/// E29 — Longest k-good segment (Codeforces 616D, OBBLIGATORIO all'esame): il sottoarray più
/// lungo con al più k valori distinti; restituisci (l, r) 0-based inclusivi, a parità il più a sinistra.
/// Esempio delle note (k = 3): [1, 1, 2, 3, 3, 4, 4, 1, 3, 2, 3] -> (3, 8)  (3 3 4 4 1 3)
/// Obiettivo: Θ(n) operazioni su una mappa valore -> occorrenze nella finestra.
/// Rust: `HashMap::entry`, `get_mut`, `remove` quando il contatore va a 0 (così `len()` = distinti).
pub fn longest_k_good(a: &[u32], k: usize) -> (usize, usize) {
    todo!()
}

/// E31 — Longest k-good con BINARY SEARCH FOR THE ANSWER (l'alternativa proposta in aula):
/// restituisci solo la LUNGHEZZA del più lungo sottoarray con al più k valori distinti.
/// Predicato: "esiste un k-good di lunghezza L?" con una finestra scorrevole di lunghezza fissa.
/// Esempio della lezione (k = 3): [1, 1, 2, 3, 3, 4, 4, 7, 3, 2, 3] -> 6 (in aula è uscito 5: perché?)
/// Obiettivo: Θ(n log n). Rust: `HashMap`, `usize::div_ceil` per il punto medio "alto".
pub fn longest_k_good_len_bs(a: &[u32], k: usize) -> usize {
    todo!()
}

/// E32 — Sweep line: massimo numero di intervalli CHIUSI [s, e] sovrapposti in un punto.
/// Esempio: [(1, 3), (3, 5)] -> 2 (si toccano in 3); [(1, 6), (2, 4), (3, 8), (5, 9)] -> 3.
/// Obiettivo: Θ(n log n) con eventi (coordinata, inizio/fine) ordinati e un contatore.
/// Attenzione all'ordine degli eventi a parità di coordinata.
pub fn max_overlaps(iv: &[(i64, i64)]) -> usize {
    todo!()
}

/// E33 — Profondità massima di una sequenza di parentesi bilanciate (l'altra faccia di E32:
/// '(' = inizio, ')' = fine nell'ordine della sweep line). Ignora gli altri caratteri.
/// Esempio della lavagna: "(((()()))()()()())" -> 4.
pub fn max_depth_parentheses(s: &str) -> usize {
    todo!()
}

/// E34 — Auto e carburante (ripreso a inizio L7): tratti `d[0..=n]` km fra partenza, stazioni
/// 1..n e arrivo; alla stazione i si aggiungono `f[i-1]` litri; consumo `cons` litri/km.
/// Restituisci G*, il minimo carburante iniziale per arrivare.
/// Esempio: d = [3, 4, 2], f = [1, 5], cons = 1 -> 6.
/// Obiettivo: binary search for the answer su G + simulazione Θ(n) per il predicato.
/// (Bonus: c'è anche una soluzione ad hoc in Θ(n): quale?)
pub fn min_initial_fuel(d: &[u64], f: &[u64], cons: u64) -> u64 {
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

    #[test]
    fn e28_pair_with_sum() {
        let a = [1, 3, 4, 6, 8, 11];
        assert_eq!(pair_with_sum(&a, 10), Some((2, 3)));
        assert_eq!(pair_with_sum(&a, 2), None);
        let mut rng = Rng(5);
        for _ in 0..1000 {
            let n = rng.below(15) as usize;
            let mut v: Vec<i64> = (0..n).map(|_| rng.below(20) as i64).collect();
            v.sort();
            let t = rng.below(40) as i64;
            let brute = (0..n).any(|i| (i + 1..n).any(|j| v[i] + v[j] == t));
            let got = pair_with_sum(&v, t);
            assert_eq!(got.is_some(), brute, "{v:?} {t}");
            if let Some((i, j)) = got {
                assert!(i < j && v[i] + v[j] == t);
            }
        }
    }

    fn k_good_brute(a: &[u32], k: usize) -> usize {
        let mut best = 0;
        for l in 0..a.len() {
            let mut s = std::collections::HashSet::new();
            for r in l..a.len() {
                s.insert(a[r]);
                if s.len() > k {
                    break;
                }
                best = best.max(r - l + 1);
            }
        }
        best
    }

    #[test]
    fn e29_longest_k_good() {
        // esempio delle note (k = 3): 1 1 2 3 3 4 4 1 3 2 3
        let a = [1, 1, 2, 3, 3, 4, 4, 1, 3, 2, 3];
        let (l, r) = longest_k_good(&a, 3);
        assert_eq!(r - l + 1, 6);
        assert_eq!((l, r), (3, 8)); // 3 3 4 4 1 3
        // esempio Codeforces 616D
        assert_eq!(longest_k_good(&[1, 2, 3, 4, 5], 5), (0, 4));
        assert_eq!(longest_k_good(&[6, 5, 1, 2, 3, 2, 1, 4, 5], 3), (2, 6));
        assert_eq!(longest_k_good(&[1, 2, 3], 1), (0, 0));
        let mut rng = Rng(7);
        for _ in 0..1000 {
            let n = 1 + rng.below(25) as usize;
            let a: Vec<u32> = (0..n).map(|_| rng.below(5) as u32).collect();
            let k = 1 + rng.below(4) as usize;
            let (l, r) = longest_k_good(&a, k);
            assert_eq!(r - l + 1, k_good_brute(&a, k), "{a:?} k={k}");
        }
    }

    fn k_good_brute_len(a: &[u32], k: usize) -> usize {
        let mut best = 0;
        for l in 0..a.len() {
            let mut s = std::collections::HashSet::new();
            for r in l..a.len() {
                s.insert(a[r]);
                if s.len() > k {
                    break;
                }
                best = best.max(r - l + 1);
            }
        }
        best
    }

    #[test]
    fn e31_k_good_binary_search() {
        assert_eq!(longest_k_good_len_bs(&[1, 1, 2, 3, 3, 4, 4, 7, 3, 2, 3], 3), 6);
        assert_eq!(longest_k_good_len_bs(&[1, 2, 3, 4, 5], 5), 5);
        assert_eq!(longest_k_good_len_bs(&[1, 2, 3], 1), 1);
        let mut rng = Rng(23);
        for _ in 0..1000 {
            let n = 1 + rng.below(25) as usize;
            let a: Vec<u32> = (0..n).map(|_| rng.below(5) as u32).collect();
            let k = 1 + rng.below(6) as usize;
            assert_eq!(longest_k_good_len_bs(&a, k), k_good_brute_len(&a, k), "{a:?} k={k}");
        }
    }

    #[test]
    fn e32_max_overlaps() {
        assert_eq!(max_overlaps(&[]), 0);
        assert_eq!(max_overlaps(&[(1, 3), (3, 5)]), 2);
        assert_eq!(max_overlaps(&[(1, 2), (3, 5)]), 1);
        assert_eq!(max_overlaps(&[(1, 6), (2, 4), (3, 8), (5, 9)]), 3);
        let mut rng = Rng(29);
        for _ in 0..2000 {
            let n = rng.below(10) as usize;
            let iv: Vec<(i64, i64)> = (0..n)
                .map(|_| {
                    let s = rng.below(30) as i64;
                    (s, s + rng.below(10) as i64)
                })
                .collect();
            let brute = (0..40).map(|x| iv.iter().filter(|&&(s, e)| s <= x && x <= e).count()).max().unwrap();
            assert_eq!(max_overlaps(&iv), brute, "{iv:?}");
        }
    }

    #[test]
    fn e33_parentheses_depth() {
        assert_eq!(max_depth_parentheses("(((()()))()()()())"), 4);
        assert_eq!(max_depth_parentheses(""), 0);
        assert_eq!(max_depth_parentheses("()()"), 1);
        assert_eq!(max_depth_parentheses("(a(b)c)"), 2);
    }

    #[test]
    fn e34_car_min_initial_fuel() {
        assert_eq!(min_initial_fuel(&[3, 4, 2], &[1, 5], 1), 6);
        assert_eq!(min_initial_fuel(&[0], &[], 7), 0);
        assert_eq!(min_initial_fuel(&[5], &[], 2), 10);
        let mut rng = Rng(31);
        for _ in 0..2000 {
            let n = rng.below(6) as usize;
            let d: Vec<u64> = (0..=n).map(|_| rng.below(10)).collect();
            let f: Vec<u64> = (0..n).map(|_| rng.below(15)).collect();
            let cons = 1 + rng.below(3);
            // brute force: prova ogni G a partire da 0
            let ok = |g: u64| {
                let mut fuel = g as i64;
                for i in 0..=n {
                    fuel -= (cons * d[i]) as i64;
                    if fuel < 0 {
                        return false;
                    }
                    if i < n {
                        fuel += f[i] as i64;
                    }
                }
                true
            };
            let brute = (0..).find(|&g| ok(g)).unwrap();
            assert_eq!(min_initial_fuel(&d, &f, cons), brute, "{d:?} {f:?} {cons}");
        }
    }
}
