//! L10 (19/10/2026): Little Girl and Maximum Sum, Number of Ways, Fenwick tree (start).
//! Redone on the recording L_10.MP4 (21/10); notes in L10.pdf.
//!
//! `little_girl`, `number_of_ways` are the prof's code from
//! <https://pages.di.unipi.it/rossano/blog/2023/prefixsums>; `FenwickTree` is the prof's code from
//! <https://pages.di.unipi.it/rossano/blog/2023/fenwick>. Copied verbatim, only comments added.
//! Brute forces and tests are [extra].

pub mod fenwick;

// We assumes queries are 0-based indexed
/// Prof's code (blog): Little Girl and Maximum Sum (Codeforces 276C).
/// Frequencies of the positions with the "+1 at l, -1 at r+1, then prefix sums" trick,
/// then the largest values go to the most frequent positions (sort both).
pub fn little_girl(a: &[i64], q: &[(usize, usize)]) -> i64 {
    if a.is_empty() {
        return 0;
    }

    let mut u = vec![0i64; a.len()];

    for &(l, r) in q {
        assert!(l <= r);
        assert!(r < u.len());

        u[l] += 1;
        if r + 1 < u.len() {
            u[r + 1] -= 1;
        }
    }

    let mut f = u
        .iter()
        .scan(0, |sum, e| {
            *sum += e;
            Some(*sum)
        })
        .collect::<Vec<_>>();

    // we sort both f and a in decreasing order, nothing changes
    f.sort_unstable();
    let mut a_sorted = a.to_vec();
    a_sorted.sort_unstable();

    a_sorted
        .iter()
        .zip(f)
        .fold(0, |result, (value, freq)| result + value * freq)
}

/// Prof's code (blog): Number of Ways (Codeforces 466C).
/// `c[i]` = number of suffixes starting at position >= i whose sum is S/3; for every prefix
/// a[..=i] with sum S/3 add c[i + 2] (the third part must start at least two positions later).
pub fn number_of_ways(a: &[i64]) -> usize {
    let sum: i64 = a.iter().sum();

    if sum % 3 != 0 {
        return 0;
    }

    let target = sum / 3;
    let mut c: Vec<_> = a
        .iter()
        .rev()
        .scan(0, |sum, e| {
            *sum += e;
            Some(*sum)
        })
        .scan(0, |counter, sum| {
            if sum == target {
                *counter += 1usize
            };
            Some(*counter)
        })
        .collect();

    c.reverse();

    let mut result = 0;
    let mut sum = 0;
    for (i, &v) in a[..a.len() - 2].iter().enumerate() {
        sum += v;
        if sum == target {
            result += c[i + 2];
        }
    }

    result
}

/// [extra] Step #1 of Little Girl with the FIRST solution discussed in class: sweep line.
/// Events (+1 at the beginning, -1 after the end) sorted by position, then a scan of all the
/// positions: Theta(n + m log m). With counting sort on the positions (endpoints are <= n) it becomes
/// Theta(n + m), and the counting arrays C_b / C_e are exactly the array B of the prof's code.
/// Queries 0-based, as in `little_girl`.
pub fn frequencies_sweep(n: usize, q: &[(usize, usize)]) -> Vec<i64> {
    let mut events: Vec<(usize, i64)> = q.iter().flat_map(|&(l, r)| [(l, 1), (r + 1, -1)]).collect();
    events.sort_unstable();
    let mut f = vec![0i64; n];
    let (mut k, mut cur) = (0, 0);
    for (p, fp) in f.iter_mut().enumerate() {
        while k < events.len() && events[k].0 == p {
            cur += events[k].1;
            k += 1;
        }
        *fp = cur;
    }
    f
}

/// [extra] Brute force for Little Girl: the value of a given arrangement.
pub fn queries_value(a: &[i64], q: &[(usize, usize)]) -> i64 {
    q.iter().map(|&(l, r)| a[l..=r].iter().sum::<i64>()).sum()
}

/// [extra] Brute force for Little Girl: try all permutations (small n only).
pub fn little_girl_brute(a: &[i64], q: &[(usize, usize)]) -> i64 {
    fn rec(a: &mut Vec<i64>, k: usize, q: &[(usize, usize)], best: &mut i64) {
        if k == a.len() {
            *best = (*best).max(queries_value(a, q));
            return;
        }
        for i in k..a.len() {
            a.swap(k, i);
            rec(a, k + 1, q, best);
            a.swap(k, i);
        }
    }
    let mut v = a.to_vec();
    let mut best = i64::MIN;
    rec(&mut v, 0, q, &mut best);
    best
}

