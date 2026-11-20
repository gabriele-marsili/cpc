//! Mo's algorithm (lecture recorded on 16/11/2026, recording L_15.MP4, 41:00-93:00).
//! Prof's code copied VERBATIM from his notes "Mo's Algorithm"
//! (https://pages.di.unipi.it/rossano/blog/2023/mosalgorithm/): `three_or_more_slow`, `three_or_more`, `mos`.
//! Only this header, the comments marked [extra] and the tests are mine.
//! In the lecture the same algorithm was written in Python on the slides ("solve(A, Q)", window A[cur_i..cur_j-1]).

pub fn three_or_more_slow(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut counters: Vec<usize> = vec![0; a.len()];
    let mut answers = Vec::with_capacity(queries.len());

    for &(l, r) in queries {
        let answer = a[l..=r].iter().fold(0, |ans, &color| {
            counters[color] += 1;
            if counters[color] == 3 {
                ans + 1
            } else {
                ans
            }
        });

        answers.push(answer);

        a[l..=r].iter().for_each(|&color| counters[color] = 0);
    }

    answers
}

pub fn three_or_more(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut counters: Vec<usize> = vec![0; a.len()];
    let mut answers = Vec::with_capacity(queries.len());

    let mut cur_l = 0;
    let mut cur_r = 0; // here right endpoint is excluded
    let mut answer = 0;

    for &(l, r) in queries {
        let mut add = |i| {
            counters[a[i]] += 1;
            if counters[a[i]] == 3 {
                answer += 1
            }
        };

        while cur_l > l {
            cur_l -= 1;
            add(cur_l);
        }

        while cur_r <= r {
            add(cur_r);
            cur_r += 1;
        }

        let mut remove = |i| {
            counters[a[i]] -= 1;
            if counters[a[i]] == 2 {
                answer -= 1
            }
        };

        while cur_l < l {
            remove(cur_l);
            cur_l += 1;
        }

        while cur_r > r + 1 {
            cur_r -= 1;
            remove(cur_r);
        }

        answers.push(answer);
    }

    answers
}

pub fn mos(a: &[usize], queries: &[(usize, usize)]) -> Vec<usize> {
    // Sort the queries by bucket and get the permutation induced by this sorting.
    // The latter is needed to permute the answers back to the original ordering
    let mut sorted_queries: Vec<_> = queries.iter().cloned().collect();
    let mut permutation: Vec<usize> = (0..queries.len()).collect();

    let sqrt_n = (a.len() as f64).sqrt() as usize + 1;
    sorted_queries.sort_by_key(|&(l, r)| (l / sqrt_n, r));
    permutation.sort_by_key(|&i| (queries[i].0 / sqrt_n, queries[i].1));

    let answers = three_or_more(a, &sorted_queries);

    let mut permuted_answers = vec![0; answers.len()];
    for (i, answer) in permutation.into_iter().zip(answers) {
        permuted_answers[i] = answer;
    }

    permuted_answers
}

/// [extra] number of moves of cur_l and cur_r (= calls to add/remove) when the queries are processed
/// in the given order: the quantity the lecture's analysis bounds by O((n + q)·sqrt(n)) for Mo's order.
pub fn moves(queries: &[(usize, usize)]) -> usize {
    let (mut cl, mut cr, mut m) = (0usize, 0usize, 0usize); // window [cl, cr)
    for &(l, r) in queries {
        m += cl.abs_diff(l) + cr.abs_diff(r + 1);
        (cl, cr) = (l, r + 1);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn lecture_examples() {
        // slide "Three or More" (56:00-58:30): answers 1, 2, 0 (after the students' corrections)
        let a = [3, 1, 3, 0, 3, 1, 1, 0, 3, 1, 0, 0];
        let q = [(0, 5), (1, 9), (6, 10)];
        assert_eq!(three_or_more_slow(&a, &q), vec![1, 2, 0]);
        assert_eq!(three_or_more(&a, &q), vec![1, 2, 0]);
        assert_eq!(mos(&a, &q), vec![1, 2, 0]);
        // running example (68:00-71:40): from (0,6) to (2,10), both with answer 2
        let b = [1, 3, 1, 2, 1, 3, 3, 0, 2, 1, 2, 2, 3, 1, 0, 3];
        assert_eq!(three_or_more(&b, &[(0, 6), (2, 10)]), vec![2, 2]);
    }

    #[test]
    fn mo_order_of_the_slide() {
        // slide "Running example" (92:20): n = 16, sqrt(n) = 4, groups [0,3], [4,7], [8,11], [12,15]
        let b = [1, 3, 1, 2, 1, 3, 3, 0, 2, 1, 2, 2, 3, 1, 0, 3];
        let q = [(5, 13), (0, 6), (9, 15), (2, 10), (4, 8), (12, 15), (1, 3), (6, 11)];
        let mut lect: Vec<(usize, usize)> = q.to_vec();
        lect.sort_by_key(|&(i, j)| (i / 4, j)); // the lecture's key (i / sqrt(n), j)
        assert_eq!(lect, vec![(1, 3), (0, 6), (2, 10), (4, 8), (6, 11), (5, 13), (9, 15), (12, 15)]);
        assert!(moves(&lect) < moves(&q));
        assert_eq!(mos(&b, &q), three_or_more_slow(&b, &q));
    }

    #[test]
    fn bad_inputs_of_the_lecture() {
        // n nested prefixes [0, n-1], [0, n-2], ...: Theta(n) moves in this order (a best case for
        // "do not start from scratch"), Theta(n^2) for the slow algorithm; alternating long/short: Theta(n^2)
        let n = 1000;
        let shrinking: Vec<(usize, usize)> = (0..n).rev().map(|r| (0, r)).collect();
        assert!(moves(&shrinking) <= 2 * n);
        let mut alt = Vec::new();
        for k in 0..n / 2 {
            alt.push((0, n - 1 - k));
            alt.push((0, k));
        }
        assert!(moves(&alt) > n * n / 4);
        let mut sorted = alt.clone();
        let s = (n as f64).sqrt() as usize + 1;
        sorted.sort_by_key(|&(l, r)| (l / s, r));
        assert!(moves(&sorted) <= 2 * n);
    }

    #[test]
    fn random_vs_slow() {
        let mut rng = Rng(0x0300_3E);
        for _ in 0..200 {
            let n = 1 + rng.below(60) as usize;
            let a: Vec<usize> = (0..n).map(|_| rng.below(n.min(5) as u64) as usize).collect();
            let q: Vec<(usize, usize)> = (0..1 + rng.below(40))
                .map(|_| {
                    let l = rng.below(n as u64) as usize;
                    (l, l + rng.below((n - l) as u64) as usize)
                })
                .collect();
            let want = three_or_more_slow(&a, &q);
            assert_eq!(mos(&a, &q), want);
            assert_eq!(three_or_more(&a, &q), want);
        }
    }
}
