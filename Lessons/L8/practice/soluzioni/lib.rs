//! SOLUZIONE (scritta da Claude, non dal prof). L8 (12/10) — puzzle della scacchiera (Gomory), Alice & Bob, closest pair of points (sweep line).
//!
//! SOLUZIONI: copia questo file su src/lib.rs per provarle (dopo aver salvato il tuo lavoro).
//! Test: cargo test -p practice_l8   (un esercizio: cargo test -p practice_l8 e37)

#![allow(unused_variables)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E35 — Domino sulla scacchiera n x m (m pari) senza due celle a e b di colore DIVERSO (il puzzle).
/// Restituisci i domino (coppie di celle adiacenti) che coprono tutte le altre celle; None se a e b
/// hanno lo stesso colore. Idea della lezione: un ciclo che passa per tutte le celle; togliendo a e b
/// si spezza in due catene di lunghezza pari. Prima scrivi il ciclo, poi spezzalo.
/// Rust: `Vec<(usize, usize)>`, `iter().position`, indici modulo len.
pub fn domino_tiling(n: usize, m: usize, a: (usize, usize), b: (usize, usize)) -> Option<Vec<((usize, usize), (usize, usize))>> {
    if (a.0 + a.1) % 2 == (b.0 + b.1) % 2 {
        return None;
    }
    let mut cyc: Vec<(usize, usize)> = (0..m).map(|c| (0, c)).collect();
    for (k, c) in (1..m).rev().enumerate() {
        if k % 2 == 0 {
            cyc.extend((1..n).map(|r| (r, c)));
        } else {
            cyc.extend((1..n).rev().map(|r| (r, c)));
        }
    }
    cyc.extend((1..n).rev().map(|r| (r, 0)));
    let ia = cyc.iter().position(|&c| c == a).unwrap();
    let ib = cyc.iter().position(|&c| c == b).unwrap();
    let len = cyc.len();
    let mut res = Vec::new();
    for (from, to) in [(ia, ib), (ib, ia)] {
        let mut i = (from + 1) % len;
        while i != to {
            let j = (i + 1) % len;
            res.push((cyc[i], cyc[j]));
            i = (j + 1) % len;
        }
    }
    Some(res)
}

/// E36 — Alice & Bob, strategia pari/dispari. `alice_move(coins, l, r)`: le monete rimaste sono
/// coins[l..=r] (lunghezza pari, tocca ad Alice); restituisci true se Alice prende a sinistra.
/// Alice decide all'inizio (guardando TUTTE le monete) se prendere le posizioni pari o dispari e
/// poi le prende tutte. Esempio: [10, 2, 100, 1, 5, 6, 3, 4]: dispari 118, pari 13.
pub fn alice_move(coins: &[i64], l: usize, r: usize) -> bool {
    let odd: i64 = coins.iter().step_by(2).sum(); // indici 0-based pari
    let even: i64 = coins.iter().skip(1).step_by(2).sum();
    let take_even_index = odd >= even;
    (l % 2 == 0) == take_even_index
}

/// E37 — Valore ottimo del gioco: (guadagno di chi muove - guadagno dell'altro) su coins, entrambi
/// ottimi. Prima la ricorsione esponenziale (l'albero della lavagna), poi memorizza gli intervalli
/// [l, r] (solo Θ(n^2) diversi): è dynamic programming, che vedremo più avanti.
/// Esempio: [10, 2, 100, 1, 5, 6, 3, 4] -> 105 (Alice 118, Bob 13: qui l'ottimo coincide con la strategia pari/dispari)
pub fn best_difference(coins: &[i64]) -> i64 {
    let n = coins.len();
    if n == 0 {
        return 0;
    }
    // dp[l][r] = miglior differenza su coins[l..=r]
    let mut dp = vec![vec![0i64; n]; n];
    for l in (0..n).rev() {
        dp[l][l] = coins[l];
        for r in l + 1..n {
            dp[l][r] = (coins[l] - dp[l + 1][r]).max(coins[r] - dp[l][r - 1]);
        }
    }
    dp[0][n - 1]
}

