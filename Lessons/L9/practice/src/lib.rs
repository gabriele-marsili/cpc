//! L9 (PROVVISORIA) — Frogs and Mosquitoes: non ancora spiegato a lezione, dalle note 2025 del prof (ex E30 di L7).
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
