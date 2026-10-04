// L4 (28/09/2026) — Duplicate element in an array (prima "pearl")
// + L5 (30/09/2026) PROVVISORIA: registrazione non ancora disponibile, ricostruita dalle
//   note 2025 del prof (Pearls_2025.pdf p.8-17): destroy A, Floyd, majority element,
//   Boyer-Moore, majority con inserimenti/cancellazioni, Misra-Gries.
//
// File generato da Claude: NON è codice del prof (a lezione non ha scritto codice).
// Le soluzioni seguono quelle descritte a lezione; `floyd` viene dalle note 2025
// del prof (Pearls_2025.pdf p.9-10) ed è un'anteprima: in L4 non è stata vista.
// La tua soluzione `my_sol` (lib.rs) non è toccata: qui ci sono solo test in più.
//
// Problema: A[0..=n] contiene n+1 valori in {0, ..., n-1}  =>  c'è almeno un duplicato.
// Trovarne uno qualsiasi.

use std::collections::HashSet;

/// Soluzione #1: hash set. Θ(n) tempo atteso (whp), Θ(n) spazio extra.
pub fn hash_set(a: &[usize]) -> Option<usize> {
    let mut h = HashSet::with_capacity(a.len());
    a.iter().copied().find(|&x| !h.insert(x))
}

/// Soluzione #2: tabella ad accesso diretto / bit vector. Θ(n) deterministico, più veloce
/// dell'hash set, ma ancora Θ(n) bit di spazio extra.
pub fn direct_access(a: &[usize]) -> Option<usize> {
    let mut seen = vec![false; a.len()];
    a.iter().copied().find(|&x| std::mem::replace(&mut seen[x], true))
}

/// Idea "ingenua" di lezione: per ogni bit, prende il valore più frequente su TUTTO l'array.
/// SBAGLIATA: controesempio in aula [3,3,5,6,5,2,0,4,7] -> restituisce 7 (non è un duplicato).
/// Assume n potenza di 2. Tenuta solo per i test.
pub fn bits_majority_naive(a: &[usize]) -> usize {
    let n = a.len() - 1;
    let bits = n.trailing_zeros();
    let mut e = 0;
    for b in (0..bits).rev() {
        let ones = a.iter().filter(|&&x| (x >> b) & 1 == 1).count();
        if ones > a.len() - ones {
            e |= 1 << b;
        }
    }
    e
}