/// E38 — Closest pair of points: restituisci il QUADRATO della distanza minima (None se < 2 punti).
/// Obiettivo Θ(n log n): ordina per x; BTreeSet dei punti già visti ordinati per (y, x); per ogni
/// punto guarda solo il range y in [y - δ, y + δ] e CANCELLA i punti con x' < x - δ che incontri.
/// Rust: `BTreeSet::range`, tuple come chiavi, `i64` per i quadrati (|coordinate| <= 1e9).
pub fn closest_pair_dist2(pts: &[(i64, i64)]) -> Option<i64> {
    if pts.len() < 2 {
        return None;
    }
    let mut p = pts.to_vec();
    p.sort();
    let mut best = i64::MAX;
    let mut bst: std::collections::BTreeSet<(i64, i64, usize)> = std::collections::BTreeSet::new();
    for (i, &(x, y)) in p.iter().enumerate() {
        let s = if best == i64::MAX { i64::MAX / 4 } else {
            let mut s = (best as f64).sqrt() as i64;
            while s * s > best { s -= 1 }
            while (s + 1) * (s + 1) <= best { s += 1 }
            s
        };
        let mut dead = Vec::new();
        for &(yy, xx, j) in bst.range((y.saturating_sub(s), i64::MIN, 0)..=(y.saturating_add(s), i64::MAX, usize::MAX)) {
            if x - xx > s {
                dead.push((yy, xx, j));
            } else {
                best = best.min((x - xx) * (x - xx) + (y - yy) * (y - yy));
            }
        }
        for d in dead {
            bst.remove(&d);
        }
        bst.insert((y, x, i));
    }
    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    #[test]
    fn e35_domino_tiling() {
        for (n, m) in [(8, 8), (4, 6), (3, 4), (2, 2)] {
            let cells: Vec<(usize, usize)> = (0..n).flat_map(|r| (0..m).map(move |c| (r, c))).collect();
            for &a in &cells {
                for &b in &cells {
                    if a == b {
                        continue;
                    }
                    let t = domino_tiling(n, m, a, b);
                    if (a.0 + a.1) % 2 == (b.0 + b.1) % 2 {
                        assert!(t.is_none(), "{a:?} {b:?} stesso colore");
                        continue;
                    }
                    let t = t.expect("colori diversi: deve esistere");
                    let mut seen = std::collections::HashSet::new();
                    for &(p, q) in &t {
                        assert_eq!(p.0.abs_diff(q.0) + p.1.abs_diff(q.1), 1, "{p:?} {q:?} non adiacenti");
                        assert!(p.0 < n && p.1 < m && q.0 < n && q.1 < m);
                        assert!(seen.insert(p) && seen.insert(q), "cella coperta due volte");
                    }
                    assert_eq!(seen.len(), n * m - 2);
                    assert!(!seen.contains(&a) && !seen.contains(&b));
                }
            }
        }
    }

    fn play(coins: &[i64], rng: &mut Rng) -> (i64, i64) {
        let (mut l, mut r) = (0usize, coins.len() - 1);
        let (mut a, mut b) = (0, 0);
        loop {
            if alice_move(coins, l, r) { a += coins[l]; l += 1 } else { a += coins[r]; r -= 1 }
            if rng.below(2) == 0 { b += coins[l]; if l == r { break } l += 1 } else { b += coins[r]; if l == r { break } r -= 1 }
        }
        (a, b)
    }

    #[test]
    fn e36_alice_never_loses() {
        let mut rng = Rng(41);
        let mut r2 = Rng(1);
        assert_eq!(play(&[10, 2, 100, 1, 5, 6, 3, 4], &mut r2).0, 118);
        for _ in 0..3000 {
            let n = 2 * (1 + rng.below(6));
            let coins: Vec<i64> = (0..n).map(|_| rng.below(50) as i64).collect();
            let (a, b) = play(&coins, &mut rng);
            assert!(a >= b, "{coins:?}: Alice {a}, Bob {b}");
        }
    }

    #[test]
    fn e37_best_difference() {
        fn brute(c: &[i64]) -> i64 {
            if c.is_empty() { return 0; }
            let n = c.len();
            (c[0] - brute(&c[1..])).max(c[n - 1] - brute(&c[..n - 1]))
        }
        assert_eq!(best_difference(&[]), 0);
        assert_eq!(best_difference(&[10, 2, 100, 1, 5, 6, 3, 4]), 105);
        let mut rng = Rng(43);
        for _ in 0..500 {
            let n = rng.below(12);
            let c: Vec<i64> = (0..n).map(|_| rng.below(30) as i64).collect();
            assert_eq!(best_difference(&c), brute(&c), "{c:?}");
        }
        // deve essere veloce anche su n grande (niente albero esponenziale)
        let big: Vec<i64> = (0..2000).map(|i| (i * 37 % 101) as i64).collect();
        let _ = best_difference(&big);
    }

    #[test]
    fn e38_closest_pair() {
        assert_eq!(closest_pair_dist2(&[(0, 0)]), None);
        assert_eq!(closest_pair_dist2(&[(0, 0), (3, 4), (10, 10), (11, 11)]), Some(2));
        let mut rng = Rng(47);
        for _ in 0..3000 {
            let n = 2 + rng.below(40);
            let range = 1 + rng.below(200);
            let p: Vec<(i64, i64)> = (0..n).map(|_| (rng.below(range) as i64, rng.below(range) as i64)).collect();
            let mut b = i64::MAX;
            for i in 0..n { for j in i + 1..n {
                let (dx, dy) = (p[i].0 - p[j].0, p[i].1 - p[j].1);
                b = b.min(dx * dx + dy * dy);
            } }
            assert_eq!(closest_pair_dist2(&p), Some(b), "{p:?}");
        }
        let line: Vec<(i64, i64)> = (0..20000).map(|k| (5, k * 7)).collect();
        assert_eq!(closest_pair_dist2(&line), Some(49));
    }
}
