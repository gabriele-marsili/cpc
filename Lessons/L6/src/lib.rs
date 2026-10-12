// L6 — PROVVISORIO (lezione del 05/10 non ancora integrata dalla registrazione). Codice per gli
// argomenti che in L5 il prof ha annunciato per la lezione dopo (dalle note 2025, Pearls_2025.pdf
// p.14-17, in Lessons/L5): maggioranza con inserimenti e cancellazioni, Misra-Gries.
// Codice generato da Claude, NON del prof. Test: cargo test -p l6

/// Note 2025 p.14: maggioranza con inserimenti e cancellazioni. Per ogni bit si tiene quanti
/// elementi hanno quel bit a 1; il bit i dell'elemento di maggioranza è il valore più frequente
/// in posizione i. O(log u) tempo per operazione e O(log u) spazio (u = valore massimo).
/// Il risultato è significativo solo se una maggioranza esiste.
pub struct BitMajority {
    ones: Vec<usize>,
    len: usize,
}

impl BitMajority {
    pub fn new(bits: usize) -> Self {
        BitMajority { ones: vec![0; bits], len: 0 }
    }
    pub fn insert(&mut self, x: u64) {
        self.len += 1;
        for (b, c) in self.ones.iter_mut().enumerate() {
            *c += ((x >> b) & 1) as usize;
        }
    }
    pub fn delete(&mut self, x: u64) {
        self.len -= 1;
        for (b, c) in self.ones.iter_mut().enumerate() {
            *c -= ((x >> b) & 1) as usize;
        }
    }
    pub fn candidate(&self) -> u64 {
        self.ones.iter().enumerate()
            .filter(|&(_, &c)| c > self.len - c)
            .map(|(b, _)| 1u64 << b)
            .sum()
    }
}

/// Note 2025 p.16-17: Misra-Gries heavy hitters. Gli elementi con almeno n/T + 1 occorrenze
/// sono di sicuro tra i candidati restituiti (ma possono esserci falsi positivi: per averli
/// esatti serve una seconda passata, `heavy_hitters`) [extra].
/// Come nelle note: si aggiunge il nuovo elemento; se i candidati diventano più di T, si
/// decrementano tutti e si eliminano gli zeri. Θ(n) e non O(T·n): ogni decremento "paga"
/// un incremento precedente, e gli incrementi sono n.
pub fn misra_gries(a: &[i64], t: usize) -> std::collections::HashMap<i64, usize> {
    let mut k: std::collections::HashMap<i64, usize> = std::collections::HashMap::new();
    for &e in a {
        *k.entry(e).or_insert(0) += 1;
        if k.len() > t {
            k.retain(|_, c| {
                *c -= 1;
                *c > 0
            });
        }
    }
    k
}

/// Misra-Gries + verifica: gli elementi con almeno n/T + 1 occorrenze, ordinati.
pub fn heavy_hitters(a: &[i64], t: usize) -> Vec<i64> {
    let mut out: Vec<i64> = misra_gries(a, t)
        .into_keys()
        .filter(|c| a.iter().filter(|&&x| x == *c).count() >= a.len() / t + 1)
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rng(u64);
    impl Rng {
        fn below(&mut self, n: usize) -> usize {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 % n as u64) as usize
        }
    }

    const BM: [i64; 9] = [5, 3, 1, 1, 2, 3, 1, 1, 1];

    #[test]
    fn bit_majority_example_from_notes() {
        let mut m = BitMajority::new(3);
        for &x in &BM {
            m.insert(x as u64);
        }
        assert_eq!(m.candidate(), 1);
        for _ in 0..5 {
            m.delete(1); // D D D D D
        }
        m.insert(3); // I  -> rimane 5 3 2 3 3
        assert_eq!(m.candidate(), 3);
    }

    #[test]
    fn misra_gries_against_brute_force() {
        let mut rng = Rng(99);
        for _ in 0..2000 {
            let n = 1 + rng.below(40);
            let t = 1 + rng.below(4);
            let a: Vec<i64> = (0..n).map(|_| rng.below(6) as i64).collect();
            let mut brute: Vec<i64> = (0..6)
                .filter(|&v| a.iter().filter(|&&x| x == v).count() >= n / t + 1)
                .collect();
            brute.sort();
            assert_eq!(heavy_hitters(&a, t), brute, "{a:?} t={t}");
        }
    }
}

