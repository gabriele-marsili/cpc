// L3 (23/09/2026) — Sliding Window Maxima (deque), Trapping Rain Water, 100 prisoners
//
// Ogni file in src/bin/ diventa un binario: cargo run -p l3 --bin <nome_file>
// Test:                                   cargo test -p l3
//
// Il modulo `swm` contiene il codice del prof. Rossano, copiato dalla sua nota
// https://pages.di.unipi.it/rossano/blog/2023/swm/  (solo commenti aggiunti).
// Tutto il resto NON è codice del prof (a lezione non ha scritto codice):
//   - extra::linear_lecture: la deque come descritta a lezione (coppie <e,p>, rimuove <= y)
//   - trw: Trapping Rain Water, idee discusse a lezione implementate da me
//   - extra: next larger element, simulazione dei 100 prigionieri

pub mod swm {
    use std::collections::{BTreeSet, BinaryHeap, VecDeque};

    // ---------------------------------------------------------------------
    // 1) Brute force — Θ(n·k)
    // `windows(k)` itera su tutte le finestre contigue di lunghezza k.
    // ---------------------------------------------------------------------
    pub fn brute_force(v: &Vec<i32>, k: usize) -> Vec<i32> {
        v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
    }

    // ---------------------------------------------------------------------
    // 2) BST (BTreeSet come multiset) — Θ(n log k)
    // Coppie (valore, posizione): la posizione rende distinti i duplicati.
    // max_sf evita di chiamare .last() a ogni passo: serve solo quando
    // esce dalla finestra proprio il massimo corrente.
    // ---------------------------------------------------------------------
    pub fn bst(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let n = nums.len();
        if k > n {
            return Vec::<i32>::new();
        }

        let mut maxs = Vec::with_capacity(n - k + 1);
        let mut set = BTreeSet::new();
        let mut max_sf = nums[0];

        for (i, &v) in nums.iter().enumerate() {
            set.insert((v, i));
            max_sf = max_sf.max(v);

            if i >= k {
                set.remove(&(nums[i - k], i - k));
                if max_sf == nums[i - k] {
                    max_sf = set.last().unwrap().0;
                }
            }
            if i >= k - 1 {
                maxs.push(max_sf);
            }
        }
        maxs
    }

    // ---------------------------------------------------------------------
    // 3) Max-heap con lazy delete — Θ(n log n)
    // Non si cancella mai un elemento arbitrario: si estrae il massimo
    // finché la sua posizione è fuori dalla finestra.
    // #inserimenti = n, #estrazioni <= n  =>  Θ(n log n)
    // ---------------------------------------------------------------------
    pub fn heap(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let n = nums.len();
        if k > n {
            return Vec::<i32>::new();
        }

        let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

        for i in 0..k - 1 {
            heap.push((nums[i], i));
        }

        let mut maxs = Vec::with_capacity(n - k + 1);

        for i in k - 1..n {
            heap.push((nums[i], i));
            while let Some((_, idx)) = heap.peek() {
                if *idx < i - (k - 1) {
                    heap.pop();
                } else {
                    break;
                }
            }
            maxs.push(heap.peek().unwrap().0);
        }
        maxs
    }

    // ---------------------------------------------------------------------
    // 4) Deque — Θ(n)
    // Q contiene POSIZIONI, con valori in ordine decrescente
    // (= i right leaders della finestra corrente). Il massimo è in testa.
    // ---------------------------------------------------------------------
    pub fn linear(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let n = nums.len();
        if k > n {
            return Vec::<i32>::new();
        }

        let mut q: VecDeque<usize> = VecDeque::new();
        let mut maxs: Vec<i32> = Vec::with_capacity(n - k + 1);

        // prima finestra: solo il passo "togli i dominati dalla coda"
        for i in 0..k {
            while (!q.is_empty()) && nums[i] > nums[*q.back().unwrap()] {
                q.pop_back();
            }
            q.push_back(i);
        }
        maxs.push(nums[*q.front().unwrap()]);

        for i in k..n {
            // (1) togli dalla testa le posizioni uscite dalla finestra
            while !q.is_empty() && q.front().unwrap() + k <= i {
                q.pop_front();
            }
            // (2) togli dalla coda gli elementi dominati da nums[i]
            while (!q.is_empty()) && nums[i] > nums[*q.back().unwrap()] {
                q.pop_back();
            }
            // (3) inserisci il nuovo elemento, (4) la testa è il massimo
            q.push_back(i);
            maxs.push(nums[*q.front().unwrap()]);
        }
        maxs
    }
}

