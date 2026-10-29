//! Soluzioni di practice_l13 (E53-E55), scritte da Claude (non dal prof).
//! Per provarle: copia questo file al posto di src/lib.rs (salvando prima il tuo!) e lancia i test.

#![allow(dead_code)]

#[cfg(test)]
mod util;

struct Fenwick {
    t: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { t: vec![0; n + 1] }
    }
    fn add(&mut self, mut i: usize, v: i64) {
        while i < self.t.len() {
            self.t[i] += v;
            i += i & i.wrapping_neg();
        }
    }
    fn sum(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.t[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

pub struct UpdateArray {
    v: Fenwick,
    n: usize,
}

impl UpdateArray {
    pub fn new(n: usize) -> Self {
        UpdateArray { v: Fenwick::new(n), n }
    }
    pub fn update(&mut self, i: usize, j: usize, v: i64) {
        self.v.add(i, v);
        if j < self.n {
            self.v.add(j + 1, -v); // cancella la propagazione dopo j
        }
    }
    pub fn access(&self, i: usize) -> i64 {
        self.v.sum(i)
    }
}

pub struct RangeSum {
    ft1: UpdateArray,
    err: UpdateArray,
    n: usize,
}

impl RangeSum {
    pub fn new(n: usize) -> Self {
        RangeSum { ft1: UpdateArray::new(n), err: UpdateArray::new(n), n }
    }
    pub fn range_update(&mut self, i: usize, j: usize, v: i64) {
        self.ft1.update(i, j, v);
        self.err.update(i, j, v * (i as i64 - 1)); // dentro: i*v conta i-1 celle di troppo
        if j < self.n {
            self.err.update(j + 1, self.n, -v * (j - i + 1) as i64); // dopo: manca tutto il range
        }
    }
    pub fn sum(&self, i: usize) -> i64 {
        if i == 0 { 0 } else { i as i64 * self.ft1.access(i) - self.err.access(i) }
    }
    pub fn access(&self, i: usize) -> i64 {
        self.sum(i) - self.sum(i - 1)
    }
}

pub struct SegTree {
    n: usize,
    t: Vec<i64>,
}

impl SegTree {
    pub fn new(a: &[i64]) -> Self {
        let mut st = SegTree { n: a.len(), t: vec![0; 4 * a.len().max(1)] };
        if !a.is_empty() {
            st.build(1, 0, a.len() - 1, a);
        }
        st
    }
    fn build(&mut self, k: usize, lo: usize, hi: usize, a: &[i64]) {
        if lo == hi {
            self.t[k] = a[lo];
            return;
        }
        let m = (lo + hi) / 2;
        self.build(2 * k, lo, m, a);
        self.build(2 * k + 1, m + 1, hi, a);
        self.t[k] = self.t[2 * k] + self.t[2 * k + 1];
    }
    pub fn add(&mut self, i: usize, v: i64) {
        let (mut k, mut lo, mut hi) = (1, 0, self.n - 1);
        loop {
            self.t[k] += v;
            if lo == hi {
                return;
            }
            let m = (lo + hi) / 2;
            if i <= m {
                k *= 2;
                hi = m;
            } else {
                k = 2 * k + 1;
                lo = m + 1;
            }
        }
    }
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.query(1, 0, self.n - 1, l, r)
    }
    fn query(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize) -> i64 {
        if hi < l || r < lo {
            return 0;
        }
        if l <= lo && hi <= r {
            return self.t[k];
        }
        let m = (lo + hi) / 2;
        self.query(2 * k, lo, m, l, r) + self.query(2 * k + 1, m + 1, hi, l, r)
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