/// [extra] Brute force for Number of Ways: all pairs of cut points, O(n^2) with prefix sums.
pub fn number_of_ways_brute(a: &[i64]) -> usize {
    let n = a.len();
    let mut p = vec![0i64; n + 1];
    for i in 0..n {
        p[i + 1] = p[i] + a[i];
    }
    let mut count = 0;
    // parts a[0..i], a[i..j], a[j..n], all non-empty
    for i in 1..n {
        for j in i + 1..n {
            if p[i] == p[j] - p[i] && p[j] - p[i] == p[n] - p[j] {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
pub(crate) mod rng {
    pub struct Rng(pub u64);
    impl Rng {
        pub fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        pub fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rng::Rng;
    use super::*;

    #[test]
    fn notes_example_little_girl() {
        // lecture: A = [5,2,4,1,3], queries (1,5),(2,3),(2,3) (1-based) -> 27 as given, 33 optimal
        let a = [5, 2, 4, 1, 3];
        let q = [(0, 4), (1, 2), (1, 2)];
        assert_eq!(queries_value(&a, &q), 27);
        assert_eq!(queries_value(&[2, 5, 4, 1, 3], &q), 33);
        assert_eq!(little_girl(&a, &q), 33);
        assert_eq!(little_girl_brute(&a, &q), 33);
    }

    #[test]
    fn notes_example_frequencies() {
        // lecture: Q = (2,4),(1,3),(2,5) (1-based) -> F = [1,3,3,2,1], B = [1,2,0,-1,-1]
        let q = [(1, 3), (0, 2), (1, 4)];
        let mut b = vec![0i64; 5];
        for &(l, r) in &q {
            b[l] += 1;
            if r + 1 < 5 {
                b[r + 1] -= 1;
            }
        }
        assert_eq!(b, vec![1, 2, 0, -1, -1]);
        let f: Vec<i64> = b.iter().scan(0, |s, e| { *s += e; Some(*s) }).collect();
        assert_eq!(f, vec![1, 3, 3, 2, 1]);
        assert_eq!(frequencies_sweep(5, &q), f);
    }

    #[test]
    fn number_of_ways_examples() {
        assert_eq!(number_of_ways(&[1, 2, 3, 0, 3]), 2); // your notes / Codeforces sample
        assert_eq!(number_of_ways(&[0, 1, -1, 0]), 1); // Codeforces sample
        assert_eq!(number_of_ways(&[4, 1]), 0); // Codeforces sample (n < 3)
        assert_eq!(number_of_ways(&[0, 0, 0, 0]), 3);
    }

    // [extra] the prof's code computes a.len() - 2: with n = 1 (allowed by Codeforces) it underflows.
    #[test]
    #[should_panic]
    fn number_of_ways_n1_panics() {
        number_of_ways(&[0]);
    }

    #[test]
    fn random_vs_brute() {
        let mut rng = Rng(0xD1B54A32D192ED03);
        for _ in 0..300 {
            let n = 1 + rng.below(6) as usize;
            let a: Vec<i64> = (0..n).map(|_| rng.below(11) as i64 - 3).collect();
            let m = rng.below(5) as usize;
            let q: Vec<(usize, usize)> = (0..m)
                .map(|_| {
                    let l = rng.below(n as u64) as usize;
                    (l, l + rng.below((n - l) as u64) as usize)
                })
                .collect();
            assert_eq!(little_girl(&a, &q), little_girl_brute(&a, &q), "a={a:?} q={q:?}");
            let brute: Vec<i64> = (0..n).map(|p| q.iter().filter(|&&(l, r)| l <= p && p <= r).count() as i64).collect();
            assert_eq!(frequencies_sweep(n, &q), brute, "q={q:?}");
        }
        for _ in 0..500 {
            let n = 3 + rng.below(12) as usize;
            let a: Vec<i64> = (0..n).map(|_| rng.below(5) as i64 - 2).collect();
            assert_eq!(number_of_ways(&a), number_of_ways_brute(&a), "a={a:?}");
        }
    }
}
