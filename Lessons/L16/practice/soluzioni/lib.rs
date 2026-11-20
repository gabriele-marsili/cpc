//! L16 — E65-E68: static RMQ (sparse table, blocchi, msb) e la rappresentazione di un albero in O(n) bit.
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l16. Posizioni 0-based, estremi inclusi. Niente crate esterni.

#![allow(unused_variables, unused_mut, dead_code)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E65 — Sparse table GENERICA per un'operazione IDEMPOTENTE (f(x, x) = x: min, max, gcd, and, or):
/// è l'unica proprietà che serve per combinare due intervalli che si SOVRAPPONGONO.
/// `new` Θ(n log n), `query(i, j)` O(1) = f su A[i..=j].
/// Rust: generics `T: Copy`, `F: Fn(T, T) -> T` come campo; attento all'underflow di usize in `j - (1 << k) + 1`.
pub struct SparseTable<T, F> {
    t: Vec<Vec<T>>,
    f: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SparseTable<T, F> {
    pub fn new(a: &[T], f: F) -> Self {
        let n = a.len();
        let mut t = vec![a.to_vec()];
        let mut k = 1;
        while (1usize << k) <= n {
            let h = 1usize << (k - 1);
            let prev = &t[k - 1];
            let row: Vec<T> = (0..=n - (1 << k)).map(|i| f(prev[i], prev[i + h])).collect();
            t.push(row);
            k += 1;
        }
        SparseTable { t, f }
    }
    pub fn query(&self, i: usize, j: usize) -> T {
        let k = (usize::BITS - 1 - (j - i + 1).leading_zeros()) as usize;
        (self.f)(self.t[k][i], self.t[k][j + 1 - (1 << k)])
    }
}

/// E66 — msb con la tabella del prof: MSB8[b] = posizione del bit più significativo del byte b
/// (MSB8[0] = 0, non usato). Riempila A TEMPO DI COMPILAZIONE (`const` + `while`: niente `for` nelle const).
/// `msb(x)` per x > 0 guarda i byte dal più significativo: al più 8 accessi alla tabella, niente leading_zeros.
pub const MSB8: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut b = 2;
    while b < 256 {
        t[b] = t[b >> 1] + 1;
        b += 1;
    }
    t
};

pub fn msb(x: u64) -> u32 {
    let mut byte = 8;
    while byte > 0 {
        byte -= 1;
        let v = (x >> (8 * byte)) & 0xFF;
        if v != 0 {
            return 8 * byte + MSB8[v as usize] as u32;
        }
    }
    panic!("msb(0) non è definito")
}

/// E67 — Solution #3 della lezione: blocchi di b ≈ log2 n elementi, A'[t] = minimo del blocco t, sparse table
/// (di POSIZIONI) su A'. `rmq(i, j)` = posizione del minimo più a sinistra di A[i..=j] in Θ(log n):
/// prefisso e suffisso scanditi, parte centrale allineata ai blocchi in O(1). Spazio Θ(n): `stored_words`
/// = numero di usize memorizzati (minimi dei blocchi + sparse table su A'), deve essere <= 3n.
/// Attenzione ai casi "i e j nello stesso blocco" e "blocchi adiacenti" (parte centrale vuota).
pub struct BlockRmq<'a> {
    a: &'a [i64],
    b: usize,
    mins: Vec<usize>,
    st: Vec<Vec<usize>>, // st[k][t] = indice (in A') del minimo di A'[t .. t + 2^k - 1]
}

impl<'a> BlockRmq<'a> {
    pub fn new(a: &'a [i64]) -> Self {
        let n = a.len();
        let b = (usize::BITS - n.max(2).leading_zeros()) as usize;
        let mins: Vec<usize> = (0..n)
            .step_by(b)
            .map(|s| (s..(s + b).min(n)).fold(s, |m, p| if a[p] < a[m] { p } else { m }))
            .collect();
        let m = mins.len();
        let better = |x: usize, y: usize| if a[mins[x]] <= a[mins[y]] { x } else { y };
        let mut st = vec![(0..m).collect::<Vec<usize>>()];
        let mut k = 1;
        while (1usize << k) <= m {
            let h = 1usize << (k - 1);
            let row = (0..=m - (1 << k)).map(|t| better(st[k - 1][t], st[k - 1][t + h])).collect();
            st.push(row);
            k += 1;
        }
        BlockRmq { a, b, mins, st }
    }

