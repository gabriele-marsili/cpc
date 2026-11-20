//! The digression of the lecture (from 39:40): how many bits to store the SHAPE of a tree (no values)?
//! - with pointers / indices in an array: Θ(n log n) bits;
//! - "child at 2i, 2i+1" (heap layout) works only if all levels are full but the last one;
//! - BALANCED PARENTHESES: visit the tree from the root, emit "(" when you enter a node and ")" when you
//!   leave it (pre-order + post-order): 2 bits per node, 2n bits. Decoding: "(" = go down to a new child,
//!   ")" = go back up. The tree is ORDERED but not BINARY: for a node with a single child we do not know if
//!   it is the left or the right one (extra bits needed). 2n bits is optimal: there are about 2^(2n) trees
//!   with n nodes (Catalan number, log2 = 2n - Θ(log n)).
//! - with rank/select on the bit vector one can navigate WITHOUT decompressing (children, parent, subtree
//!   size, even LCA in O(1)): not explained.
//! - the second representation: level by level, the degree of every node in UNARY. The prof left the number
//!   of bits as a question at the end of the recording (see practice E68).

/// Ordered tree: `children[v]` in left-to-right order, root = 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    pub children: Vec<Vec<usize>>,
}

impl Tree {
    pub fn len(&self) -> usize {
        self.children.len()
    }
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Renumber the nodes in pre-order (so that two equal shapes compare equal).
    pub fn canonical(&self) -> Tree {
        let mut order = Vec::new();
        let mut stack = vec![0];
        while let Some(v) = stack.pop() {
            order.push(v);
            for &c in self.children[v].iter().rev() {
                stack.push(c);
            }
        }
        let mut id = vec![0; self.len()];
        for (k, &v) in order.iter().enumerate() {
            id[v] = k;
        }
        let mut ch = vec![Vec::new(); self.len()];
        for &v in &order {
            ch[id[v]] = self.children[v].iter().map(|&c| id[c]).collect();
        }
        Tree { children: ch }
    }
}

/// `true` = "(", `false` = ")". Exactly 2n bits.
pub fn encode(t: &Tree) -> Vec<bool> {
    let mut out = Vec::with_capacity(2 * t.len());
    // iterative DFS: (node, next child index)
    let mut stack = vec![(0usize, 0usize)];
    out.push(true);
    while let Some(&mut (v, ref mut next)) = stack.last_mut() {
        if *next < t.children[v].len() {
            let c = t.children[v][*next];
            *next += 1;
            out.push(true); // enter c
            stack.push((c, 0));
        } else {
            out.push(false); // leave v
            stack.pop();
        }
    }
    out
}

/// Inverse of `encode` (nodes numbered in pre-order). Panics on an unbalanced sequence.
pub fn decode(bits: &[bool]) -> Tree {
    let mut children: Vec<Vec<usize>> = Vec::new();
    let mut path: Vec<usize> = Vec::new();
    for &b in bits {
        if b {
            let v = children.len();
            children.push(Vec::new());
            if let Some(&p) = path.last() {
                children[p].push(v);
            }
            path.push(v); // "(" : go down
        } else {
            path.pop().expect("unbalanced"); // ")" : go up
        }
    }
    assert!(path.is_empty(), "unbalanced");
    Tree { children }
}

pub fn to_string(bits: &[bool]) -> String {
    bits.iter().map(|&b| if b { '(' } else { ')' }).collect()
}

/// A BINARY tree with explicit left/right, to show what BP loses.
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Vec<Option<usize>>,
    pub right: Vec<Option<usize>>,
}

impl Binary {
    /// Forget left/right: keep the children in order (left first).
    pub fn as_ordered(&self) -> Tree {
        let children = (0..self.left.len())
            .map(|v| self.left[v].into_iter().chain(self.right[v]).collect())
            .collect();
        Tree { children }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;

    fn random_tree(r: &mut Rng, n: usize) -> Tree {
        let mut children = vec![Vec::new(); n];
        for v in 1..n {
            let p = r.below(v);
            children[p].push(v);
        }
        Tree { children }
    }

    #[test]
    fn round_trip_and_2n_bits() {
        let mut r = Rng(2718);
        for _ in 0..500 {
            let n = 1 + r.below(40);
            let t = random_tree(&mut r, n).canonical();
            let bits = encode(&t);
            assert_eq!(bits.len(), 2 * n);
            assert_eq!(decode(&bits), t);
        }
    }

    #[test]
    fn single_child_left_or_right_is_lost() {
        // root with one child: as the LEFT child or as the RIGHT child -> same parentheses "(())"
        let l = Binary { left: vec![Some(1), None], right: vec![None, None] };
        let rr = Binary { left: vec![None, None], right: vec![Some(1), None] };
        assert_eq!(to_string(&encode(&l.as_ordered())), "(())");
        assert_eq!(encode(&l.as_ordered()), encode(&rr.as_ordered()));
    }

    #[test]
    fn catalan_lower_bound() {
        // number of binary trees with n nodes = Catalan(n); log2 Catalan(n) = 2n - Θ(log n) < 2n
        let mut c: Vec<u128> = vec![1];
        for n in 1..=60usize {
            let next = (0..n).map(|i| c[i] * c[n - 1 - i]).sum();
            c.push(next);
        }
        for n in [10usize, 30, 60] {
            let bits = (c[n] as f64).log2();
            assert!(bits < 2.0 * n as f64 && bits > 2.0 * n as f64 - 1.5 * (n as f64).log2() - 1.0, "n={n} bits={bits}");
        }
    }
}
