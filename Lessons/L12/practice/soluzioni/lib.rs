//! Soluzioni di practice_l12 (E50-E52), scritte da Claude (non dal prof).
//! Per provarle: copia questo file al posto di src/lib.rs (salvando prima il tuo!) e lancia i test.

#![allow(dead_code)]

#[cfg(test)]
mod util;

/// Fenwick tree 1-based (come E46).
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
    fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l > r { 0 } else { self.sum(r) - self.sum(l - 1) }
    }
}

pub fn inversions_fenwick(a: &[u64]) -> u64 {
    let max = a.iter().copied().max().unwrap_or(0) as usize;
    let mut f = Fenwick::new(max);
    let mut res = 0;
    for &x in a {
        let x = x as usize;
        res += f.range_sum(x + 1, max) as u64; // valori già visti (a sinistra) e più grandi
        f.add(x, 1);
    }
    res
}

pub fn remap(a: &[u64]) -> Vec<u64> {
    let mut d = a.to_vec();
    d.sort_unstable();
    d.dedup();
    a.iter().map(|x| d.binary_search(x).unwrap() as u64 + 1).collect()
}

pub fn inversions_any(a: &[u64]) -> u64 {
    inversions_fenwick(&remap(a))
}

pub fn nested_segments(segs: &[(i64, i64)]) -> Vec<usize> {
    let n = segs.len();
    let mut c: Vec<i64> = segs.iter().flat_map(|&(l, r)| [l, r]).collect();
    c.sort_unstable();
    let pos = |x: i64| c.binary_search(&x).unwrap() + 1; // 1..=2n
    let mut f = Fenwick::new(2 * n);
    for &(l, _) in segs {
        f.add(pos(l), 1);
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by_key(|&i| std::cmp::Reverse(segs[i].1));
    let mut ans = vec![0; n];
    for i in order {
        let (l, r) = (pos(segs[i].0), pos(segs[i].1));
        ans[i] = (f.range_sum(l, r) - 1) as usize;
        f.add(l, -1);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    fn inv_brute(a: &[u64]) -> u64 {
        let mut c = 0;
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                if a[i] > a[j] {
                    c += 1;
                }
            }
        }
        c
    }
    fn nested_brute(s: &[(i64, i64)]) -> Vec<usize> {
        s.iter().map(|&(l, r)| s.iter().filter(|&&(l2, r2)| l < l2 && r2 < r).count()).collect()
    }

    #[test]
    fn e50_lecture_examples() {
        assert_eq!(inversions_fenwick(&[3, 2, 1, 6, 5, 4]), 6);
        // lavagna 26/10: 8 posizioni, valori fino a 6 (F è indicizzato per VALORE)
        assert_eq!(inversions_fenwick(&[3, 2, 1, 6, 5, 4, 3, 2]), 14);
        assert_eq!(inversions_fenwick(&[]), 0);
        assert_eq!(inversions_fenwick(&[1]), 0);
        let a: Vec<u64> = (1..=1000).rev().collect();
        assert_eq!(inversions_fenwick(&a), 499_500);
    }

    #[test]
    fn e50_against_brute_force() {
        let mut rng = Rng(50);
        for _ in 0..500 {
            let n = rng.below(40);
            let a: Vec<u64> = (0..n).map(|_| 1 + rng.below(n.max(1)) as u64).collect(); // 1 <= a[i] <= n, duplicati
            assert_eq!(inversions_fenwick(&a), inv_brute(&a), "{a:?}");
        }
    }

    #[test]
    fn e51_remap() {
        // lezione: [1, 150, 1_000_000, 155, 6, 8] -> ordinati 1 6 8 150 155 1e6 -> rango 1..6
        assert_eq!(remap(&[1, 150, 1_000_000, 155, 6, 8]), vec![1, 4, 6, 5, 2, 3]);
        // valori uguali -> stesso rango, ranghi densi 1..k
        assert_eq!(remap(&[50, 7, 50, 7, 1_000_000_000_000]), vec![2, 1, 2, 1, 3]);
        assert_eq!(remap(&[]), Vec::<u64>::new());
        let a = [1, 150, 1_000_000, 155, 6, 8];
        assert_eq!(inversions_any(&a), inv_brute(&a));
    }

    #[test]
    fn e51_against_brute_force() {
        let mut rng = Rng(51);
        for _ in 0..500 {
            let n = rng.below(40);
            let a: Vec<u64> = (0..n).map(|_| rng.next_u64() % 20 * 1_000_000_007).collect();
            let r = remap(&a);
            // l'ordine relativo è preservato e i ranghi sono densi
            for i in 0..n {
                for j in 0..n {
                    assert_eq!(a[i].cmp(&a[j]), r[i].cmp(&r[j]));
                }
            }
            let mut d = a.clone();
            d.sort_unstable();
            d.dedup();
            assert_eq!(r.iter().copied().max().unwrap_or(0), d.len() as u64);
            assert_eq!(inversions_any(&a), inv_brute(&a), "{a:?}");
        }
    }

    #[test]
    fn e52_lecture_example() {
        // lavagna 26/10 (coordinate già rimappate)
        let s = [(1, 5), (2, 10), (3, 4), (6, 8), (7, 11), (9, 12)];
        assert_eq!(nested_segments(&s), vec![1, 2, 0, 0, 0, 0]);
        // esempi di Codeforces 652D
        assert_eq!(nested_segments(&[(1, 8), (2, 3), (4, 7), (5, 6)]), vec![3, 0, 1, 0]);
        assert_eq!(nested_segments(&[(3, 4), (1, 5), (2, 6)]), vec![0, 1, 1]);
        assert_eq!(nested_segments(&[]), Vec::<usize>::new());
        // coordinate enormi e negative: serve il remapping
        assert_eq!(nested_segments(&[(-1_000_000_000, 1_000_000_000), (-5, 5)]), vec![1, 0]);
    }

    #[test]
    fn e52_against_brute_force() {
        let mut rng = Rng(52);
        for _ in 0..400 {
            let n = rng.below(15);
            let p = rng.permutation(2 * n);
            let s: Vec<(i64, i64)> = p
                .chunks(2)
                .map(|c| {
                    let (x, y) = (c[0] as i64 * 1_000_003 - 50_000_000, c[1] as i64 * 1_000_003 - 50_000_000);
                    (x.min(y), x.max(y))
                })
                .collect();
            assert_eq!(nested_segments(&s), nested_brute(&s), "{s:?}");
        }
    }
}
