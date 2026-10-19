//! L9 (14/10/2026), second part: static prefix sums.
//!
//! `prefix_sums_scan` and `Ilya` are the prof's code, copied from
//! <https://pages.di.unipi.it/rossano/blog/2023/prefixsums> (only comments and the
//! `pub` on the struct added). The rest (`prefix_sums`, `range_sum`, brute forces, tests) is [extra].

/// Prof's code (blog): prefix sums with `scan`. `psums[i] = a[0] + ... + a[i]`.
pub fn prefix_sums_scan(a: &[i64]) -> Vec<i64> {
    let psums = a
        .iter()
        .scan(0, |sum, e| {
            *sum += e;
            Some(*sum)
        })
        .collect::<Vec<_>>();
    psums
}

/// [extra] Same, with a leading 0: `p[i] = a[0] + ... + a[i-1]`, so `p.len() == a.len() + 1`.
/// With the extra 0 the range sum needs no `if`: sum of `a[l..r]` = `p[r] - p[l]`.
pub fn prefix_sums(a: &[i64]) -> Vec<i64> {
    let mut p = Vec::with_capacity(a.len() + 1);
    p.push(0);
    for &x in a {
        p.push(p.last().unwrap() + x);
    }
    p
}

/// [extra] RangeSum(l, r) on the half-open range `a[l..r]` (0-based), O(1).
/// In the lecture (1-based, closed): RangeSum(i, j) = Sum(j) - Sum(i - 1).
pub fn range_sum(p: &[i64], l: usize, r: usize) -> i64 {
    assert!(l <= r && r < p.len());
    p[r] - p[l]
}

/// [extra] Recover A from its prefix sums (lecture: "the difference between consecutive elements"),
/// so storing P instead of A costs no extra space (an implicit data structure).
pub fn recover(psums: &[i64]) -> Vec<i64> {
    let mut a = Vec::with_capacity(psums.len());
    let mut prev = 0;
    for &s in psums {
        a.push(s - prev);
        prev = s;
    }
    a
}

/// Prof's code (blog): Ilya and Queries (Codeforces 313B).
/// `psums` = prefix sums of the binary vector `B[k] = (s[k] == s[k+1])`, length n - 1.
#[derive(Debug)]
pub struct Ilya {
    psums: Vec<usize>,
}

impl Ilya {
    pub fn new(s: &str) -> Self {
        let psums = s
            .as_bytes()
            .windows(2)
            .map(|w| if w[0] == w[1] { 1usize } else { 0usize })
            .scan(0, |sum, e| {
                *sum += e;
                Some(*sum)
            })
            .collect::<Vec<_>>();

        Self { psums }
    }

    // Queries use 0-based indexing
    // q(i, j) = #{k : i <= k < j, s[k] == s[k+1]}  (the 1-based query (l, r) is q(l-1, r-1))
    pub fn q(&self, i: usize, j: usize) -> usize {
        assert!(i < j);
        assert!(j <= self.psums.len());

        self.psums[j - 1] - if i != 0 { self.psums[i - 1] } else { 0 }
    }
}

/// [extra] Brute force for Ilya and Queries: O(j - i) per query.
pub fn ilya_brute(s: &str, i: usize, j: usize) -> usize {
    let b = s.as_bytes();
    (i..j).filter(|&k| b[k] == b[k + 1]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
    }

    #[test]
    fn blog_example() {
        // example of the blog post
        let a = vec![2, 4, 1, 7, 3, 0, 4, 2];
        assert_eq!(prefix_sums_scan(&a), vec![2, 6, 7, 14, 17, 17, 21, 23]);
        assert_eq!(recover(&prefix_sums_scan(&a)), a);
        let p = prefix_sums(&a);
        assert_eq!(range_sum(&p, 0, 8), 23);
        assert_eq!(range_sum(&p, 3, 5), 10);
        assert_eq!(range_sum(&p, 4, 4), 0);
    }

    #[test]
    fn lecture_example_ilya() {
        // lecture: S = a b b a b b a (positions 1..7), Q(2,6) = 2
        let s = "abbabba";
        let il = Ilya::new(s);
        assert_eq!(il.q(1, 5), 2); // 1-based (2,6) -> 0-based (1,5)
        assert_eq!(ilya_brute(s, 1, 5), 2);
        assert_eq!(il.q(0, 6), 2);
        assert_eq!(il.q(2, 4), 0);
    }

    #[test]
    fn random_vs_brute() {
        let mut rng = Rng(0x9E3779B97F4A7C15);
        for _ in 0..300 {
            let n = 1 + rng.below(40) as usize;
            let a: Vec<i64> = (0..n).map(|_| rng.below(21) as i64 - 10).collect();
            let p = prefix_sums(&a);
            assert_eq!(&p[1..], &prefix_sums_scan(&a)[..]);
            for _ in 0..20 {
                let l = rng.below(n as u64 + 1) as usize;
                let r = l + rng.below((n - l) as u64 + 1) as usize;
                assert_eq!(range_sum(&p, l, r), a[l..r].iter().sum::<i64>());
            }
            if n >= 2 {
                let s: String = (0..n).map(|_| if rng.below(2) == 0 { 'a' } else { 'b' }).collect();
                let il = Ilya::new(&s);
                for _ in 0..20 {
                    let i = rng.below(n as u64 - 1) as usize;
                    let j = i + 1 + rng.below((n - 1 - i) as u64) as usize;
                    assert_eq!(il.q(i, j), ilya_brute(&s, i, j));
                }
            }
        }
    }
}
