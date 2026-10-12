// L7 (07/10/2026) — sistemata sulla registrazione L_07.MP4.
// Lezione: fine binary search for the answer (social distancing, auto/carburante), puzzle della
// scacchiera, two pointers trick (longest k-good segment, palindromo, alternativa con binary search
// for the answer), sweep line (massimo numero di intervalli sovrapposti, parentesi bilanciate).
// Frogs and Mosquitoes NON è stato spiegato il 07/10: spostato in Lessons/L9 (crate l9).
// Codice scritto da Claude seguendo la lezione, NON del prof. Test: cargo test -p l7

use std::collections::HashMap;

/// [extra] Esempio classico del two pointers trick (riferimento GeeksforGeeks della pagina del
/// corso): in un array ORDINATO, esiste una coppia i < j con a[i] + a[j] == target?
/// Un puntatore parte da sinistra, uno da destra; ogni passo ne muove uno: Θ(n).
pub fn pair_with_sum(a: &[i64], target: i64) -> Option<(usize, usize)> {
    if a.len() < 2 {
        return None;
    }
    let (mut i, mut j) = (0, a.len() - 1);
    while i < j {
        let s = a[i] + a[j];
        if s == target {
            return Some((i, j));
        } else if s < target {
            i += 1; // con a[i] la somma più grande possibile (con a[j]) è troppo piccola
        } else {
            j -= 1; // con a[j] la somma più piccola possibile (con a[i]) è troppo grande
        }
    }
    None
}

/// Longest k-good segment (note 2025 p.5, Codeforces 616D): il sottoarray più lungo con al più k
/// valori distinti. Restituisce (l, r) 0-based inclusivi; in caso di parità il più a sinistra.
/// Two pointers: r avanza sempre; finché la finestra ha più di k distinti, avanza l.
/// Ogni indice entra ed esce una volta: Θ(n) operazioni sulla mappa (Θ(n) atteso con HashMap,
/// Θ(n log n) con un BST: il "Θ(n (log n)*)" delle note).
pub fn longest_k_good(a: &[u32], k: usize) -> (usize, usize) {
    assert!(!a.is_empty() && k >= 1);
    let mut cnt: HashMap<u32, usize> = HashMap::new();
    let mut l = 0;
    let mut best = (0, 0);
    for r in 0..a.len() {
        *cnt.entry(a[r]).or_insert(0) += 1;
        while cnt.len() > k {
            let c = cnt.get_mut(&a[l]).unwrap();
            *c -= 1;
            if *c == 0 {
                cnt.remove(&a[l]);
            }
            l += 1;
        }
        if r - l > best.1 - best.0 {
            best = (l, r);
        }
    }
    best
}

