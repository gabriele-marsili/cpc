//! SOLUZIONI L15 (E59-E62) — scritte da Claude, non dal prof. Copia questo file su src/lib.rs per provarle.

#[cfg(test)]
mod util;

// ---------------------------------------------------------------- E59
pub struct GenericSegTree<T, F> {
    n: usize,
    t: Vec<T>,
    neutral: T,
    combine: F,
}

impl<T: Clone, F: Fn(T, T) -> T> GenericSegTree<T, F> {
    pub fn new(a: &[T], neutral: T, combine: F) -> Self {
        let n = a.len();
        let mut st = GenericSegTree { n, t: vec![neutral.clone(); 4 * n.max(1)], neutral, combine };
        if n > 0 {
            st.build(a, 0, 0, n - 1);
        }
        st
    }
    fn build(&mut self, a: &[T], u: usize, l: usize, r: usize) {
        if l == r {
            self.t[u] = a[l].clone();
            return;
        }
        let m = (l + r) / 2;
        self.build(a, 2 * u + 1, l, m);
        self.build(a, 2 * u + 2, m + 1, r);
        self.t[u] = (self.combine)(self.t[2 * u + 1].clone(), self.t[2 * u + 2].clone());
    }
    pub fn set(&mut self, i: usize, x: T) {
        self.set_rec(0, 0, self.n - 1, i, x);
    }
    fn set_rec(&mut self, u: usize, l: usize, r: usize, i: usize, x: T) {
        if l == r {
            self.t[u] = x;
            return;
        }
        let m = (l + r) / 2;
        if i <= m { self.set_rec(2 * u + 1, l, m, i, x) } else { self.set_rec(2 * u + 2, m + 1, r, i, x) }
        self.t[u] = (self.combine)(self.t[2 * u + 1].clone(), self.t[2 * u + 2].clone());
    }
    pub fn query(&self, i: usize, j: usize) -> T {
        self.q(0, 0, self.n - 1, i, j)
    }
    fn q(&self, u: usize, l: usize, r: usize, i: usize, j: usize) -> T {
        if i <= l && r <= j {
            return self.t[u].clone();
        }
        if r < i || j < l {
            return self.neutral.clone();
        }
        let m = (l + r) / 2;
        (self.combine)(self.q(2 * u + 1, l, m, i, j), self.q(2 * u + 2, m + 1, r, i, j))
    }
}

// ---------------------------------------------------------------- E60
pub struct SearchFrom {
    n: usize,
    t: Vec<u64>,
}