// =====================================================================
// Binary search (note del prof: blog 2023 "Applications of binary search" e
// Binary_search_2025.pdf p.2-4). Il codice del blog è copiato con solo commenti; dove
// usava il crate `num` è adattato a u64 (segnalato).
// =====================================================================

/// Codice del prof (blog): ricerca binaria classica su un array ordinato.
/// `middle = low + (high - low)/2` evita l'overflow di `(low + high)/2`. Θ(log n).
pub fn binary_search<T: Ord>(arr: &[T], key: T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let middle = low + (high - low) / 2;

        match key.cmp(&arr[middle]) {
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Less => high = middle,
            std::cmp::Ordering::Greater => low = middle + 1,
        }
    }
    None
}

/// Codice del prof (blog): posizione della PRIMA occorrenza di `key` (dopo un match si
/// continua a cercare a sinistra con high = middle).
pub fn binary_search_first<T: Ord>(arr: &[T], key: T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len(); // note that high is excluded

    let mut ans = None;

    while low < high {
        let middle = low + (high - low) / 2;

        match key.cmp(&arr[middle]) {
            std::cmp::Ordering::Equal => {
                ans = Some(middle);
                high = middle
            }
            std::cmp::Ordering::Less => high = middle,
            std::cmp::Ordering::Greater => low = middle + 1,
        }
    }

    ans
}

/// Codice del prof (blog), ADATTATO a u64 (l'originale è generico con il crate `num`).
/// Binary search for the answer: `pred` è monotono (true ... true false ... false su [low, high));
/// restituisce l'ULTIMO valore per cui `pred` è vero. Θ(log(high - low)) valutazioni di pred.
pub fn binary_search_range<F: Fn(u64) -> bool>(low: u64, high: u64, pred: F) -> Option<u64> {
    let mut low = low;
    let mut high = high;

    let mut ans = None;

    while low < high {
        let middle = low + (high - low) / 2;

        match pred(middle) {
            true => {
                low = middle + 1;
                ans = Some(middle)
            }
            false => high = middle,
        }
    }

    ans
}

/// Codice del prof (blog): radice quadrata intera come one-liner con binary_search_range.
/// [extra] `x * x` può andare in overflow per v vicino a u64::MAX: qui uso checked_mul.
pub fn sqrt(v: u64) -> u64 {
    binary_search_range(0, v + 1, |x| x.checked_mul(x).is_some_and(|q| q <= v)).unwrap()
}

/// Social distancing (blog + note 2025 p.2-3): n intervalli disgiunti [a, b] di interi,
/// piazzare c persone su punti interi in modo che la distanza minima sia la più grande possibile.
/// Predicato monotono "si riescono a piazzare c punti a distanza >= d", valutato greedy in
/// Θ(n + c); binary search su d in [1, l]: Θ((n + c) log l) (+ ordinamento).
/// Codice del prof dal blog, con due modifiche [extra]:
/// 1. stop anticipato appena cnt == c (così ogni valutazione è Θ(n + c) come nelle note);
/// 2. BUG del blog corretto: la ricerca era su d in [1, l] con l = numero totale di punti degli
///    intervalli, ma con spazi vuoti tra gli intervalli la risposta può essere più grande
///    (es. [(1,1),(4,5),(9,12),(14,16)], c = 2: l = 10, ma la risposta è 15 = 16 - 1).
///    Il limite giusto è l'ampiezza (ultimo estremo - primo estremo), come nelle note 2025
///    (d* in [0, ℓ/c] con ℓ = lunghezza della retta).
pub fn select_intervals(intervals: &mut Vec<(usize, usize)>, c: usize) -> Option<usize> {
    let l = intervals
        .iter()
        .fold(0, |acc, interval| acc + interval.1 - interval.0 + 1); // overall length

    if l < c {
        // there is no solution
        return None;
    }

    intervals.sort_unstable();

    // A closure implements our predicate
    let pred = |d: u64| -> bool {
        let d = d as usize;
        let mut last_selected = intervals[0].0;
        let mut cnt = 1;
        for &interval in intervals.iter() {
            while cnt < c && interval.0.max(last_selected + d) <= interval.1 {
                last_selected = interval.0.max(last_selected + d);
                cnt += 1;
            }
        }

        cnt >= c
    };

    let span = intervals.iter().map(|iv| iv.1).max().unwrap() - intervals[0].0; // [extra] fix
    // blog: binary_search_range(1, l + 1, pred)
    binary_search_range(1, span as u64 + 2, pred).map(|d| d as usize)
}

