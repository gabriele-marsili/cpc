// L8 (12/10/2026) — dalla registrazione L_08.MP4.
// Lezione: soluzione del puzzle della scacchiera (catena/ciclo sulla scacchiera), puzzle Alice & Bob
// (strategia pari/dispari, albero di gioco esponenziale -> sottoproblemi quadratici -> DP),
// closest pair of points con sweep line + BST ordinato per y.
// Frogs and Mosquitoes NON ancora spiegato (resta in L7, appendice/E30).
// Codice scritto da Claude seguendo la lezione, NON del prof. Test: cargo test -p l8

use std::collections::{BTreeSet, HashMap};

/// Ciclo che passa per tutte le celle di una scacchiera n x m (n >= 2, m pari), come quello della
/// lavagna: riga 0 da sinistra a destra, poi serpentina sulle colonne m-1..1 (righe 1..n-1),
/// infine risale la colonna 0. Celle consecutive (anche l'ultima e la prima) sono adiacenti.
pub fn board_cycle(n: usize, m: usize) -> Vec<(usize, usize)> {
    assert!(n >= 2 && m >= 2 && m % 2 == 0);
    let mut cyc: Vec<(usize, usize)> = (0..m).map(|c| (0, c)).collect();
    for (k, c) in (1..m).rev().enumerate() {
        if k % 2 == 0 {
            cyc.extend((1..n).map(|r| (r, c))); // giù
        } else {
            cyc.extend((1..n).rev().map(|r| (r, c))); // su
        }
    }
    // m-1 colonne (dispari): l'ultima (colonna 1) è percorsa verso il basso -> siamo in (n-1, 1)
    cyc.extend((1..n).rev().map(|r| (r, 0)));
    cyc
}

/// [extra] Puzzle della lezione (teorema di Gomory): tolte due celle di colore DIVERSO, la scacchiera
/// si copre sempre con domino. Il ciclo si spezza in due catene; ognuna inizia e finisce con colori
/// diversi, quindi ha lunghezza pari e si copre prendendo le celle a coppie consecutive.
/// Restituisce i domino (coppie di celle adiacenti), oppure None se le celle hanno lo stesso colore.
pub fn domino_tiling(
    n: usize,
    m: usize,
    a: (usize, usize),
    b: (usize, usize),
) -> Option<Vec<((usize, usize), (usize, usize))>> {
    if (a.0 + a.1) % 2 == (b.0 + b.1) % 2 {
        return None; // stesso colore: impossibile (argomento di parità di L7)
    }
    let cyc = board_cycle(n, m);
    let ia = cyc.iter().position(|&c| c == a).unwrap();
    let ib = cyc.iter().position(|&c| c == b).unwrap();
    let len = cyc.len();
    let mut dominoes = Vec::with_capacity(len / 2 - 1);
    // due catene: da a+1 a b-1 e da b+1 a a-1 (circolarmente)
    for (from, to) in [(ia, ib), (ib, ia)] {
        let mut i = (from + 1) % len;
        while i != to {
            let j = (i + 1) % len;
            dominoes.push((cyc[i], cyc[j]));
            i = (j + 1) % len;
        }
    }
    Some(dominoes)
}

/// Alice & Bob: 2n monete in fila, a turno si prende una delle due estremità, Alice inizia.
/// Strategia della lezione: Alice confronta la somma delle posizioni dispari (1-based) con quella
/// delle pari e prende sempre tutte le monete della classe migliore: all'inizio le estremità hanno
/// parità diversa; dopo la sua mossa Bob vede solo monete dell'altra classe, e qualunque scelta
/// faccia scopre una moneta della classe di Alice. Restituisce (somma dispari, somma pari).
pub fn parity_sums(coins: &[i64]) -> (i64, i64) {
    let odd = coins.iter().step_by(2).sum(); // posizioni 1, 3, 5, ... (1-based)
    let even = coins.iter().skip(1).step_by(2).sum();
    (odd, even)
}

/// Mossa della strategia pari/dispari: dato l'intervallo rimasto [l, r] (0-based, lunghezza pari)
/// e la classe scelta da Alice (`take_odd` = posizioni 0-based pari = 1-based dispari),
/// restituisce true se Alice deve prendere a sinistra.
pub fn parity_move(l: usize, take_odd: bool) -> bool {
    // con lunghezza pari, l ed r hanno parità diversa: prendo l'estremità della mia classe
    (l % 2 == 0) == take_odd
}

