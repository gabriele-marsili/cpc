//! L13 — Update the Array, prefix sums con range update, segment tree (E53-E55).
//!
//! Esercizi: sostituisci ogni todo!() con la tua implementazione.
//! Test: cargo test -p practice_l13
//! E53/E54 con posizioni 1-based come alla lavagna; E55 0-based come alla lavagna del segment tree.
//! Il Fenwick tree scrivilo tu (puoi riusare il tuo di E46/E50): niente crate esterni.

#![allow(unused_variables, unused_mut, dead_code)] // le tracce con todo!() non usano i parametri

#[cfg(test)]
mod util;

/// E53 — Update the Array (SPOJ UPDATEIT). Array A[1..=n], inizialmente tutto 0.
/// `update(i, j, v)`: A[k] += v per ogni k in [i, j]; `access(i)`: A[i]. Entrambe Θ(log n).
/// Idea della lezione: NON memorizzare A ma V con A[i] = V[1] + ... + V[i] (prefix sum in un Fenwick tree):
/// update = due Add su V, access = una Sum su V.
pub struct UpdateArray {
    // i tuoi campi
}

impl UpdateArray {
    pub fn new(n: usize) -> Self {
        todo!()
    }
    pub fn update(&mut self, i: usize, j: usize, v: i64) {
        todo!()
    }
    pub fn access(&self, i: usize) -> i64 {
        todo!()
    }
}

/// E54 — Prefix sums dinamiche con range update. Array A[1..=n] inizialmente 0.
/// `range_update(i, j, v)`: A[k] += v per k in [i, j]; `sum(i)` = A[1] + ... + A[i] (sum(0) = 0);
/// `access(i)` = A[i]. Tutto Θ(log n), con la soluzione della lezione:
///   Sum(i) = i * FT1.Access(i) - Err.Access(i)
/// dove FT1 ed Err sono strutture "range update + access" (riusa la logica di E53).
/// Rust: due campi dello stesso tipo, cast `as i64` sugli indici.
pub struct RangeSum {
    // i tuoi campi
}

impl RangeSum {
    pub fn new(n: usize) -> Self {
        todo!()
    }
    pub fn range_update(&mut self, i: usize, j: usize, v: i64) {
        todo!()
    }
    pub fn sum(&self, i: usize) -> i64 {
        todo!()
    }
    pub fn access(&self, i: usize) -> i64 {
        todo!()
    }
}

/// E55 — Segment tree per somme, posizioni 0-based. Ogni nodo contiene la risposta (la somma) del suo range.
/// `new`: costruzione BOTTOM-UP in Θ(n) (anche quando n non è una potenza di 2);
/// `add(i, v)`: A[i] += v in Θ(log n); `range_sum(l, r)`: A[l] + ... + A[r], top-down dalla radice
/// con i tre casi della lezione (disgiunto -> stop; contenuto -> prendi la risposta; altrimenti entrambi i figli).
/// Rust: `Vec<i64>` di dimensione 4n con nodo 1 = radice e figli 2k, 2k+1, funzioni ricorsive private.
pub struct SegTree {
    // i tuoi campi
}

impl SegTree {
    pub fn new(a: &[i64]) -> Self {
        todo!()
    }
    pub fn add(&mut self, i: usize, v: i64) {
        todo!()
    }
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    #[test]
    fn e53_lecture_example() {
        // lavagna (posizioni 1..=9): Update(2,6,10), Update(1,4,5) -> Access(2) = 15
        let mut u = UpdateArray::new(9);
        u.update(2, 6, 10);
        u.update(1, 4, 5);
        let a: Vec<i64> = (1..=9).map(|i| u.access(i)).collect();
        assert_eq!(a, vec![5, 15, 15, 15, 10, 10, 0, 0, 0]);
        let mut u = UpdateArray::new(1);
        u.update(1, 1, -7);
        assert_eq!(u.access(1), -7);
    }

    #[test]
    fn e54_lecture_examples() {
        let mut r = RangeSum::new(10);
        r.range_update(4, 6, 10);
        assert_eq!(r.sum(6), 30);
        assert_eq!(r.sum(10), 30);
        assert_eq!(r.sum(3), 0);
        assert_eq!(r.sum(0), 0);
        // lavagna 40:00-66:00 (n = 8)
        let mut r = RangeSum::new(8);
        r.range_update(2, 7, 10);
        assert_eq!(r.sum(6), 50);
        assert_eq!(r.sum(8), 60);
        r.range_update(3, 4, 5);
        assert_eq!(r.sum(6), 60);
        assert_eq!(r.sum(4), 40);
        assert_eq!(r.access(4), 15);
    }

    #[test]
    fn e53_e54_against_naive() {
        let mut rng = Rng(53);
        for _ in 0..300 {
            let n = 1 + rng.below(30);
            let mut a = vec![0i64; n + 1];
            let mut u = UpdateArray::new(n);
            let mut r = RangeSum::new(n);
            for _ in 0..80 {
                let i = 1 + rng.below(n);
                let j = i + rng.below(n - i + 1);
                match rng.below(3) {
                    0 => {
                        let v = rng.below(41) as i64 - 20;
                        u.update(i, j, v);
                        r.range_update(i, j, v);
                        for x in &mut a[i..=j] {
                            *x += v;
                        }
                    }
                    1 => {
                        assert_eq!(u.access(i), a[i]);
                        assert_eq!(r.access(i), a[i]);
                    }
                    _ => assert_eq!(r.sum(j), a[1..=j].iter().sum::<i64>()),
                }
            }
        }
    }

    #[test]
    fn e55_board_example() {
        let mut st = SegTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        assert_eq!(st.range_sum(0, 7), 27);
        assert_eq!(st.range_sum(2, 5), 21);
        assert_eq!(st.range_sum(4, 4), 6);
        st.add(4, 3);
        assert_eq!(st.range_sum(2, 5), 24);
        assert_eq!(st.range_sum(0, 7), 30);
        let st = SegTree::new(&[42]);
        assert_eq!(st.range_sum(0, 0), 42);
    }

    #[test]
    fn e55_against_naive() {
        let mut rng = Rng(55);
        for _ in 0..300 {
            let n = 1 + rng.below(50); // anche n non potenza di 2
            let mut a: Vec<i64> = rng.vec_i32(n, -50, 50).into_iter().map(i64::from).collect();
            let mut st = SegTree::new(&a);
            for _ in 0..80 {
                let l = rng.below(n);
                let r = l + rng.below(n - l);
                if rng.below(2) == 0 {
                    let v = rng.below(41) as i64 - 20;
                    st.add(l, v);
                    a[l] += v;
                } else {
                    assert_eq!(st.range_sum(l, r), a[l..=r].iter().sum::<i64>(), "n={n} l={l} r={r}");
                }
            }
        }
    }
}