/// Exponential (binary) search (note 2025 p.3) [implementazione NON del prof]: risposta t non
/// limitata a priori. Fase 1: prova le potenze di 2 finché pred diventa falso; fase 2: ricerca
/// binaria nell'ultimo intervallo [2^i, 2^{i+1}). Θ(log t) valutazioni.
/// Qui pred è monotono true ... true false ...; restituisce l'ultimo valore vero (pred(0) deve
/// essere vero).
pub fn exponential_search<F: Fn(u64) -> bool>(pred: F) -> u64 {
    let mut hi = 1u64;
    while pred(hi) {
        hi *= 2;
    }
    binary_search_range(hi / 2, hi, &pred).unwrap_or(hi / 2)
}

#[cfg(test)]
mod tests_binary_search {
    use super::*;

    #[test]
    fn classic_and_first_occurrence() {
        let a = [1, 3, 3, 3, 5, 8, 8, 13];
        assert_eq!(binary_search(&a, 5), Some(4));
        assert!(matches!(binary_search(&a, 3), Some(1..=3)));
        assert_eq!(binary_search(&a, 4), None);
        assert_eq!(binary_search_first(&a, 3), Some(1));
        assert_eq!(binary_search_first(&a, 8), Some(5));
        assert_eq!(binary_search_first(&a, 0), None);
        assert_eq!(binary_search_first::<i32>(&[], 0), None);
    }

    #[test]
    fn sqrt_against_brute_force() {
        for v in 0..2000u64 {
            let r = sqrt(v);
            assert!(r * r <= v && (r + 1) * (r + 1) > v, "v={v}");
        }
        assert_eq!(sqrt(u64::MAX - 1), 4294967295); // con u64::MAX il `v + 1` del blog va in overflow
    }

    fn social_brute(iv: &[(usize, usize)], c: usize) -> Option<usize> {
        let pts: Vec<usize> = iv.iter().flat_map(|&(a, b)| a..=b).collect();
        if pts.len() < c {
            return None;
        }
        // il miglior d tale che il greedy (sempre ottimo per un d fissato) piazza c punti
        (1..=pts.len() + 100)
            .filter(|&d| {
                let mut last = pts[0];
                let mut cnt = 1;
                for &p in &pts[1..] {
                    if p >= last + d {
                        last = p;
                        cnt += 1;
                    }
                }
                cnt >= c
            })
            .max()
    }