pub mod extra {
    use std::collections::VecDeque;

    // ---------------------------------------------------------------------
    // SWM con la deque, versione "lezione" (pseudocodice alla lavagna)
    // Q contiene coppie <e, p> = (valore, posizione). Dalla coda si rimuovono
    // gli elementi <= y  (il codice della nota usa <, cioè tiene gli uguali).
    // Come a lezione, la finestra "parte prima dell'array": out[i] è il max
    // di A[max(0,i-k+1) ..= i], quindi le prime k-1 risposte sono su finestre parziali.
    // ---------------------------------------------------------------------
    pub fn linear_lecture(a: &[i32], k: usize) -> Vec<i32> {
        let mut q: VecDeque<(i32, usize)> = VecDeque::new();
        let mut out = Vec::with_capacity(a.len());
        for (p, &y) in a.iter().enumerate() {
            // head: via tutto ciò che è fuori dalla finestra, stop al primo dentro
            while let Some(&(_, ph)) = q.front() {
                if ph + k <= p { q.pop_front(); } else { break; }
            }
            // tail: via gli elementi <= y, stop al primo > y
            while let Some(&(e, _)) = q.back() {
                if e <= y { q.pop_back(); } else { break; }
            }
            q.push_back((y, p));
            out.push(q.front().unwrap().0); // la testa è il max
        }
        out
    }