/// Simula una partita: Alice usa la strategia pari/dispari, Bob usa `bob(l, r) -> true se sinistra`.
/// Restituisce (guadagno di Alice, guadagno di Bob).
pub fn play_parity_vs(
    coins: &[i64],
    mut bob: impl FnMut(&[i64], usize, usize) -> bool,
) -> (i64, i64) {
    assert!(coins.len() % 2 == 0);
    let (odd, even) = parity_sums(coins);
    let take_odd = odd >= even;
    let (mut l, mut r) = (0usize, coins.len());
    let (mut a, mut b) = (0, 0);
    while l < r {
        let left = parity_move(l, take_odd);
        if left {
            a += coins[l];
            l += 1
        } else {
            r -= 1;
            a += coins[r]
        }
        let left = bob(coins, l, r - 1);
        if left {
            b += coins[l];
            l += 1
        } else {
            r -= 1;
            b += coins[r]
        }
    }
    (a, b)
}

/// [extra] Miglior risultato possibile per chi muove su coins[l..=r]: (mio guadagno - avversario),
/// entrambi giocano in modo ottimo. È l'albero di gioco della lavagna (2^n foglie) ma i sottoproblemi
/// distinti sono solo i Θ(n^2) intervalli [l, r]: memorizzandoli (dynamic programming, più avanti nel
/// corso) costa Θ(n^2).
pub fn best_difference(coins: &[i64]) -> i64 {
    fn go(c: &[i64], l: usize, r: usize, memo: &mut HashMap<(usize, usize), i64>) -> i64 {
        if l > r {
            return 0;
        }
        if let Some(&v) = memo.get(&(l, r)) {
            return v;
        }
        let take_left = c[l] - if l < r { go(c, l + 1, r, memo) } else { 0 };
        let take_right = c[r] - if l < r { go(c, l, r - 1, memo) } else { 0 };
        let v = take_left.max(take_right);
        memo.insert((l, r), v);
        v
    }
    if coins.is_empty() {
        return 0;
    }
    go(coins, 0, coins.len() - 1, &mut HashMap::new())
}

fn dist2(p: (i64, i64), q: (i64, i64)) -> i64 {
    let (dx, dy) = (p.0 - q.0, p.1 - q.1);
    dx * dx + dy * dy
}

fn isqrt(v: i64) -> i64 {
    let mut s = (v as f64).sqrt() as i64;
    while s * s > v {
        s -= 1
    }
    while (s + 1) * (s + 1) <= v {
        s += 1
    }
    s
}