/// Soluzione #3 (lezione): Θ(log n) passate, si ricostruisce e bit per bit dal più
/// significativo, contando solo gli elementi che iniziano col prefisso già trovato.
/// Θ(n log n) tempo, O(1) spazio extra, A in sola lettura e letto solo in sequenza.
///
/// Versione per n qualsiasi (a lezione n era una potenza di 2) [extra]: invece dei bit si
/// dimezza l'intervallo di valori [lo, hi) che contiene di sicuro un duplicato: se in
/// [lo, mid) cadono più di (mid - lo) elementi, lì c'è un duplicato (piccionaia), altrimenti
/// sta in [mid, hi). Con n potenza di 2 è esattamente "contare 0 e 1 sul bit successivo".
pub fn bit_by_bit(a: &[usize]) -> usize {
    let (mut lo, mut hi) = (0usize, a.len() - 1); // valori in [lo, hi)
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        let left = a.iter().filter(|&&x| lo <= x && x < mid).count();
        if left > mid - lo {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    lo
}

/// Anteprima dalle note 2025 [extra]: Floyd. Si vede A come lista: i -> A[i].
/// Nessuna cella punta a n (i valori sono < n), quindi partendo da n si entra in un ciclo
/// ("rho"): l'ingresso del ciclo ha due frecce entranti, cioè è un valore duplicato.
/// Θ(n) tempo, O(1) spazio, A in sola lettura (ma con accesso casuale).
pub fn floyd(a: &[usize]) -> usize {
    let n = a.len() - 1;
    let (mut slow, mut fast) = (a[n], a[a[n]]);
    while slow != fast {
        slow = a[slow];
        fast = a[a[fast]];
    }
    // fase 2: uno riparte dall'inizio, entrambi a velocità 1: si incontrano all'ingresso
    let mut s = n;
    while s != slow {
        s = a[s];
        slow = a[slow];
    }
    s
}

// =====================================================================
// L5 (PROVVISORIA, dalle note 2025) — implementazioni NON del prof
// =====================================================================

/// Note 2025 p.8, "destroy A": si segue la lista i -> A[i] partendo da n (nessuna cella punta
/// a n). Ogni cella visitata viene marcata con il valore n (che non è un valore valido):
/// la prima volta che arriviamo su una cella già marcata, quel numero di cella ha due frecce
/// entranti, cioè è un valore duplicato. Θ(n) tempo, O(1) spazio extra, ma A viene distrutto.
pub fn follow_pointers_destroy(a: &mut [usize]) -> usize {
    let n = a.len() - 1;
    let mut cur = n;
    loop {
        let nxt = a[cur];
        a[cur] = n; // marca come visitata
        if a[nxt] == n {
            return nxt;
        }
        cur = nxt;
    }
}

/// Note 2025 p.13: Boyer-Moore majority vote. Candidato c e contatore.
/// Restituisce il candidato: se un elemento di maggioranza (> n/2 occorrenze) esiste, è c.
/// Θ(n) tempo, O(1) spazio. Se la maggioranza non è garantita serve una seconda passata
/// di verifica (vedi `majority`) [extra].
pub fn boyer_moore_candidate(a: &[i64]) -> Option<i64> {
    // esattamente come nelle note: se A[i] == c +1, altrimenti -1; se il contatore va a 0,
    // A[i] diventa il nuovo candidato con contatore 1
    let (&first, rest) = a.split_first()?;
    let (mut c, mut count) = (first, 1usize);
    for &x in rest {
        if x == c {
            count += 1;
        } else {
            count -= 1;
            if count == 0 {
                c = x;
                count = 1;
            }
        }
    }
    Some(c)
}

/// Candidato di Boyer-Moore + passata di verifica: l'elemento di maggioranza, se esiste.
pub fn majority(a: &[i64]) -> Option<i64> {
    let c = boyer_moore_candidate(a)?;
    (a.iter().filter(|&&x| x == c).count() > a.len() / 2).then_some(c)
}

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
    use crate::my_sol;

    struct Rng(u64);
    impl Rng {
        fn below(&mut self, n: usize) -> usize {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 % n as u64) as usize
        }
    }

    fn is_dup(a: &[usize], x: usize) -> bool {
        a.iter().filter(|&&y| y == x).count() >= 2
    }

    const LESSON: [usize; 9] = [1, 3, 5, 6, 5, 2, 0, 4, 7];

    #[test]
    fn lesson_example_all_solutions() {
        assert_eq!(hash_set(&LESSON), Some(5));
        assert_eq!(direct_access(&LESSON), Some(5));
        assert_eq!(bit_by_bit(&LESSON), 5);
        assert_eq!(floyd(&LESSON), 5);
        assert_eq!(bits_majority_naive(&LESSON), 5); // qui funziona per caso...
    }

    #[test]
    fn naive_bits_counterexample_from_class() {
        // "mettiamo un 3 al primo posto": duplicati {3, 5}, ma l'idea ingenua dà 111 = 7
        let a = [3, 3, 5, 6, 5, 2, 0, 4, 7];
        assert_eq!(bits_majority_naive(&a), 7);
        assert!(!is_dup(&a, 7));
        assert!(is_dup(&a, bit_by_bit(&a))); // restringendosi al prefisso funziona (dà 5)
    }

    #[test]
    fn all_solutions_on_random_arrays() {
        let mut rng = Rng(0x9E3779B97F4A7C15);
        for _ in 0..3000 {
            let n = 1 + rng.below(40);
            let a: Vec<usize> = (0..=n).map(|_| rng.below(n)).collect();
            assert!(is_dup(&a, hash_set(&a).unwrap()), "{a:?}");
            assert!(is_dup(&a, direct_access(&a).unwrap()), "{a:?}");
            assert!(is_dup(&a, bit_by_bit(&a)), "{a:?}");
            assert!(is_dup(&a, floyd(&a)), "{a:?}");
        }
    }

    /// Controllo della TUA soluzione (my_sol) contro la definizione, su input casuali
    #[test]
    fn my_sol_on_random_arrays() {
        let mut rng = Rng(12345);
        for _ in 0..3000 {
            let n = 1 + rng.below(40);
            let orig: Vec<usize> = (0..=n).map(|_| rng.below(n)).collect();
            let d = my_sol(&mut orig.clone()).expect("un duplicato esiste sempre");
            assert!(is_dup(&orig, d), "{orig:?} -> {d}");
        }
    }

    /// my_sol fa al più n scambi in tutto: ognuno mette definitivamente un valore al suo posto
    #[test]
    fn my_sol_is_linear_on_adversarial_input() {
        let n = 200_000;
        let mut a: Vec<usize> = (1..n).chain([0, n - 1]).collect(); // un solo grande ciclo + duplicato
        assert_eq!(a.len(), n + 1);
        assert_eq!(my_sol(&mut a), Some(n - 1));
    }

    // ------------------------- L5 (provvisoria) -------------------------

    #[test]
    fn destroy_a_example_from_notes() {
        let mut a = LESSON.to_vec();
        assert_eq!(follow_pointers_destroy(&mut a), 5); // 8 -> 7 -> 4 -> 5 -> 2 -> 5
    }

    #[test]
    fn destroy_a_and_floyd_on_random_arrays() {
        let mut rng = Rng(777);
        for _ in 0..3000 {
            let n = 1 + rng.below(40);
            let a: Vec<usize> = (0..=n).map(|_| rng.below(n)).collect();
            assert!(is_dup(&a, follow_pointers_destroy(&mut a.clone())), "{a:?}");
            // Floyd e destroy A trovano lo STESSO duplicato: l'ingresso del ciclo raggiunto da n
            assert_eq!(floyd(&a), follow_pointers_destroy(&mut a.clone()), "{a:?}");
        }
    }

    const BM: [i64; 9] = [5, 3, 1, 1, 2, 3, 1, 1, 1];

    #[test]
    fn boyer_moore_example_from_notes() {
        assert_eq!(boyer_moore_candidate(&BM), Some(1)); // contatore finale 3, non la frequenza (5)
        assert_eq!(majority(&BM), Some(1));
        // senza maggioranza il candidato può essere qualsiasi: serve la verifica
        assert_eq!(majority(&[1, 2, 3]), None);
    }

    #[test]
    fn majority_against_brute_force() {
        let mut rng = Rng(4242);
        for _ in 0..3000 {
            let n = rng.below(30);
            let a: Vec<i64> = (0..n).map(|_| rng.below(3) as i64).collect();
            let brute = (0..3).find(|&v| a.iter().filter(|&&x| x == v).count() > n / 2);
            assert_eq!(majority(&a), brute, "{a:?}");
        }
    }

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
