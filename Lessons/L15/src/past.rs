//! "Why should we query the past?" (prof's 2026 notes, end of the persistent segment tree section):
//! a persistent segment tree turns a sweep into a data structure.
//! Scan A from left to right; version t = segment tree INDEXED BY VALUE on the counters C_t, after the
//! first t additions add(A[t'], 1): C_t[x] = occurrences of x in A[0..t-1]. Then
//! - count(i, j, x) = #values < x in A[i..=j] = range_sum(0, x-1, j+1) - range_sum(0, x-1, i)
//!   (two queries on two past versions), O(log V);
//! - kth(i, j, k) = k-th smallest value in A[i..=j]: descend versions j+1 and i together; the difference of
//!   the two left children tells how many values of A[i..=j] fall in the left half (as in Search), O(log V).
//! Space O(V + n log V) with path copying.
//! Example of the notes, A = [5, 1, 3, 1, 4, 7, 2, 1], V = 8: count(2, 6, 4) = 4 - 1 = 3; the 2nd smallest
//! value of A[2..=6] is 2.
//! [extra] code mine (the notes give the algorithm, not the code); values must be in [0, V).
//! The notes' last exercise ("how many values are at most x", inserting POSITIONS in increasing order of
//! value) is NOT solved here: it is yours.

#[derive(Clone, Copy)]
struct Node {
    cnt: usize,
    left: usize,
    right: usize,
}

pub struct PastCounts {
    v: usize,
    nodes: Vec<Node>,
    roots: Vec<usize>, // roots[t]: counters of the prefix A[0..t-1]
}

impl PastCounts {
    pub fn new(a: &[usize], v: usize) -> Self {
        assert!(v >= 1 && a.iter().all(|&x| x < v));
        // node 0 = the empty tree of every range (shared): its children are itself
        let mut pc = PastCounts { v, nodes: vec![Node { cnt: 0, left: 0, right: 0 }], roots: vec![0] };
        for &x in a {
            let last = *pc.roots.last().unwrap();
            let r = pc.inc(last, 0, v - 1, x);
            pc.roots.push(r);
        }
        pc
    }
    fn inc(&mut self, u: usize, l: usize, r: usize, x: usize) -> usize {
        let mut node = self.nodes[u];
        node.cnt += 1;
        if l < r {
            let m = (l + r) / 2;
            if x <= m {
                node.left = self.inc(node.left, l, m, x);
            } else {
                node.right = self.inc(node.right, m + 1, r, x);
            }
        }
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    /// number of values in [0, hi] at version t (prefix of the counters)
    fn prefix(&self, u: usize, l: usize, r: usize, hi: usize) -> usize {
        if u == 0 || l > hi {
            return 0;
        }
        if r <= hi {
            return self.nodes[u].cnt;
        }
        let m = (l + r) / 2;
        self.prefix(self.nodes[u].left, l, m, hi) + self.prefix(self.nodes[u].right, m + 1, r, hi)
    }
    /// number of values smaller than x in A[i..=j]
    pub fn count(&self, i: usize, j: usize, x: usize) -> usize {
        if x == 0 {
            return 0;
        }
        let hi = (x - 1).min(self.v - 1);
        self.prefix(self.roots[j + 1], 0, self.v - 1, hi) - self.prefix(self.roots[i], 0, self.v - 1, hi)
    }
    /// k-th smallest value (k >= 1) in A[i..=j]
    pub fn kth(&self, i: usize, j: usize, mut k: usize) -> usize {
        assert!(k >= 1 && k <= j - i + 1);
        let (mut a, mut b) = (self.roots[j + 1], self.roots[i]);
        let (mut l, mut r) = (0, self.v - 1);
        while l < r {
            let m = (l + r) / 2;
            let in_left = self.nodes[self.nodes[a].left].cnt - self.nodes[self.nodes[b].left].cnt;
            if in_left >= k {
                (a, b, r) = (self.nodes[a].left, self.nodes[b].left, m);
            } else {
                k -= in_left;
                (a, b, l) = (self.nodes[a].right, self.nodes[b].right, m + 1);
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    #[test]
    fn notes_example() {
        let pc = PastCounts::new(&[5, 1, 3, 1, 4, 7, 2, 1], 8);
        assert_eq!(pc.count(2, 6, 4), 3);
        assert_eq!(pc.kth(2, 6, 2), 2);
        assert_eq!(pc.kth(2, 6, 1), 1);
        assert_eq!(pc.kth(2, 6, 5), 7);
    }

    #[test]
    fn random_vs_naive() {
        let mut rng = Rng(0x9A57);
        for _ in 0..200 {
            let n = 1 + rng.below(30) as usize;
            let v = 1 + rng.below(12) as usize;
            let a: Vec<usize> = (0..n).map(|_| rng.below(v as u64) as usize).collect();
            let pc = PastCounts::new(&a, v);
            for _ in 0..40 {
                let i = rng.below(n as u64) as usize;
                let j = i + rng.below((n - i) as u64) as usize;
                let x = rng.below(v as u64 + 2) as usize;
                assert_eq!(pc.count(i, j, x), a[i..=j].iter().filter(|&&y| y < x).count());
                let mut s = a[i..=j].to_vec();
                s.sort_unstable();
                let k = 1 + rng.below(s.len() as u64) as usize;
                assert_eq!(pc.kth(i, j, k), s[k - 1]);
            }
        }
    }
}