    // ---------------------------------------------------------------------
    // Next Larger Element — Θ(n)   [NON del prof: esercizio della nota]
    // Per ogni i, il primo elemento a destra strettamente maggiore di v[i]
    // (None se non esiste). Stack di posizioni "in attesa" della risposta:
    // i valori nello stack sono non crescenti, come nella deque della SWM.
    // ---------------------------------------------------------------------
    pub fn next_larger(v: &[i32]) -> Vec<Option<i32>> {
        let mut ans = vec![None; v.len()];
        let mut stack: Vec<usize> = Vec::new();
        for (i, &x) in v.iter().enumerate() {
            while let Some(&top) = stack.last() {
                if v[top] < x {
                    ans[top] = Some(x);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        ans
    }

    // ---------------------------------------------------------------------
    // 100 prigionieri — strategia "segui il ciclo"   [NON del prof]
    // drawers[d] = numero nel cassetto d (permutazione di 0..n).
    // Il prigioniero p apre p, poi drawers[p], ... al massimo n/2 cassetti.
    // ---------------------------------------------------------------------
    pub fn prisoner_finds(drawers: &[usize], p: usize) -> bool {
        let mut d = p;
        for _ in 0..drawers.len() / 2 {
            if drawers[d] == p {
                return true;
            }
            d = drawers[d];
        }
        false
    }

    pub fn all_succeed(drawers: &[usize]) -> bool {
        (0..drawers.len()).all(|p| prisoner_finds(drawers, p))
    }

    /// Lunghezza del ciclo più lungo della permutazione.
    /// all_succeed(d) <=> longest_cycle(d) <= n/2
    pub fn longest_cycle(drawers: &[usize]) -> usize {
        let mut seen = vec![false; drawers.len()];
        let mut best = 0;
        for s in 0..drawers.len() {
            let mut len = 0;
            let mut d = s;
            while !seen[d] {
                seen[d] = true;
                d = drawers[d];
                len += 1;
            }
            best = best.max(len);
        }
        best
    }

    /// Probabilità esatta di successo: 1 - Σ_{l=n/2+1}^{n} 1/l  (n pari)
    pub fn exact_success_probability(n: usize) -> f64 {
        1.0 - ((n / 2 + 1)..=n).map(|l| 1.0 / l as f64).sum::<f64>()
    }

    // ---------------------------------------------------------------------
    // È un albero binario un BST?   [pseudocodice dalle note del prof
    // (SlidingWindowMaxima.pdf p.8), implementazione NON del prof]
    // P(v) -> (è BST, min, max) del sottoalbero; NULL -> (true, +inf, -inf).
    // Convenzione delle note: sinistra <= chiave < destra.
    // ---------------------------------------------------------------------
    pub struct Node {
        pub key: i64,
        pub left: Option<Box<Node>>,
        pub right: Option<Box<Node>>,
    }

    impl Node {
        pub fn leaf(key: i64) -> Option<Box<Node>> {
            Some(Box::new(Node { key, left: None, right: None }))
        }
        pub fn new(key: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
            Some(Box::new(Node { key, left, right }))
        }
    }

    fn p(v: &Option<Box<Node>>) -> (bool, i64, i64) {
        match v {
            None => (true, i64::MAX, i64::MIN),
            Some(n) => {
                let (rl, ml, big_ml) = p(&n.left);
                let (rr, mr, big_mr) = p(&n.right);
                let ok = rl && rr && big_ml <= n.key && n.key < mr;
                (ok, n.key.min(ml).min(mr), n.key.max(big_ml).max(big_mr))
            }
        }
    }

    pub fn is_bst(root: &Option<Box<Node>>) -> bool {
        p(root).0
    }

    /// Controllo SBAGLIATO (solo figli diretti), per il confronto nei test
    pub fn is_bst_local_only(root: &Option<Box<Node>>) -> bool {
        match root {
            None => true,
            Some(n) => {
                n.left.as_ref().map_or(true, |l| l.key <= n.key)
                    && n.right.as_ref().map_or(true, |r| n.key < r.key)
                    && is_bst_local_only(&n.left)
                    && is_bst_local_only(&n.right)
            }
        }
    }

    /// Generatore pseudo-casuale minimale (xorshift64*), per non dipendere da crate esterni.
    pub struct Rng(pub u64);
    impl Rng {
        pub fn next_u64(&mut self) -> u64 {
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            self.0.wrapping_mul(0x2545F4914F6CDD1D)
        }
        pub fn below(&mut self, n: usize) -> usize {
            (self.next_u64() % n as u64) as usize
        }
        /// Permutazione casuale di 0..n (Fisher–Yates)
        pub fn permutation(&mut self, n: usize) -> Vec<usize> {
            let mut p: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                let j = self.below(i + 1);
                p.swap(i, j);
            }
            p
        }
    }
}

pub mod trw {
    // ---------------------------------------------------------------------
    // Trapping Rain Water   [idee dalla lezione, implementazioni NON del prof]
    // Acqua sopra la posizione i = min(max a sinistra, max a destra) - H[i]
    // (usando massimi che includono i stesso, il valore è sempre >= 0)
    // ---------------------------------------------------------------------

    /// Θ(n^2): per ogni i ricalcola max a sinistra e a destra
    pub fn brute_force(h: &[u64]) -> u64 {
        (0..h.len())
            .map(|i| {
                let l = *h[..=i].iter().max().unwrap();
                let r = *h[i..].iter().max().unwrap();
                l.min(r) - h[i]
            })
            .sum()
    }

    /// Soluzione del prof: precalcola MR (max a destra) in un array, poi
    /// scansione da sinistra tenendo il max visto finora. Θ(n) tempo, Θ(n) spazio extra.
    pub fn prefix_max(h: &[u64]) -> u64 {
        let n = h.len();
        let mut mr = vec![0; n];
        let mut m = 0;
        for i in (0..n).rev() {
            m = m.max(h[i]);
            mr[i] = m;
        }
        let mut ml = 0;
        let mut water = 0;
        for i in 0..n {
            ml = ml.max(h[i]);
            water += ml.min(mr[i]) - h[i];
        }
        water
    }

    /// Due puntatori (idea proposta in aula): Θ(n) tempo, O(1) spazio extra.
    /// Si avanza sempre il lato col massimo più basso: per quella posizione il
    /// min(max sx, max dx) è già noto, perché dall'altra parte c'è qualcosa di più alto.
    pub fn two_pointers(h: &[u64]) -> u64 {
        if h.is_empty() {
            return 0;
        }
        let (mut l, mut r) = (0, h.len() - 1);
        let (mut ml, mut mr) = (0, 0);
        let mut water = 0;
        while l <= r {
            ml = ml.max(h[l]);
            mr = mr.max(h[r]);
            if ml <= mr {
                water += ml - h[l];
                l += 1;
            } else {
                water += mr - h[r];
                if r == 0 { break; }
                r -= 1;
            }
        }
        water
    }
}

#[cfg(test)]
mod tests {
    use super::extra::*;
    use super::swm::*;
    use super::trw;

    // esempio usato negli appunti di L2 e L3
    fn example() -> (Vec<i32>, usize) {
        (vec![1, 2, 3, 1, 4, 5, 2, 3, 1], 3)
    }

    #[test]
    fn swm_running_example() {
        let (v, k) = example();
        let expected = vec![3, 3, 4, 5, 5, 5, 3];
        assert_eq!(brute_force(&v, k), expected);
        assert_eq!(bst(&v, k), expected);
        assert_eq!(heap(&v, k), expected);
        assert_eq!(linear(&v, k), expected);
    }

    #[test]
    fn swm_lecture_example() {
        // esempio usato a lezione (A modificato alla lavagna), k = 3
        let a = vec![1, 3, 2, 1, 1, 2, 4, 3, 2];
        let full = vec![3, 3, 2, 2, 4, 4, 4];
        assert_eq!(linear(&a, 3), full);
        // versione lezione: prime 2 risposte su finestre parziali (1, 3)
        let lec = linear_lecture(&a, 3);
        assert_eq!(&lec[..6], &[1, 3, 3, 3, 2, 2]); // quanto fatto alla lavagna
        assert_eq!(&lec[2..], &full[..]);
    }

    #[test]
    fn is_bst_counterexample_from_notes() {
        // 10 con figlio sinistro 5, che ha figlio destro 12: i controlli locali passano
        let t = Node::new(10, Node::new(5, None, Node::leaf(12)), None);
        assert!(is_bst_local_only(&t));
        assert!(!is_bst(&t));
        // albero delle note (p.6): 8 / 5 10 / 3 6 9 12 / 1 7
        let good = Node::new(8,
            Node::new(5, Node::new(3, Node::leaf(1), None), Node::new(6, None, Node::leaf(7))),
            Node::new(10, Node::leaf(9), Node::leaf(12)));
        assert!(is_bst(&good));
        assert!(is_bst(&None));
    }

    #[test]
    fn trw_example() {
        let h = [6, 2, 0, 4, 0, 1, 0, 5, 0, 3];
        assert_eq!(trw::brute_force(&h), 26);
        assert_eq!(trw::prefix_max(&h), 26);
        assert_eq!(trw::two_pointers(&h), 26);
    }

    #[test]
    fn trw_random_against_brute_force() {
        let mut rng = Rng(99);
        for _ in 0..1000 {
            let n = rng.below(40);
            let h: Vec<u64> = (0..n).map(|_| rng.below(8) as u64).collect();
            let b = trw::brute_force(&h);
            assert_eq!(trw::prefix_max(&h), b, "{h:?}");
            assert_eq!(trw::two_pointers(&h), b, "{h:?}");
        }
    }

    #[test]
    fn swm_edge_cases() {
        let v = vec![5, 5, 5, 1];
        for k in 1..=4 {
            let b = brute_force(&v, k);
            assert_eq!(bst(&v, k), b, "bst k={k}");
            assert_eq!(heap(&v, k), b, "heap k={k}");
            assert_eq!(linear(&v, k), b, "linear k={k}");
        }
        assert!(linear(&v, 5).is_empty()); // k > n
    }

    #[test]
    fn swm_random_against_brute_force() {
        let mut rng = Rng(0x1234_5678);
        for _ in 0..500 {
            let n = 1 + rng.below(60);
            let v: Vec<i32> = (0..n).map(|_| rng.below(10) as i32 - 5).collect();
            let k = 1 + rng.below(n);
            let b = brute_force(&v, k);
            assert_eq!(bst(&v, k), b);
            assert_eq!(heap(&v, k), b);
            assert_eq!(linear(&v, k), b);
            assert_eq!(&linear_lecture(&v, k)[k - 1..], &b[..]);
        }
    }

    #[test]
    fn next_larger_example() {
        assert_eq!(
            next_larger(&[2, 1, 4, 3, 3, 5]),
            vec![Some(4), Some(4), Some(5), Some(5), Some(5), None]
        );
    }

    #[test]
    fn prisoners_success_iff_no_long_cycle() {
        let mut rng = Rng(42);
        for _ in 0..2000 {
            let p = rng.permutation(100);
            assert_eq!(all_succeed(&p), longest_cycle(&p) <= 50);
        }
    }

    #[test]
    fn prisoners_exact_probability() {
        let p = exact_success_probability(100);
        assert!((p - 0.3118).abs() < 1e-4);
    }
}
