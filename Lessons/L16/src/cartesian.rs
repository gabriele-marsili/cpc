//! [extra, PREVIEW] Solution #4 (O(1) query, Θ(n) space) was only started: the last heading on the board is
//! "Cartesian tree", then the prof digressed on how to store a tree shape in 2n bits (see `bp`).
//! This module is NOT from the lecture: it is the standard definition, so you can check the next lecture.
//!
//! Cartesian tree of A: the root is the (leftmost) minimum, the left subtree is the Cartesian tree of the
//! part on its left, the right subtree that of the part on its right. Then RMQ(i, j) = LCA(i, j).
//! Built left to right with a stack (the "right spine") in Θ(n).

pub struct Cartesian {
    pub parent: Vec<Option<usize>>,
    pub left: Vec<Option<usize>>,
    pub right: Vec<Option<usize>>,
    pub root: usize,
}

impl Cartesian {
    pub fn new<T: Ord>(a: &[T]) -> Self {
        let n = a.len();
        let (mut parent, mut left, mut right) = (vec![None; n], vec![None; n], vec![None; n]);
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..n {
            let mut last = None;
            // pop the strictly larger ones: an equal earlier value stays above (leftmost minimum = ancestor)
            while let Some(&top) = stack.last() {
                if a[top] > a[i] {
                    last = stack.pop();
                } else {
                    break;
                }
            }
            if let Some(l) = last {
                left[i] = Some(l);
                parent[l] = Some(i);
            }
            if let Some(&top) = stack.last() {
                right[top] = Some(i);
                parent[i] = Some(top);
            }
            stack.push(i);
        }
        let root = stack.first().copied().unwrap_or(0);
        Cartesian { parent, left, right, root }
    }

    fn depth(&self, mut v: usize) -> usize {
        let mut d = 0;
        while let Some(p) = self.parent[v] {
            v = p;
            d += 1;
        }
        d
    }

    /// Naive LCA (walk up): enough to check RMQ = LCA.
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        let (mut du, mut dv) = (self.depth(u), self.depth(v));
        while du > dv {
            u = self.parent[u].unwrap();
            du -= 1;
        }
        while dv > du {
            v = self.parent[v].unwrap();
            dv -= 1;
        }
        while u != v {
            u = self.parent[u].unwrap();
            v = self.parent[v].unwrap();
        }
        u
    }

    /// Shape of the tree in balanced parentheses (children in order left, right): 2n bits.
    /// Two blocks with the same shape have the same answers to all in-block RMQs (positions).
    pub fn shape(&self) -> Vec<bool> {
        let n = self.parent.len();
        let children = (0..n).map(|v| self.left[v].into_iter().chain(self.right[v]).collect()).collect();
        let t = crate::bp::Tree { children };
        // re-root at self.root
        let mut out = Vec::new();
        fn rec(t: &crate::bp::Tree, v: usize, out: &mut Vec<bool>) {
            out.push(true);
            for &c in &t.children[v] {
                rec(t, c, out);
            }
            out.push(false);
        }
        if n > 0 {
            rec(&t, self.root, &mut out);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LECTURE_A, rmq_naive, rng::Rng};

    #[test]
    fn lecture_array() {
        let c = Cartesian::new(&LECTURE_A);
        assert_eq!(c.root, 5); // the 1
        assert_eq!(c.lca(0, 6), 5);
        assert_eq!(c.lca(0, 3), 1); // min of 5,2,7,3 is the 2 in position 1
    }

    #[test]
    fn rmq_is_lca() {
        let mut r = Rng(9);
        for _ in 0..300 {
            let n = 1 + r.below(40);
            let a = r.vec(n, -3, 4);
            let c = Cartesian::new(&a);
            for i in 0..n {
                for j in i..n {
                    assert_eq!(c.lca(i, j), rmq_naive(&a, i, j), "a={a:?} {i} {j}");
                }
            }
        }
    }
}
