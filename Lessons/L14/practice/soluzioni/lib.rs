//! SOLUZIONI L14 (E56-E58) — scritte da Claude, non dal prof. Copia questo file su src/lib.rs per provarle.
//! Posizioni 0-based; nodo 1 = radice, figli 2k e 2k+1, array di 4n nodi.

#[cfg(test)]
mod util;

// ---------------------------------------------------------------- E56
pub struct MinTree {
    n: usize,
    a: Vec<i64>,
    t: Vec<usize>,
}

impl MinTree {
    pub fn new(a: &[i64]) -> Self {
        let mut st = MinTree { n: a.len(), a: a.to_vec(), t: vec![0; 4 * a.len().max(1)] };
        if !a.is_empty() {
            st.build(1, 0, a.len() - 1);
        }
        st
    }
    fn better(&self, p: usize, q: usize) -> usize {
        if self.a[q] < self.a[p] || (self.a[q] == self.a[p] && q < p) { q } else { p }
    }
    fn build(&mut self, k: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.t[k] = lo;
            return;
        }
        let m = (lo + hi) / 2;
        self.build(2 * k, lo, m);
        self.build(2 * k + 1, m + 1, hi);
        self.t[k] = self.better(self.t[2 * k], self.t[2 * k + 1]);
    }
    pub fn add(&mut self, i: usize, v: i64) {
        self.a[i] += v;
        self.up(1, 0, self.n - 1, i);
    }
    fn up(&mut self, k: usize, lo: usize, hi: usize, i: usize) {
        if lo == hi {
            return;
        }
        let m = (lo + hi) / 2;
        if i <= m { self.up(2 * k, lo, m, i) } else { self.up(2 * k + 1, m + 1, hi, i) }
        self.t[k] = self.better(self.t[2 * k], self.t[2 * k + 1]);
    }
    pub fn rmq(&self, l: usize, r: usize) -> usize {
        self.q(1, 0, self.n - 1, l, r).unwrap()
    }
    fn q(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize) -> Option<usize> {
        if hi < l || r < lo {
            return None; // no overlap
        }
        if l <= lo && hi <= r {
            return Some(self.t[k]); // total overlap
        }
        let m = (lo + hi) / 2; // partial overlap: entrambi i figli
        match (self.q(2 * k, lo, m, l, r), self.q(2 * k + 1, m + 1, hi, l, r)) {
            (Some(p), Some(q)) => Some(self.better(p, q)),
            (p, q) => p.or(q),
        }
    }
}

// ---------------------------------------------------------------- E57
pub struct MinCount {
    n: usize,
    t: Vec<(i64, usize)>,
}

pub fn combine(x: (i64, usize), y: (i64, usize)) -> (i64, usize) {
    if x.0 == y.0 {
        (x.0, x.1 + y.1)
    } else if x.0 < y.0 {
        x
    } else {
        y
    }
}

impl MinCount {
    pub fn new(a: &[i64]) -> Self {
        let mut st = MinCount { n: a.len(), t: vec![(0, 0); 4 * a.len().max(1)] };
        if !a.is_empty() {
            st.build(1, 0, a.len() - 1, a);
        }
        st
    }
    fn build(&mut self, k: usize, lo: usize, hi: usize, a: &[i64]) {
        if lo == hi {
            self.t[k] = (a[lo], 1);
            return;
        }
        let m = (lo + hi) / 2;
        self.build(2 * k, lo, m, a);
        self.build(2 * k + 1, m + 1, hi, a);
        self.t[k] = combine(self.t[2 * k], self.t[2 * k + 1]);
    }
    pub fn add(&mut self, i: usize, v: i64) {
        self.up(1, 0, self.n - 1, i, v);
    }
    fn up(&mut self, k: usize, lo: usize, hi: usize, i: usize, v: i64) {
        if lo == hi {
            self.t[k].0 += v;
            return;
        }
        let m = (lo + hi) / 2;
        if i <= m { self.up(2 * k, lo, m, i, v) } else { self.up(2 * k + 1, m + 1, hi, i, v) }
        self.t[k] = combine(self.t[2 * k], self.t[2 * k + 1]);
    }
    pub fn query(&self, l: usize, r: usize) -> (i64, usize) {
        self.q(1, 0, self.n - 1, l, r).unwrap()
    }
    fn q(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize) -> Option<(i64, usize)> {
        if hi < l || r < lo {
            return None;
        }
        if l <= lo && hi <= r {
            return Some(self.t[k]);
        }
        let m = (lo + hi) / 2;
        match (self.q(2 * k, lo, m, l, r), self.q(2 * k + 1, m + 1, hi, l, r)) {
            (Some(x), Some(y)) => Some(combine(x, y)),
            (x, y) => x.or(y),
        }
    }
}

// ---------------------------------------------------------------- E58
pub struct PrefixSearch {
    n: usize,
    t: Vec<u64>,
}

