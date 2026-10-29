//! Soluzioni di practice_l11 (E47-E49), scritte da Claude (non dal prof).
//! Per provarle: copia questo file al posto di src/lib.rs (salvando prima il tuo!) e lancia i test.

#![allow(dead_code)]

#[cfg(test)]
mod util;

pub fn lsb(i: usize) -> usize {
    i & i.wrapping_neg()
}

pub fn parent(i: usize) -> usize {
    i - lsb(i)
}

pub fn level(i: usize) -> usize {
    i.count_ones() as usize
}

pub fn sum_path(mut i: usize) -> Vec<usize> {
    let mut p = Vec::with_capacity(usize::BITS as usize);
    while i != 0 {
        p.push(i);
        i = parent(i);
    }
    p
}

pub fn add_path(mut i: usize, n: usize) -> Vec<usize> {
    let mut p = Vec::new();
    while i <= n {
        p.push(i);
        i += lsb(i);
    }
    p
}

pub fn merge_sort_inversions(a: &[i64]) -> u64 {
    fn rec(v: &mut [i64], buf: &mut Vec<i64>) -> u64 {
        let n = v.len();
        if n <= 1 {
            return 0;
        }
        let (l, r) = v.split_at_mut(n / 2);
        let mut c = rec(l, buf) + rec(r, buf);
        buf.clear();
        let (mut i, mut j) = (0, 0);
        while i < l.len() || j < r.len() {
            if j == r.len() || (i < l.len() && l[i] <= r[j]) {
                buf.push(l[i]);
                i += 1;
            } else {
                c += (l.len() - i) as u64; // r[j] è più piccolo di tutti quelli rimasti a sinistra
                buf.push(r[j]);
                j += 1;
            }
        }
        v.copy_from_slice(buf);
        c
    }
    let mut v = a.to_vec();
    let mut buf = Vec::with_capacity(v.len());
    rec(&mut v, &mut buf)
}

pub fn insertion_sort_inversions(a: &[i64]) -> u64 {
    let mut v = a.to_vec();
    let mut c = 0;
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            c += 1;
            j -= 1;
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::util::Rng;
    use super::*;

    fn inv_brute(a: &[i64]) -> u64 {
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

    #[test]
    fn e47_lecture_examples() {
        // lavagna: 7 = 0111 -> 0001; Sum(7) visita 7, 6, 4 (poi la radice 0); Add(5, ·) visita 5, 6, 8
        assert_eq!(lsb(7), 1);
        assert_eq!(lsb(6), 2);
        assert_eq!(lsb(0b1011000), 0b1000);
        assert_eq!(parent(7), 6);
        assert_eq!(parent(6), 4);
        assert_eq!(parent(4), 0);
        assert_eq!(sum_path(7), vec![7, 6, 4]);
        assert_eq!(add_path(5, 8), vec![5, 6, 8]);
        assert_eq!(add_path(8, 8), vec![8]);
        assert_eq!(sum_path(8), vec![8]);
        // livello = numero di bit a 1: 1, 2, 4, 8 al livello 1; 3, 5, 6 al livello 2; 7 al livello 3
        assert_eq!(level(8), 1);
        assert_eq!(level(6), 2);
        assert_eq!(level(7), 3);
        assert_eq!(level(0), 0);
    }

    #[test]
    fn e47_against_definition() {
        for i in 1usize..2000 {
            let l = 1usize << i.trailing_zeros();
            assert_eq!(lsb(i), l);
            assert_eq!(level(i), i.count_ones() as usize);
            // il nodo i copre (i - lsb(i), i]: i nodi di sum_path(i) partizionano [1, i]
            let p = sum_path(i);
            let mut covered = 0;
            let mut hi = i;
            for &x in &p {
                assert_eq!(x, hi, "i={i}");
                covered += lsb(x);
                hi = x - lsb(x);
            }
            assert_eq!(covered, i);
            assert!(p.len() <= 11);
            // add_path(i, n): esattamente i nodi x <= n il cui intervallo contiene i
            let n = 2000;
            let expected: Vec<usize> = (i..=n).filter(|&x| x - lsb(x) < i).collect();
            assert_eq!(add_path(i, n), expected, "i={i}");
        }
    }

    #[test]
    fn e48_e49_examples() {
        // lavagna: A = 3 2 1 6 5 4 ha 6 inversioni
        for a in [vec![3, 2, 1, 6, 5, 4], vec![3, 2, 1], vec![], vec![7], vec![1, 2, 3]] {
            let b = inv_brute(&a);
            assert_eq!(insertion_sort_inversions(&a), b, "{a:?}");
            assert_eq!(merge_sort_inversions(&a), b, "{a:?}");
        }
        assert_eq!(merge_sort_inversions(&[3, 2, 1, 6, 5, 4]), 6);
        // lavagna 84:40: l'albero del mergesort su 3 2 6 5 4 3 7 8
        assert_eq!(merge_sort_inversions(&[3, 2, 6, 5, 4, 3, 7, 8]), 7); // 3>2; 6>5,4,3; 5>4,3; 4>3
        // decrescente: n(n-1)/2
        let a: Vec<i64> = (1..=100).rev().collect();
        assert_eq!(merge_sort_inversions(&a), 4950);
        assert_eq!(insertion_sort_inversions(&a), 4950);
    }

    #[test]
    fn e48_e49_against_brute_force() {
        let mut rng = Rng(47);
        for _ in 0..500 {
            let n = rng.below(40);
            let a: Vec<i64> = rng.vec_i32(n, -10, 10).into_iter().map(i64::from).collect(); // duplicati e negativi
            let b = inv_brute(&a);
            assert_eq!(insertion_sort_inversions(&a), b, "{a:?}");
            assert_eq!(merge_sort_inversions(&a), b, "{a:?}");
        }
    }

    #[test]
    fn e48_large() {
        // 2e5 elementi: deve essere veloce (Θ(n log n)) e il risultato non sta in u32
        let n = 200_000;
        let a: Vec<i64> = (0..n).rev().collect();
        assert_eq!(merge_sort_inversions(&a), (n as u64) * (n as u64 - 1) / 2);
    }
}