impl SearchFrom {
    pub fn new(a: &[u64]) -> Self {
        let mut st = SearchFrom { n: a.len(), t: vec![0; 4 * a.len().max(1)] };
        st.build(a, 0, 0, a.len() - 1);
        st
    }
    fn build(&mut self, a: &[u64], u: usize, l: usize, r: usize) {
        if l == r {
            self.t[u] = a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(a, 2 * u + 1, l, m);
        self.build(a, 2 * u + 2, m + 1, r);
        self.t[u] = self.t[2 * u + 1] + self.t[2 * u + 2];
    }
    pub fn add(&mut self, i: usize, v: u64) {
        let (mut u, mut l, mut r) = (0, 0, self.n - 1);
        loop {
            self.t[u] += v;
            if l == r {
                return;
            }
            let m = (l + r) / 2;
            if i <= m { (u, r) = (2 * u + 1, m) } else { (u, l) = (2 * u + 2, m + 1) }
        }
    }
    /// visita da sinistra a destra dei nodi di [p, n-1]: un nodo in total overlap la cui somma non basta
    /// viene "saltato" in O(1) (s -= somma); appena un nodo basta si scende al suo interno come in search.
    /// Si scende al più una volta per intero: O(log n).
    pub fn search(&self, p: usize, s: u64) -> Option<usize> {
        let mut s = s;
        self.go(0, 0, self.n - 1, p, &mut s)
    }
    fn go(&self, u: usize, l: usize, r: usize, p: usize, s: &mut u64) -> Option<usize> {
        if r < p {
            return None; // no overlap
        }
        if p <= l && self.t[u] < *s {
            *s -= self.t[u]; // total overlap che non basta: tutto il nodo nel prefisso
            return None;
        }
        if l == r {
            *s = 0;
            return Some(l); // foglia che fa arrivare a s (o s == 0)
        }
        let m = (l + r) / 2;
        self.go(2 * u + 1, l, m, p, s).or_else(|| self.go(2 * u + 2, m + 1, r, p, s))
    }
}

// ---------------------------------------------------------------- E61
pub struct LazyMin {
    n: usize,
    t: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazyMin {
    pub fn new(a: &[i64]) -> Self {
        let mut st = LazyMin { n: a.len(), t: vec![0; 4 * a.len()], lazy: vec![0; 4 * a.len()] };
        st.build(a, 0, 0, a.len() - 1);
        st
    }
    fn build(&mut self, a: &[i64], u: usize, l: usize, r: usize) {
        if l == r {
            self.t[u] = a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(a, 2 * u + 1, l, m);
        self.build(a, 2 * u + 2, m + 1, r);
        self.t[u] = self.t[2 * u + 1].min(self.t[2 * u + 2]);
    }
    fn apply(&mut self, u: usize, l: usize, r: usize, v: i64) {
        self.t[u] += v; // aggiungere v a tutti aggiunge v al minimo
        if l < r {
            self.lazy[u] += v; // comporre due addizioni = sommarle
        }
    }
    fn push(&mut self, u: usize, l: usize, r: usize) {
        if self.lazy[u] != 0 {
            let m = (l + r) / 2;
            let v = std::mem::take(&mut self.lazy[u]);
            self.apply(2 * u + 1, l, m, v);
            self.apply(2 * u + 2, m + 1, r, v);
        }
    }
    pub fn range_update(&mut self, i: usize, j: usize, v: i64) {
        self.upd(0, 0, self.n - 1, i, j, v);
    }
    fn upd(&mut self, u: usize, l: usize, r: usize, i: usize, j: usize, v: i64) {
        if i <= l && r <= j {
            self.apply(u, l, r, v);
            return;
        }
        if r < i || j < l {
            return;
        }
        self.push(u, l, r);
        let m = (l + r) / 2;
        self.upd(2 * u + 1, l, m, i, j, v);
        self.upd(2 * u + 2, m + 1, r, i, j, v);
        self.t[u] = self.t[2 * u + 1].min(self.t[2 * u + 2]);
    }
    pub fn range_min(&mut self, i: usize, j: usize) -> i64 {
        self.q(0, 0, self.n - 1, i, j)
    }
    fn q(&mut self, u: usize, l: usize, r: usize, i: usize, j: usize) -> i64 {
        if i <= l && r <= j {
            return self.t[u];
        }
        if r < i || j < l {
            return i64::MAX; // neutro del minimo
        }
        self.push(u, l, r);
        let m = (l + r) / 2;
        self.q(2 * u + 1, l, m, i, j).min(self.q(2 * u + 2, m + 1, r, i, j))
    }
}

// ---------------------------------------------------------------- E62
#[derive(Clone, Copy)]
struct Node {
    cnt: usize,
    left: usize,
    right: usize,
}

/// Versione t = segment tree indicizzato per POSIZIONE in cui sono state inserite (+1) le posizioni dei
/// t valori più piccoli. count(i, j, x) = range_sum(i, j) sulla versione k, k = #valori <= x (binary search
/// sui valori ordinati). Path copying: O(n log n) spazio, query O(log n).
pub struct AtMost {
    n: usize,
    sorted: Vec<i64>,
    nodes: Vec<Node>,
    roots: Vec<usize>,
}

impl AtMost {
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&p| a[p]);
        let mut am = AtMost {
            n,
            sorted: order.iter().map(|&p| a[p]).collect(),
            nodes: vec![Node { cnt: 0, left: 0, right: 0 }], // nodo 0 = albero vuoto condiviso
            roots: vec![0],
        };
        for &p in &order {
            let last = *am.roots.last().unwrap();
            let r = am.ins(last, 0, n - 1, p);
            am.roots.push(r);
        }
        am
    }
    fn ins(&mut self, u: usize, l: usize, r: usize, p: usize) -> usize {
        let mut node = self.nodes[u];
        node.cnt += 1;
        if l < r {
            let m = (l + r) / 2;
            if p <= m { node.left = self.ins(node.left, l, m, p) } else { node.right = self.ins(node.right, m + 1, r, p) }
        }
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    fn sum(&self, u: usize, l: usize, r: usize, i: usize, j: usize) -> usize {
        if u == 0 || r < i || j < l {
            return 0;
        }
        if i <= l && r <= j {
            return self.nodes[u].cnt;
        }
        let m = (l + r) / 2;
        self.sum(self.nodes[u].left, l, m, i, j) + self.sum(self.nodes[u].right, m + 1, r, i, j)
    }
    pub fn count(&self, i: usize, j: usize, x: i64) -> usize {
        let k = self.sorted.partition_point(|&y| y <= x);
        self.sum(self.roots[k], 0, self.n - 1, i, j)
    }
}

// ---------------------------------------------------------------- E63, E64: Mo
fn mo_order(n: usize, queries: &[(usize, usize)]) -> Vec<usize> {
    let s = (n as f64).sqrt() as usize + 1;
    let mut ord: Vec<usize> = (0..queries.len()).collect();
    ord.sort_by_key(|&k| (queries[k].0 / s, queries[k].1));
    ord
}

pub fn powerful_array(a: &[u64], max_val: usize, queries: &[(usize, usize)]) -> Vec<u64> {
    let mut cnt = vec![0u64; max_val];
    let mut ans = vec![0u64; queries.len()];
    let (mut cl, mut cr, mut cur) = (0usize, 0usize, 0u64); // finestra [cl, cr)
    for k in mo_order(a.len(), queries) {
        let (l, r) = queries[k];
        // (K+1)² - K² = 2K + 1  -> aggiungere e costa e·(2K+1); togliere e·(2K-1)
        let add = |i: usize, cnt: &mut Vec<u64>, cur: &mut u64| {
            let e = a[i];
            *cur += e * (2 * cnt[e as usize] + 1);
            cnt[e as usize] += 1;
        };
        let remove = |i: usize, cnt: &mut Vec<u64>, cur: &mut u64| {
            let e = a[i];
            cnt[e as usize] -= 1;
            *cur -= e * (2 * cnt[e as usize] + 1);
        };
        while cl > l {
            cl -= 1;
            add(cl, &mut cnt, &mut cur);
        }
        while cr <= r {
            add(cr, &mut cnt, &mut cur);
            cr += 1;
        }
        while cl < l {
            remove(cl, &mut cnt, &mut cur);
            cl += 1;
        }
        while cr > r + 1 {
            cr -= 1;
            remove(cr, &mut cnt, &mut cur);
        }
        ans[k] = cur;
    }
    ans
}

pub fn distinct(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut cnt = vec![0usize; a.len()];
    let mut ans = vec![0usize; queries.len()];
    let (mut cl, mut cr, mut cur) = (0usize, 0usize, 0usize);
    for k in mo_order(a.len(), queries) {
        let (l, r) = queries[k];
        while cl > l {
            cl -= 1;
            cnt[a[cl]] += 1;
            cur += (cnt[a[cl]] == 1) as usize;
        }
        while cr <= r {
            cnt[a[cr]] += 1;
            cur += (cnt[a[cr]] == 1) as usize;
            cr += 1;
        }
        while cl < l {
            cnt[a[cl]] -= 1;
            cur -= (cnt[a[cl]] == 0) as usize;
            cl += 1;
        }
        while cr > r + 1 {
            cr -= 1;
            cnt[a[cr]] -= 1;
            cur -= (cnt[a[cr]] == 0) as usize;
        }
        ans[k] = cur;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    const A: [i64; 8] = [5, 1, 3, 1, 4, 7, 2, 1]; // esempio delle note del prof (2026)

    #[test]
    fn e59_checks_of_the_notes() {
        let sum = GenericSegTree::new(&A, 0i64, |x, y| x + y);
        assert_eq!(sum.query(2, 6), 17);
        // RMQ: (valore, posizione), il min di tuple sceglie a parità la posizione più a sinistra
        let pairs: Vec<(i64, usize)> = A.iter().enumerate().map(|(p, &v)| (v, p)).collect();
        let mut rmq = GenericSegTree::new(&pairs, (i64::MAX, usize::MAX), |x, y| x.min(y));
        assert_eq!(rmq.query(2, 7).1, 3);
        rmq.set(1, (5, 1)); // add(1, 4) delle note
        assert_eq!(rmq.query(0, 7).1, 3);
        // minimo con occorrenze
        let occ: Vec<(i64, u32)> = A.iter().map(|&v| (v, 1)).collect();
        let comb = |x: (i64, u32), y: (i64, u32)| {
            if x.0 == y.0 { (x.0, x.1 + y.1) } else if x.0 < y.0 { x } else { y }
        };
        let mut mo = GenericSegTree::new(&occ, (i64::MAX, 0), comb);
        assert_eq!(mo.query(0, 7), (1, 3));
        mo.set(1, (0, 1)); // add(1, -1)
        assert_eq!(mo.query(0, 7), (0, 1));
    }

    #[test]
    fn e59_non_commutative() {
        // concatenazione di stringhe: associativa ma NON commutativa (sinistra prima, destra dopo)
        let s: Vec<String> = "abcdefg".chars().map(|c| c.to_string()).collect();
        let mut t = GenericSegTree::new(&s, String::new(), |x, y| x + &y);
        assert_eq!(t.query(1, 5), "bcdef");
        t.set(3, "X".to_string());
        assert_eq!(t.query(0, 6), "abcXefg");
    }

    #[test]
    fn e60_checks_and_random() {
        let mut t = SearchFrom::new(&[5, 1, 3, 1, 4, 7, 2, 1]);
        assert_eq!(t.search(2, 8), Some(4)); // 3 + 1 + 4 >= 8
        assert_eq!(t.search(0, 20), Some(5)); // = search(20) delle note
        assert_eq!(t.search(7, 2), None);
        assert_eq!(t.search(5, 0), Some(5));
        let mut rng = Rng(60);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<u64> = (0..n).map(|_| rng.below(5) as u64).collect();
            t = SearchFrom::new(&a);
            for _ in 0..60 {
                let p = rng.below(n);
                if rng.below(3) == 0 {
                    let v = rng.below(4) as u64;
                    t.add(p, v);
                    a[p] += v;
                } else {
                    let s = rng.below(5 * n + 2) as u64;
                    let mut acc = 0;
                    let want = (p..n).find(|&i| {
                        acc += a[i];
                        acc >= s
                    });
                    assert_eq!(t.search(p, s), want, "a={a:?} p={p} s={s}");
                }
            }
        }
    }

    #[test]
    fn e61_checks_and_random() {
        let mut t = LazyMin::new(&A);
        t.range_update(0, 4, 20);
        assert_eq!(t.range_min(0, 7), 1);
        assert_eq!(t.range_min(0, 3), 21);
        let mut rng = Rng(61);
        for _ in 0..300 {
            let n = 1 + rng.below(40);
            let mut a: Vec<i64> = rng.vec_i32(n, -20, 20).into_iter().map(i64::from).collect();
            let mut t = LazyMin::new(&a);
            for _ in 0..80 {
                let i = rng.below(n);
                let j = i + rng.below(n - i);
                if rng.below(2) == 0 {
                    let v = rng.below(21) as i64 - 10;
                    t.range_update(i, j, v);
                    for x in &mut a[i..=j] {
                        *x += v;
                    }
                } else {
                    assert_eq!(t.range_min(i, j), *a[i..=j].iter().min().unwrap());
                }
            }
        }
    }

    #[test]
    fn e62_checks_and_random() {
        let t = AtMost::new(&A);
        assert_eq!(t.count(1, 6, 2), 3); // 1, 1, 2
        assert_eq!(t.count(0, 7, 0), 0);
        assert_eq!(t.count(0, 7, 100), 8);
        let mut rng = Rng(62);
        for _ in 0..200 {
            let n = 1 + rng.below(40);
            let a: Vec<i64> = rng.vec_i32(n, -1000, 1000).into_iter().map(|x| i64::from(x) * 3).collect();
            let t = AtMost::new(&a);
            for _ in 0..60 {
                let i = rng.below(n);
                let j = i + rng.below(n - i);
                let x = rng.below(6001) as i64 - 3000;
                assert_eq!(t.count(i, j, x), a[i..=j].iter().filter(|&&y| y <= x).count());
            }
        }
    }

    #[test]
    fn e63_lecture_and_random() {
        // slide "Powerful Array": A = [2, 1, 3, 2, 2, 2, 1, 4], power(1, 6) = 1·2² + 3·1² + 2·3² = 25
        let a = [2u64, 1, 3, 2, 2, 2, 1, 4];
        assert_eq!(powerful_array(&a, 5, &[(1, 6), (0, 0), (0, 7)]), vec![25, 2, 1 * 4 + 2 * 16 + 3 + 4]);
        let mut rng = Rng(63);
        for _ in 0..200 {
            let n = 1 + rng.below(50);
            let a: Vec<u64> = (0..n).map(|_| 1 + rng.below(6) as u64).collect();
            let q: Vec<(usize, usize)> = (0..1 + rng.below(40))
                .map(|_| {
                    let l = rng.below(n);
                    (l, l + rng.below(n - l))
                })
                .collect();
            let got = powerful_array(&a, 7, &q);
            for (k, &(l, r)) in q.iter().enumerate() {
                let mut c = [0u64; 7];
                for &x in &a[l..=r] {
                    c[x as usize] += 1;
                }
                let want: u64 = (0..7).map(|e| e as u64 * c[e] * c[e]).sum();
                assert_eq!(got[k], want, "a={a:?} q={:?}", (l, r));
            }
        }
    }

    #[test]
    fn e64_random() {
        assert_eq!(distinct(&[1, 3, 1, 2, 1, 3, 3, 0, 2, 1, 2, 2, 3, 1, 0, 3], &[(0, 6), (2, 10), (7, 7)]), vec![3, 4, 1]);
        let mut rng = Rng(64);
        for _ in 0..200 {
            let n = 1 + rng.below(50);
            let a: Vec<usize> = (0..n).map(|_| rng.below(n.min(6))).collect();
            let q: Vec<(usize, usize)> = (0..1 + rng.below(40))
                .map(|_| {
                    let l = rng.below(n);
                    (l, l + rng.below(n - l))
                })
                .collect();
            let got = distinct(&a, &q);
            for (k, &(l, r)) in q.iter().enumerate() {
                let mut v = a[l..=r].to_vec();
                v.sort_unstable();
                v.dedup();
                assert_eq!(got[k], v.len());
            }
        }
    }
}
