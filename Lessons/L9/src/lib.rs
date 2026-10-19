// L9 (mercoledì 14/10/2026) — Frogs and Mosquitoes (Codeforces 609F, problema OBBLIGATORIO), spiegato a lezione
// (registrazione L09.MP4, prima ora), + prefix sums statiche (modulo `prefix_sums`, seconda parte della lezione).
// frogs_and_mosquitoes: codice scritto da Claude, NON del prof (il prof non ha dato codice per questo problema);
// segue la soluzione della lezione (issue #1, #2, #3). Test: cargo test -p l9

pub mod prefix_sums;

use std::collections::BTreeMap;

/// Frogs and Mosquitoes (lezione del 14/10 e note 2025 p.10-13, Codeforces 609F).
/// `frogs[i] = (x_i, t_i)`: posizione e lunghezza della lingua (le x sono distinte).
/// `mosquitoes[j] = (b_j, a_j)`: posizione e valore, nell'ordine in cui atterrano.
/// Una rana i mangia b se x_i <= b <= x_i + t_i; se più rane possono, mangia la più a sinistra;
/// dopo aver mangiato, t_i += a e la rana può mangiare zanzare atterrate prima e non ancora mangiate.
/// Restituisce per ogni rana (nell'ordine di input) (zanzare mangiate, lunghezza finale della lingua).
///
/// Soluzione delle note, Θ((n + m) log(n + m)):
/// - "dynamic 1D point location": BST delle rane per x; la rana che mangia b è il PREDECESSORE di b,
///   se arriva fino a b;
/// - issue #1: intervalli sovrapposti -> preprocessing che toglie le sovrapposizioni (ogni rana
///   copre da max(x_i, fine della precedente + 1) a x_i + t_i; le rane coperte del tutto spariscono);
/// - issue #2: quando una lingua cresce si creano nuove sovrapposizioni -> si tolgono le rane
///   successive coperte del tutto e si aggiorna l'inizio della prima coperta in parte
///   (ammortizzato: ogni rana viene tolta al più una volta);
/// - issue #3: zanzare non mangiate -> BST delle zanzare per posizione; dopo che una rana cresce,
///   mangia (in ordine di posizione) quelle che ora sono nel suo intervallo.
pub fn frogs_and_mosquitoes(frogs: &[(i64, i64)], mosquitoes: &[(i64, i64)]) -> Vec<(usize, i64)> {
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
    fn frogs_codeforces_examples() {
        let frogs = [(10, 2), (15, 0), (6, 1), (0, 1)];
        let mosq = [(110, 10), (1, 1), (6, 0), (15, 10), (14, 100), (12, 2)];
        assert_eq!(frogs_and_mosquitoes(&frogs, &mosq), vec![(3, 114), (1, 10), (1, 1), (1, 2)]);
        assert_eq!(frogs_and_mosquitoes(&[(10, 2)], &[(20, 2), (12, 1)]), vec![(1, 3)]);
    }

    #[test]
    fn frogs_against_brute_force() {
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