    fn scan(&self, i: usize, j: usize) -> usize {
        (i..=j).fold(i, |m, p| if self.a[p] < self.a[m] { p } else { m })
    }

    pub fn rmq(&self, i: usize, j: usize) -> usize {
        let (bi, bj) = (i / self.b, j / self.b);
        if bi == bj {
            return self.scan(i, j);
        }
        let mut best = self.scan(i, (bi + 1) * self.b - 1);
        if bi + 1 < bj {
            let (l, r) = (bi + 1, bj - 1);
            let k = (usize::BITS - 1 - (r - l + 1).leading_zeros()) as usize;
            let (x, y) = (self.st[k][l], self.st[k][r + 1 - (1 << k)]);
            let t = if self.a[self.mins[x]] <= self.a[self.mins[y]] { x } else { y };
            if self.a[self.mins[t]] < self.a[best] {
                best = self.mins[t];
            }
        }
        let suf = self.scan(bj * self.b, j);
        if self.a[suf] < self.a[best] { suf } else { best }
    }

    pub fn stored_words(&self) -> usize {
        self.mins.len() + self.st.iter().map(|r| r.len()).sum::<usize>()
    }
}

/// E68 — la DOMANDA DEL PROF alla fine della registrazione: rappresentazione livello per livello (in ordine BFS)
/// con il grado di ogni nodo in UNARIO: d figli = d volte `1` seguito da uno `0` (convenzione di questo esercizio).
/// `t[v]` = figli di v in ordine; i nodi sono numerati in ordine BFS (radice 0).
/// `louds_encode`: la sequenza di bit; `louds_decode`: l'inverso; `louds_bits(n)`: quanti bit per n nodi?
/// (rispondi prima a mano, poi scrivi la formula). Rust: `VecDeque` o un semplice contatore "prossimo id".
pub fn louds_encode(t: &[Vec<usize>]) -> Vec<bool> {
    let mut out = Vec::new();
    let mut queue = std::collections::VecDeque::from([0usize]);
    while let Some(v) = queue.pop_front() {
        for &c in &t[v] {
            out.push(true);
            queue.push_back(c);
        }
        out.push(false);
    }
    out
}

pub fn louds_decode(bits: &[bool]) -> Vec<Vec<usize>> {
    let mut t: Vec<Vec<usize>> = vec![Vec::new()];
    let mut v = 0; // nodo di cui stiamo leggendo il grado (ordine BFS)
    for &b in bits {
        if b {
            let c = t.len();
            t.push(Vec::new());
            t[v].push(c);
        } else {
            v += 1;
        }
    }
    t
}