/// Closest pair of points, sweep line della lezione. Restituisce (i, j, distanza al quadrato).
/// - ordina i punti per x (Θ(n log n)) e li processa in quest'ordine; δ = distanza minima fra i punti
///   già processati (qui δ² intero, niente radici);
/// - i candidati per (x, y) stanno nel rettangolo [x-δ, x] x [y-δ, y+δ]: al più 8 (6) punti, perché
///   sono a distanza >= δ fra loro (8 quadrati di lato δ/2, al più un punto per quadrato);
/// - "oracolo": BST dei punti già processati ORDINATO PER y; range query [y-δ, y+δ];
/// - i punti trovati con x' < x - δ (zona verde della lavagna) non serviranno MAI più (i prossimi punti
///   hanno x più grande e δ può solo diminuire): si cancellano subito. Ogni punto è inserito e
///   cancellato al più una volta => Θ(n log n) in totale.
/// Coordinate con |c| <= 1e9 (i quadrati stanno in i64).
pub fn closest_pair(pts: &[(i64, i64)]) -> Option<(usize, usize, i64)> {
    if pts.len() < 2 {
        return None;
    }
    let mut order: Vec<usize> = (0..pts.len()).collect();
    order.sort_by_key(|&i| (pts[i].0, pts[i].1));
    let mut best: Option<(usize, usize, i64)> = None;
    let mut bst: BTreeSet<(i64, i64, usize)> = BTreeSet::new(); // (y, x, indice)
    for &i in &order {
        let (x, y) = pts[i];
        let d2 = best.map_or(i64::MAX, |b| b.2);
        let s = if d2 == i64::MAX {
            i64::MAX / 4
        } else {
            isqrt(d2)
        };
        let mut to_remove = Vec::new();
        for &(yy, xx, j) in bst
            .range((y.saturating_sub(s), i64::MIN, 0)..=(y.saturating_add(s), i64::MAX, usize::MAX))
        {
            if x - xx > s {
                to_remove.push((yy, xx, j)); // a sinistra della striscia: inutile per sempre
            } else {
                let d = dist2((x, y), (xx, yy));
                if best.is_none_or(|b| d < b.2) {
                    best = Some((j.min(i), j.max(i), d));
                }
            }
        }
        for t in to_remove {
            bst.remove(&t);
        }
        bst.insert((y, x, i));
    }
    best
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
    fn cycle_is_hamiltonian() {
        for (n, m) in [(8, 8), (2, 2), (3, 4), (5, 6), (4, 2)] {
            let c = board_cycle(n, m);
            assert_eq!(c.len(), n * m);
            let set: std::collections::HashSet<_> = c.iter().collect();
            assert_eq!(set.len(), n * m);
            for k in 0..c.len() {
                let (p, q) = (c[k], c[(k + 1) % c.len()]);
                assert_eq!(
                    p.0.abs_diff(q.0) + p.1.abs_diff(q.1),
                    1,
                    "{n}x{m} {p:?} {q:?}"
                );
            }
        }
    }

    #[test]
    fn gomory_tiling_all_pairs() {
        for (n, m) in [(8, 8), (4, 6), (3, 4)] {
            let cells: Vec<(usize, usize)> =
                (0..n).flat_map(|r| (0..m).map(move |c| (r, c))).collect();
            for &a in &cells {
                for &b in &cells {
                    if a == b {
                        continue;
                    }
                    let t = domino_tiling(n, m, a, b);
                    if (a.0 + a.1) % 2 == (b.0 + b.1) % 2 {
                        assert!(t.is_none());
                        continue;
                    }
                    let t = t.unwrap();
                    let mut seen = std::collections::HashSet::new();
                    for &(p, q) in &t {
                        assert_eq!(p.0.abs_diff(q.0) + p.1.abs_diff(q.1), 1);
                        assert!(seen.insert(p) && seen.insert(q));
                    }
                    assert_eq!(seen.len(), n * m - 2);
                    assert!(!seen.contains(&a) && !seen.contains(&b));
                }
            }
        }
    }

    #[test]
    fn alice_bob_example() {
        let coins = [10, 2, 100, 1, 5, 6, 3, 4];
        assert_eq!(parity_sums(&coins), (118, 13)); // dalla lavagna: O = 118, E = 13
        // Bob ingordo (prende l'estremità più grande): Alice prende comunque tutte le dispari
        let (a, b) = play_parity_vs(&coins, |c, l, r| c[l] >= c[r]);
        assert_eq!((a, b), (118, 13));
    }

    #[test]
    fn alice_never_loses() {
        let mut rng = Rng(3);
        for _ in 0..3000 {
            let n = 2 * (1 + rng.below(6) as usize);
            let coins: Vec<i64> = (0..n).map(|_| rng.below(50) as i64).collect();
            let seed = rng.below(1000);
            let mut r2 = Rng(seed + 1);
            let (a, b) = play_parity_vs(&coins, |_, _, _| r2.below(2) == 0);
            assert!(a >= b, "{coins:?}");
            // Bob ottimo (minimizza a - b): Alice vince comunque
            let (a2, b2) = play_parity_vs(&coins, |c, l, r| {
                if l == r {
                    return true;
                }
                let left = c[l] - best_difference(&c[l + 1..=r]);
                let right = c[r] - best_difference(&c[l..r]);
                left >= right
            });
            assert!(a2 >= b2, "{coins:?}");
            // e il valore ottimo per Alice è >= 0
            assert!(best_difference(&coins) >= 0);
        }
    }

    #[test]
    fn best_difference_vs_exhaustive() {
        fn brute(c: &[i64]) -> i64 {
            if c.is_empty() {
                return 0;
            }
            let n = c.len();
            (c[0] - brute(&c[1..])).max(c[n - 1] - brute(&c[..n - 1]))
        }
        let mut rng = Rng(5);
        for _ in 0..500 {
            let n = rng.below(12) as usize;
            let c: Vec<i64> = (0..n).map(|_| rng.below(30) as i64).collect();
            assert_eq!(best_difference(&c), brute(&c));
        }
    }

    #[test]
    fn closest_pair_vs_brute() {
        assert_eq!(closest_pair(&[(0, 0)]), None);
        assert_eq!(
            closest_pair(&[(0, 0), (3, 4), (10, 10), (11, 11)]).map(|r| r.2),
            Some(2)
        );
        let mut rng = Rng(7);
        for _ in 0..3000 {
            let n = 2 + rng.below(40) as usize;
            let range = 1 + rng.below(200);
            let p: Vec<(i64, i64)> = (0..n)
                .map(|_| (rng.below(range) as i64, rng.below(range) as i64))
                .collect();
            let mut b = i64::MAX;
            for i in 0..n {
                for j in i + 1..n {
                    b = b.min(dist2(p[i], p[j]));
                }
            }
            let (i, j, d) = closest_pair(&p).unwrap();
            assert_eq!(d, b, "{p:?}");
            assert_eq!(dist2(p[i], p[j]), d);
        }
    }

    #[test]
    fn closest_pair_large() {
        // stessa x per tutti (caso avverso della lezione: punti larghi in y e stretti in x)
        let p: Vec<(i64, i64)> = (0..2000).map(|k| (5, k * 7)).collect();
        assert_eq!(closest_pair(&p).unwrap().2, 49);
    }
}