impl PrefixSearch {
    pub fn new(a: &[u64]) -> Self {
        let mut st = PrefixSearch { n: a.len(), t: vec![0; 4 * a.len().max(1)] };
        if !a.is_empty() {
            st.build(1, 0, a.len() - 1, a);
        }
        st
    }
    fn build(&mut self, k: usize, lo: usize, hi: usize, a: &[u64]) {
        if lo == hi {
            self.t[k] = a[lo];
            return;
        }
        let m = (lo + hi) / 2;
        self.build(2 * k, lo, m, a);
        self.build(2 * k + 1, m + 1, hi, a);
        self.t[k] = self.t[2 * k] + self.t[2 * k + 1];
    }
    pub fn add(&mut self, i: usize, v: u64) {
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
    /// Θ(log n): a sinistra se la somma del figlio sinistro basta, altrimenti a destra cercando s - sinistro
    pub fn search(&self, mut s: u64) -> Option<usize> {
        if self.t[1] < s {
            return None;
        }
        let (mut k, mut lo, mut hi) = (1, 0, self.n - 1);
        while lo < hi {
            let m = (lo + hi) / 2;
            if self.t[2 * k] >= s {
                k *= 2;
                hi = m;
            } else {
                s -= self.t[2 * k]; // si sottrae solo andando a destra
                k = 2 * k + 1;
                lo = m + 1;
            }
        }
        Some(lo)
    }
    fn range_sum(&self, k: usize, lo: usize, hi: usize, l: usize, r: usize) -> u64 {
        if hi < l || r < lo {
            return 0;
        }
        if l <= lo && hi <= r {
            return self.t[k];
        }
        let m = (lo + hi) / 2;
        self.range_sum(2 * k, lo, m, l, r) + self.range_sum(2 * k + 1, m + 1, hi, l, r)
    }
    /// Θ(log² n): binary search for the answer, predicato "prefisso fino a mid >= s" (monotono: valori >= 0)
    pub fn search_bs(&self, s: u64) -> Option<usize> {
        if self.t[1] < s {
            return None;
        }
        let (mut lo, mut hi) = (0, self.n - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.range_sum(1, 0, self.n - 1, 0, mid) >= s { hi = mid } else { lo = mid + 1 }
        }
        Some(lo)
    }
}
#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    #[test]
    fn e56_board_example() {
        // lavagna 52:00-60:00
        let mut t = MinTree::new(&[3, 1, -2, 4, 6, 13, 2, 0]);
        assert_eq!(t.rmq(2, 5), 2);
        assert_eq!(t.rmq(0, 7), 2);
        assert_eq!(t.rmq(4, 7), 7);
        t.add(2, 7); // -2 -> 5
        assert_eq!(t.rmq(2, 5), 3);
        assert_eq!(t.rmq(0, 7), 7);
        assert_eq!(t.rmq(0, 3), 1);
        assert_eq!(MinTree::new(&[5]).rmq(0, 0), 0);
        // parità: la posizione più a sinistra
        assert_eq!(MinTree::new(&[2, 1, 1, 1]).rmq(0, 3), 1);
    }

    #[test]
    fn e56_against_naive() {
        let mut rng = Rng(56);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<i64> = rng.vec_i32(n, -5, 6).into_iter().map(i64::from).collect();
            let mut t = MinTree::new(&a);
            for _ in 0..80 {
                let l = rng.below(n);
                let r = l + rng.below(n - l);
                if rng.below(2) == 0 {
                    let v = rng.below(11) as i64 - 5;
                    t.add(l, v);
                    a[l] += v;
                } else {
                    let best = (l..=r).fold(l, |b, i| if a[i] < a[b] { i } else { b });
                    assert_eq!(t.rmq(l, r), best, "a={a:?} l={l} r={r}");
                }
            }
        }
    }

    #[test]
    fn e57_board_example() {
        // lavagna 62:00-68:00
        let mut t = MinCount::new(&[3, 0, 0, 4, 0, 13, 2, 0]);
        assert_eq!(t.query(0, 7), (0, 4));
        assert_eq!(t.query(2, 5), (0, 2));
        t.add(4, 13); // 0 -> 13
        assert_eq!(t.query(4, 5), (13, 2));
        assert_eq!(t.query(2, 5), (0, 1));
        assert_eq!(t.query(0, 7), (0, 3));
    }

    #[test]
    fn e57_against_naive() {
        let mut rng = Rng(57);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<i64> = rng.vec_i32(n, 0, 4).into_iter().map(i64::from).collect();
            let mut t = MinCount::new(&a);
            for _ in 0..80 {
                let l = rng.below(n);
                let r = l + rng.below(n - l);
                if rng.below(2) == 0 {
                    let v = rng.below(5) as i64 - 2;
                    t.add(l, v);
                    a[l] += v;
                } else {
                    let m = *a[l..=r].iter().min().unwrap();
                    let c = a[l..=r].iter().filter(|&&x| x == m).count();
                    assert_eq!(t.query(l, r), (m, c));
                }
            }
        }
    }

    fn naive_search(a: &[u64], s: u64) -> Option<usize> {
        let mut p = 0;
        for (i, &x) in a.iter().enumerate() {
            p += x;
            if p >= s {
                return Some(i);
            }
        }
        None
    }

    #[test]
    fn e58_board_example() {
        // lavagna 74:40 e 86:00
        let t = PrefixSearch::new(&[3, 1, 0, 4, 6, 13, 2, 0]);
        assert_eq!(t.search(22), Some(5));
        assert_eq!(t.search_bs(22), Some(5));
        assert_eq!(t.search(8), Some(3));
        assert_eq!(t.search(4), Some(1)); // il primo prefisso che arriva a 4
        assert_eq!(t.search(29), Some(6));
        assert_eq!(t.search(30), None);
        assert_eq!(t.search_bs(30), None);
        assert_eq!(t.search(0), Some(0));
    }

    #[test]
    fn e58_against_naive() {
        let mut rng = Rng(58);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<u64> = (0..n).map(|_| rng.below(5) as u64).collect();
            let mut t = PrefixSearch::new(&a);
            for _ in 0..80 {
                if rng.below(3) == 0 {
                    let i = rng.below(n);
                    let v = rng.below(4) as u64;
                    t.add(i, v);
                    a[i] += v;
                } else {
                    let s = rng.below(5 * n + 2) as u64;
                    let want = naive_search(&a, s);
                    assert_eq!(t.search(s), want, "a={a:?} s={s}");
                    assert_eq!(t.search_bs(s), want, "a={a:?} s={s}");
                }
            }
        }
    }
}