/// Palindromo con due puntatori (esempio della lezione): uno parte dall'inizio, uno dalla fine.
pub fn is_palindrome(s: &[u8]) -> bool {
    if s.is_empty() {
        return true;
    }
    let (mut i, mut j) = (0, s.len() - 1);
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

/// Predicato della binary search for the answer: esiste un sottoarray k-good di lunghezza `len`?
/// Finestra scorrevole di lunghezza fissa: ogni spostamento toglie un elemento e ne aggiunge uno.
fn exists_k_good_of_len(a: &[u32], k: usize, len: usize) -> bool {
    let mut cnt: HashMap<u32, usize> = HashMap::new();
    for (r, &v) in a.iter().enumerate() {
        *cnt.entry(v).or_insert(0) += 1;
        if r >= len {
            let out = a[r - len];
            let c = cnt.get_mut(&out).unwrap();
            *c -= 1;
            if *c == 0 {
                cnt.remove(&out);
            }
        }
        if r + 1 >= len && cnt.len() <= k {
            return true; // early exit (il prof: "if you really implement it, early exit is better")
        }
    }
    false
}

/// Alternativa proposta da uno studente: binary search for the answer sulla LUNGHEZZA.
/// Predicato monotono: se esiste un k-good di lunghezza L, esiste anche di ogni lunghezza < L.
/// Θ(n log n): più lento dei due puntatori (Θ(n)), ma esempio di applicazione della tecnica.
/// Restituisce la lunghezza del più lungo sottoarray k-good (k >= 1, a non vuoto).
pub fn longest_k_good_len_bs(a: &[u32], k: usize) -> usize {
    assert!(!a.is_empty() && k >= 1);
    // invariante: lo è fattibile, hi+1 non lo è; la risposta è almeno min(k, n) (e almeno 1)
    let (mut lo, mut hi) = (k.min(a.len()), a.len());
    while lo < hi {
        let mid = lo + (hi - lo).div_ceil(2);
        if exists_k_good_of_len(a, k, mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

/// Massimo numero di intervalli sovrapposti, approccio "contatori" della lezione: un contatore per
/// ogni coordinata in [0, l], incrementato per ogni punto di ogni intervallo.
/// Θ(l + somma delle lunghezze): può essere QUADRATICO anche con l = Θ(n) (intervalli lunghi).
pub fn max_overlaps_counters(iv: &[(usize, usize)], l: usize) -> usize {
    let mut c = vec![0usize; l + 1];
    for &(s, e) in iv {
        for cx in &mut c[s..=e] {
            *cx += 1; // un incremento per OGNI punto dell'intervallo: è questo il costo
        }
    }
    c.into_iter().max().unwrap_or(0)
}

/// Sweep line: ogni intervallo chiuso [s, e] diventa due eventi (s, inizio) e (e, fine); si
/// ordinano e si scorrono con un contatore (+1 inizio, -1 fine); la risposta è il massimo.
/// Gli intervalli sono CHIUSI (x in [s_i, e_i] e x in [s_j, e_j]): a parità di coordinata gli inizi
/// vanno prima delle fine, altrimenti [1,3] e [3,5] non risulterebbero sovrapposti.
/// Θ(n log n), dominato dall'ordinamento.
pub fn max_overlaps(iv: &[(i64, i64)]) -> usize {
    let mut ev: Vec<(i64, i32)> = Vec::with_capacity(2 * iv.len());
    for &(s, e) in iv {
        ev.push((s, 0)); // 0 = inizio: viene prima di 1 = fine a parità di coordinata
        ev.push((e, 1));
    }
    ev.sort_unstable();
    let (mut cur, mut best) = (0usize, 0usize);
    for (_, kind) in ev {
        if kind == 0 {
            cur += 1;
            best = best.max(cur);
        } else {
            cur -= 1;
        }
    }
    best
}

/// Stessa cosa vista come parentesi: '(' per ogni inizio, ')' per ogni fine, nell'ordine della
/// sweep line; la risposta è la profondità massima della sequenza bilanciata.
pub fn max_depth_parentheses(s: &str) -> usize {
    let (mut d, mut best) = (0usize, 0usize);
    for ch in s.chars() {
        match ch {
            '(' => {
                d += 1;
                best = best.max(d);
            }
            ')' => d -= 1,
            _ => {}
        }
    }
    best
}

/// Auto e carburante (L6, ripreso a inizio L7). Tratti `d[0..=n]` (km) fra partenza, stazioni
/// 1..n e arrivo; alla stazione i si aggiungono `f[i-1]` litri; consumo `cons` litri/km.
/// G* = minimo carburante iniziale per arrivare. Binary search for the answer su G:
/// predicato "con G arrivo?" monotono (più carburante non fa mai male), verificato simulando
/// il viaggio in Θ(n). Totale Θ(n log(cons · Σd)).
pub fn min_initial_fuel(d: &[u64], f: &[u64], cons: u64) -> u64 {
    assert_eq!(d.len(), f.len() + 1);
    let can = |g: u64| -> bool {
        let mut fuel = g;
        for (i, &di) in d.iter().enumerate() {
            let need = cons * di;
            if fuel < need {
                return false;
            }
            fuel -= need;
            if i < f.len() {
                fuel += f[i];
            }
        }
        true
    };
    let (mut lo, mut hi) = (0u64, cons * d.iter().sum::<u64>()); // hi è sempre sufficiente
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if can(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
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
    fn pair_with_sum_examples() {
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
    fn longest_k_good_examples() {
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

    #[test]
    fn palindrome() {
        assert!(is_palindrome(b"onorarono"));
        assert!(is_palindrome(b"abba"));
        assert!(is_palindrome(b""));
        assert!(!is_palindrome(b"abca"));
    }

    #[test]
    fn k_good_binary_search_for_the_answer() {
        // esempio della lezione (k = 3): il prof conclude 5, ma 3 3 4 4 7 3 è 3-good: la risposta è 6
        let a = [1, 1, 2, 3, 3, 4, 4, 7, 3, 2, 3];
        assert_eq!(longest_k_good_len_bs(&a, 3), 6);
        assert!(exists_k_good_of_len(&a, 3, 6));
        assert!(!exists_k_good_of_len(&a, 3, 7));
        let mut rng = Rng(13);
        for _ in 0..1000 {
            let n = 1 + rng.below(25) as usize;
            let a: Vec<u32> = (0..n).map(|_| rng.below(5) as u32).collect();
            let k = 1 + rng.below(6) as usize;
            assert_eq!(longest_k_good_len_bs(&a, k), k_good_brute(&a, k), "{a:?} k={k}");
            let (l, r) = longest_k_good(&a, k);
            assert_eq!(longest_k_good_len_bs(&a, k), r - l + 1);
        }
    }

    #[test]
    fn sweep_line_max_overlaps() {
        assert_eq!(max_overlaps(&[]), 0);
        assert_eq!(max_overlaps(&[(1, 3), (3, 5)]), 2); // chiusi: si toccano in 3
        assert_eq!(max_overlaps(&[(1, 2), (3, 5)]), 1);
        assert_eq!(max_overlaps(&[(1, 6), (2, 4), (3, 8), (5, 9)]), 3);
        let mut rng = Rng(17);
        for _ in 0..2000 {
            let n = rng.below(10) as usize;
            let iv: Vec<(usize, usize)> = (0..n)
                .map(|_| {
                    let s = rng.below(30) as usize;
                    (s, s + rng.below(10) as usize)
                })
                .collect();
            let iv64: Vec<(i64, i64)> = iv.iter().map(|&(s, e)| (s as i64, e as i64)).collect();
            assert_eq!(max_overlaps(&iv64), max_overlaps_counters(&iv, 40), "{iv:?}");
        }
    }

    #[test]
    fn parentheses_depth() {
        // la sequenza della lavagna: profondità 4
        assert_eq!(max_depth_parentheses("(((()()))()()()())"), 4);
        assert_eq!(max_depth_parentheses(""), 0);
        assert_eq!(max_depth_parentheses("()()"), 1);
    }

    #[test]
    fn car_min_initial_fuel() {
        // 2 stazioni: tratti 3, 4, 2 km, consumo 1 l/km, rifornimenti 1 e 5 litri
        // serve arrivare alla prima (3), poi 3-3+1 = 1 < 4: servono 3 in più -> G* = 6
        assert_eq!(min_initial_fuel(&[3, 4, 2], &[1, 5], 1), 6);
        assert_eq!(min_initial_fuel(&[0], &[], 7), 0);
        let mut rng = Rng(19);
        for _ in 0..2000 {
            let n = rng.below(6) as usize;
            let d: Vec<u64> = (0..=n).map(|_| rng.below(10)).collect();
            let f: Vec<u64> = (0..n).map(|_| rng.below(15)).collect();
            let cons = 1 + rng.below(3);
            // soluzione "ad hoc" (in chiusura): il massimo deficit sui prefissi
            let (mut used, mut got, mut need) = (0u64, 0u64, 0u64);
            for i in 0..=n {
                used += cons * d[i];
                need = need.max(used.saturating_sub(got));
                if i < n {
                    got += f[i];
                }
            }
            assert_eq!(min_initial_fuel(&d, &f, cons), need, "{d:?} {f:?} {cons}");
        }
    }
}