    #[test]
    fn social_distancing() {
        // 5 persone, intervalli [0,2] [4,7] [9,9] [12,15]
        let mut iv = vec![(9, 9), (0, 2), (12, 15), (4, 7)];
        let d = select_intervals(&mut iv, 5).unwrap();
        assert_eq!(Some(d), social_brute(&iv, 5));
        assert_eq!(select_intervals(&mut vec![(0, 1)], 3), None);
        // confronto con forza bruta su istanze casuali
        let mut s = 7u64;
        let mut rnd = |n: usize| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            (s % n as u64) as usize
        };
        for _ in 0..500 {
            let mut iv = Vec::new();
            let mut x = rnd(3);
            for _ in 0..1 + rnd(5) {
                let len = rnd(4);
                iv.push((x, x + len));
                x += len + 1 + rnd(4);
            }
            let c = 2 + rnd(7); // con c = 1 la distanza minima non è definita
            let expected = social_brute(&iv, c);
            assert_eq!(select_intervals(&mut iv.clone(), c), expected, "{iv:?} c={c}");
        }
    }

    #[test]
    fn exponential_search_finds_threshold() {
        for t in [0u64, 1, 2, 3, 7, 8, 1000, 123_456_789] {
            assert_eq!(exponential_search(|x| x <= t), t);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Aggiunto il 05/10 (dopo la lezione, dagli appunti di Octech; registrazione non ancora disponibile).

/// Problema dell'auto e del carburante (presentato a lezione come motivazione della
/// "binary search for the answer"; enunciato dagli appunti di Octech, note 2025 p.4).
/// Punti p_1..p_n lungo la strada, p_n è il traguardo; `d[i]` = km tra il punto precedente
/// (la partenza per i = 0) e p_{i+1}; `fuel_per_km` = F; in p_{i+1} si possono rifornire
/// `refill[i]` litri. Restituisce G* = carburante minimo nel serbatoio alla partenza.
/// [extra] Soluzione NON del prof (il prof non ha ancora dato la sua, o non è negli appunti):
/// il predicato "con G alla partenza arrivo al traguardo" è monotono in G e si verifica con una
/// scansione Θ(n) => binary search sulla risposta in [0, F·Σd]: Θ(n log(F·Σd)).
/// Ipotesi: serbatoio senza capacità massima, si rifornisce all'arrivo in ogni punto.
pub fn min_initial_fuel(d: &[u64], refill: &[u64], fuel_per_km: u64) -> u64 {
    assert_eq!(d.len(), refill.len());
    let reaches = |g: u64| -> bool {
        let mut tank = g as i128;
        for (&di, &fi) in d.iter().zip(refill) {
            tank -= (di * fuel_per_km) as i128;
            if tank < 0 {
                return false;
            }
            tank += fi as i128;
        }
        true
    };
    let hi: u64 = d.iter().sum::<u64>() * fuel_per_km; // con G = F·Σd si arriva sempre
    // il predicato qui è false ... false true ... true: cerco l'ultimo G con !reaches(G)
    match binary_search_range(0, hi + 1, |g| !reaches(g)) {
        None => 0,        // reaches(0): non serve carburante
        Some(g) => g + 1, // primo G che funziona
    }
}

/// [extra] La stessa risposta in Θ(n) senza binary search: G* = max_i (F·(d_1+..+d_i) −
/// (f_1+..+f_{i-1})), cioè il massimo "debito" accumulato prima di ogni rifornimento.
pub fn min_initial_fuel_linear(d: &[u64], refill: &[u64], fuel_per_km: u64) -> u64 {
    let (mut need, mut best) = (0i128, 0i128);
    for (&di, &fi) in d.iter().zip(refill) {
        need += (di * fuel_per_km) as i128;
        best = best.max(need);
        need -= fi as i128;
    }
    best as u64
}

#[cfg(test)]
mod tests_fuel {
    use super::*;

    fn brute(d: &[u64], refill: &[u64], f: u64) -> u64 {
        (0..)
            .find(|&g: &u64| {
                let mut tank = g as i64;
                d.iter().zip(refill).all(|(&di, &fi)| {
                    tank -= (di * f) as i64;
                    let ok = tank >= 0;
                    tank += fi as i64;
                    ok
                })
            })
            .unwrap()
    }

    #[test]
    fn small_examples() {
        // 3 tratti da 10 km, 1 l/km, rifornimenti 5 e 30 litri: serve 10 per il primo tratto,
        // poi 10 - 10 + 5 = 5 < 10 => servono 15 litri alla partenza.
        assert_eq!(min_initial_fuel(&[10, 10, 10], &[5, 30, 0], 1), 15);
        assert_eq!(min_initial_fuel_linear(&[10, 10, 10], &[5, 30, 0], 1), 15);
        assert_eq!(min_initial_fuel(&[], &[], 3), 0);
        assert_eq!(min_initial_fuel(&[0, 0], &[1, 1], 2), 0);
    }

    #[test]
    fn against_brute_force() {
        let mut s = 12345u64;
        let mut rnd = |n: u64| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s % n
        };
        for _ in 0..1000 {
            let n = rnd(8) as usize;
            let f = 1 + rnd(3);
            let d: Vec<u64> = (0..n).map(|_| rnd(10)).collect();
            let r: Vec<u64> = (0..n).map(|_| rnd(25)).collect();
            let e = brute(&d, &r, f);
            assert_eq!(min_initial_fuel(&d, &r, f), e, "{d:?} {r:?} f={f}");
            assert_eq!(min_initial_fuel_linear(&d, &r, f), e, "{d:?} {r:?} f={f}");
        }
    }
}
