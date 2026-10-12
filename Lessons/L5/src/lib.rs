// L5 (30/09/2026) — dalla registrazione. Codice generato da Claude, NON del prof (a lezione non
// ha scritto codice). Test: cargo test -p l5
//
// Duplicato con accesso casuale: la soluzione del prof che sposta ogni elemento nella sua cella
// partendo da n (distrugge A), Floyd (A in sola lettura); elemento di maggioranza (Boyer-Moore).
// Maggioranza con cancellazioni e Misra-Gries non sono stati fatti in L5: sono in Lessons/L6.

/// Soluzione del prof in L5: si parte dall'ultima posizione n, si prende l'elemento e lo si mette
/// nella cella col suo indice, togliendo quello che c'era; si continua con l'elemento tolto.
/// Quando la cella di destinazione contiene già quel valore, è il duplicato.
/// Esempio: 7 va in 7 (esce 4), 4 in 4 (esce 5), 5 in 5 (esce 2), 2 in 2 (esce 5), 5 in 5: c'è già.
/// Θ(n) tempo, O(1) spazio extra, A distrutto.
pub fn place_from_n(a: &mut [usize]) -> usize {
    let n = a.len() - 1;
    let mut x = a[n];
    loop {
        if a[x] == x {
            return x;
        }
        x = std::mem::replace(&mut a[x], x);
    }
}

/// Note 2025 p.9-10: Floyd. Si vede A come lista: i -> A[i].
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


/// [extra] Variante delle note 2025 (p.8): si segue la lista i -> A[i] partendo da n (nessuna cella punta
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

    fn is_dup(a: &[usize], x: usize) -> bool {
        a.iter().filter(|&&y| y == x).count() >= 2
    }

    const LESSON: [usize; 9] = [1, 3, 5, 6, 5, 2, 0, 4, 7];

    #[test]
    fn place_from_n_example_from_lesson() {
        assert_eq!(place_from_n(&mut LESSON.to_vec()), 5);
        let mut rng = Rng(555);
        for _ in 0..3000 {
            let n = 1 + rng.below(40);
            let a: Vec<usize> = (0..=n).map(|_| rng.below(n)).collect();
            assert!(is_dup(&a, place_from_n(&mut a.clone())), "{a:?}");
        }
    }

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
}