pub fn louds_bits(n: usize) -> usize {
    // n zeri (uno per nodo) + n - 1 uni (uno per arco)
    2 * n - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Rng;

    const A: [i64; 8] = [5, 2, 7, 3, 8, 1, 6, 4]; // l'esempio della lezione

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    fn argmin(a: &[i64], i: usize, j: usize) -> usize {
        (i..=j).fold(i, |b, p| if a[p] < a[b] { p } else { b })
    }

    // ---------- E65 ----------
    #[test]
    fn e65_lecture_example() {
        let st = SparseTable::new(&A, |x: i64, y: i64| x.min(y));
        assert_eq!(st.query(0, 6), 1);
        assert_eq!(st.query(0, 3), 2);
        assert_eq!(st.query(7, 7), 4);
    }

    #[test]
    fn e65_min_max_gcd_against_brute_force() {
        let mut r = Rng(65);
        for _ in 0..200 {
            let n = 1 + r.below(70);
            let a = r.vec_i64(n, -9, 10);
            let g: Vec<u64> = (0..n).map(|_| 1 + r.below(60) as u64).collect();
            let smin = SparseTable::new(&a, |x: i64, y: i64| x.min(y));
            let smax = SparseTable::new(&a, |x: i64, y: i64| x.max(y));
            let sgcd = SparseTable::new(&g, gcd);
            for i in 0..n {
                for j in i..n {
                    assert_eq!(smin.query(i, j), *a[i..=j].iter().min().unwrap());
                    assert_eq!(smax.query(i, j), *a[i..=j].iter().max().unwrap());
                    assert_eq!(sgcd.query(i, j), g[i..=j].iter().fold(0, |x, &y| gcd(x, y)));
                }
            }
        }
    }

    #[test]
    fn e65_ranges_starting_at_zero_with_power_of_two_length() {
        // il caso che fa andare in underflow j - (1 << k) + 1 con usize
        let a: Vec<i64> = (0..64).rev().collect();
        let st = SparseTable::new(&a, |x: i64, y: i64| x.min(y));
        for len in [1usize, 2, 4, 8, 16, 32, 64] {
            assert_eq!(st.query(0, len - 1), a[len - 1]);
        }
    }

    // ---------- E66 ----------
    #[test]
    fn e66_table_and_msb() {
        assert_eq!(MSB8[1], 0);
        assert_eq!(MSB8[2], 1);
        assert_eq!(MSB8[255], 7);
        for x in (1u64..100_000).chain([u64::MAX, 1 << 63, (1 << 40) + 3, 7, 15]) {
            assert_eq!(msb(x), 63 - x.leading_zeros(), "x={x}");
        }
        assert_eq!(msb(7), 2); // RMQ(0, 6): lunghezza 7 -> k = 2
    }

    // ---------- E67 ----------
    #[test]
    fn e67_lecture_example() {
        let q = BlockRmq::new(&A);
        assert_eq!(q.rmq(0, 6), 5);
        assert_eq!(q.rmq(0, 3), 1);
    }

    #[test]
    fn e67_against_brute_force() {
        let mut r = Rng(67);
        for _ in 0..200 {
            let n = 1 + r.below(100);
            let a = r.vec_i64(n, -5, 6);
            let q = BlockRmq::new(&a);
            for i in 0..n {
                for j in i..n {
                    assert_eq!(q.rmq(i, j), argmin(&a, i, j), "a={a:?} i={i} j={j}");
                }
            }
        }
    }

    #[test]
    fn e67_linear_space() {
        for &n in &[1usize << 10, 1 << 14, 1 << 18] {
            let a: Vec<i64> = (0..n as i64).collect();
            let q = BlockRmq::new(&a);
            assert!(q.stored_words() <= 3 * n, "n={n}: {} parole", q.stored_words());
        }
    }

    // ---------- E68 ----------
    fn random_tree(r: &mut Rng, n: usize) -> Vec<Vec<usize>> {
        // nodi numerati in ordine BFS: il padre di v è un nodo già visto, i figli in ordine crescente
        let mut ch = vec![Vec::new(); n];
        let mut parent = vec![0usize; n];
        for v in 1..n {
            let lo = if v >= 2 { parent[v - 1] } else { 0 };
            parent[v] = lo + r.below(v - lo);
            ch[parent[v]].push(v);
        }
        ch
    }

    #[test]
    fn e68_small() {
        // radice con 2 figli; il primo figlio ha 1 figlio
        let t = vec![vec![1, 2], vec![3], vec![], vec![]];
        let bits = louds_encode(&t);
        let s: String = bits.iter().map(|&b| if b { '1' } else { '0' }).collect();
        assert_eq!(s, "1101000");
        assert_eq!(louds_decode(&bits), t);
    }

    #[test]
    fn e68_round_trip_and_bits() {
        let mut r = Rng(68);
        for _ in 0..300 {
            let n = 1 + r.below(50);
            let t = random_tree(&mut r, n);
            let bits = louds_encode(&t);
            assert_eq!(bits.len(), louds_bits(n));
            assert_eq!(louds_decode(&bits), t);
        }
    }
}
