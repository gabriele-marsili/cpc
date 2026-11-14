// Hands-On 2 — CPC 2026/27
// Statement: https://pages.di.unipi.it/rossano/blog/2024/handson2/

// Run with: cargo test -p handson2 ; cargo fmt -p handson2 ; cargo clippy -p handson2

// ---------------------------------------------------------------------------
// Input parsing (shared by stdin, .txt files and command-line arguments)
// ---------------------------------------------------------------------------

/// Query of Problem 1. Indices are kept exactly as they appear in the input
/// (1-based, both in the statement and in the test set).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Query1 {
    /// `0 i j T`: A[k] = min(A[k], T) for i <= k <= j
    Update { i: usize, j: usize, t: i64 },
    /// `1 i j`: max of A[i..=j]
    Max { i: usize, j: usize },
}

/// Input of Problem 1 (Min and Max).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P1Input {
    pub a: Vec<i64>,
    pub queries: Vec<Query1>,
}

/// Input of Problem 2 (Is There). Indices are kept exactly as they appear in
/// the input (0-based, both in the statement and in the test set).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P2Input {
    /// segments `(l, r)`
    pub segments: Vec<(usize, usize)>,
    /// queries `(i, j, k)` = IsThere(i, j, k)
    pub queries: Vec<(usize, usize, usize)>,
}

/// Splits a text into whitespace-separated tokens, ignoring everything after
/// `//` on each line (so the commented example of the statement can be pasted).
pub fn tokenize_line(line: &str) -> impl Iterator<Item = String> + '_ {
    line.split("//")
        .next()
        .unwrap_or("")
        .split_whitespace()
        .map(str::to_owned)
}

/// Pulls the next token and parses it as `T`; `what` names the value in errors.
fn next_num<T: std::str::FromStr>(
    tokens: &mut impl Iterator<Item = String>,
    what: &str,
) -> Result<T, String> {
    let tok = tokens
        .next()
        .ok_or_else(|| format!("{what}: input ended too early"))?;
    tok.parse()
        .map_err(|_| format!("{what}: {tok:?} is not a valid number"))
}

/// Reads `n m`, the `n` values of A and the `m` queries. Stops right after the
/// last query, so interactive input does not need Ctrl-D.
pub fn parse_p1(tokens: &mut impl Iterator<Item = String>) -> Result<P1Input, String> {
    let n: usize = next_num(tokens, "n")?;
    let m: usize = next_num(tokens, "m")?;
    let a = (0..n)
        .map(|k| next_num(tokens, &format!("A[{k}]")))
        .collect::<Result<Vec<i64>, _>>()?;
    let mut queries = Vec::with_capacity(m);
    for q in 1..=m {
        let ctx = format!("query {q}");
        let kind: i64 = next_num(tokens, &format!("{ctx}, type"))?;
        let i = next_num(tokens, &format!("{ctx}, i"))?;
        let j = next_num(tokens, &format!("{ctx}, j"))?;
        queries.push(match kind {
            0 => Query1::Update {
                i,
                j,
                t: next_num(tokens, &format!("{ctx}, T"))?,
            },
            1 => Query1::Max { i, j },
            other => return Err(format!("{ctx}: type must be 0 or 1, got {other}")),
        });
    }
    Ok(P1Input { a, queries })
}

/// Reads `n m`, the `n` segments `l r` and the `m` queries `i j k`.
pub fn parse_p2(tokens: &mut impl Iterator<Item = String>) -> Result<P2Input, String> {
    let n: usize = next_num(tokens, "n")?;
    let m: usize = next_num(tokens, "m")?;
    let mut segments = Vec::with_capacity(n);
    for s in 1..=n {
        let l = next_num(tokens, &format!("segment {s}, l"))?;
        let r = next_num(tokens, &format!("segment {s}, r"))?;
        segments.push((l, r));
    }
    let mut queries = Vec::with_capacity(m);
    for q in 1..=m {
        let i = next_num(tokens, &format!("query {q}, i"))?;
        let j = next_num(tokens, &format!("query {q}, j"))?;
        let k = next_num(tokens, &format!("query {q}, k"))?;
        queries.push((i, j, k));
    }
    Ok(P2Input { segments, queries })
}


// ---------------------------------------------------------------------------
// Problem 1 and Problem 2: solvers
// ---------------------------------------------------------------------------

/// Problem 1: segment tree on A, every node keeps the max of its range.
/// Range update "A[p] = min(A[p], t)" with lazy propagation.
pub struct SegmentTree {
    n: usize,
    /// t[k] = max of the range of node k (root k = 1, children 2k and 2k+1)
    t: Vec<i64>,
    /// lazy[k] = Some(v): the children of k still have to receive "min with v"
    lazy: Vec<Option<i64>>,
}

impl SegmentTree {
    /// O(n): each node is computed once from its two children.
    pub fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut st = SegmentTree {
            n,
            t: vec![i64::MIN; 4 * n.max(1)],
            lazy: vec![None; 4 * n.max(1)],
        };
        if n > 0 {
            st.build(1, 0, n - 1, a);
        }
        st
    }

    fn build(&mut self, k: usize, lo: usize, hi: usize, a: &[i64]) {
        if lo == hi {
            self.t[k] = a[lo];
            return;
        }
        let mid = (lo + hi) / 2;
        self.build(2 * k, lo, mid, a);
        self.build(2 * k + 1, mid + 1, hi, a);
        self.t[k] = self.t[2 * k].max(self.t[2 * k + 1]);
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Applies "min with v" to the whole range of node k without visiting it:
    /// max(min(x, v)) = min(max(x), v), so t[k] is correct immediately, and the
    /// children are told later (lazy[k]). Two pending mins combine into one.
    fn apply(&mut self, k: usize, v: i64) {
        self.t[k] = self.t[k].min(v);
        self.lazy[k] = Some(match self.lazy[k] {
            Some(old) => old.min(v),
            None => v,
        });
    }

    /// Hands the pending min of node k down to its children, before visiting them.
    /// Never called on a leaf: a leaf is always fully inside or fully outside a range.
    fn push(&mut self, k: usize) {
        if let Some(v) = self.lazy[k].take() {
            self.apply(2 * k, v);
            self.apply(2 * k + 1, v);
        }
    }

    /// A[p] = min(A[p], v) for i <= p <= j (0-based, inclusive). O(log n).
    pub fn update(&mut self, i: usize, j: usize, v: i64) {
        if i <= j && j < self.n {
            self.update_rec(1, 0, self.n - 1, i, j, v);
        }
    }

    fn update_rec(&mut self, k: usize, lo: usize, hi: usize, i: usize, j: usize, v: i64) {
        // no overlap, or every value in the range is already <= v: nothing changes
        if hi < i || lo > j || self.t[k] <= v {
            return;
        }
        if i <= lo && hi <= j {
            // range of k fully inside [i, j]: stop here
            self.apply(k, v);
            return;
        }
        self.push(k);
        let mid = (lo + hi) / 2;
        self.update_rec(2 * k, lo, mid, i, j, v);
        self.update_rec(2 * k + 1, mid + 1, hi, i, j, v);
        self.t[k] = self.t[2 * k].max(self.t[2 * k + 1]);
    }

    /// max of A[i..=j] (0-based, inclusive). O(log n).
    pub fn max(&mut self, i: usize, j: usize) -> i64 {
        assert!(
            i <= j && j < self.n,
            "max({i}, {j}) out of range, n = {}",
            self.n
        );
        self.max_rec(1, 0, self.n - 1, i, j)
    }

    /// `&mut self` because visiting a child first pushes the pending min into it.
    fn max_rec(&mut self, k: usize, lo: usize, hi: usize, i: usize, j: usize) -> i64 {
        if hi < i || lo > j {
            return i64::MIN;
        }
        if i <= lo && hi <= j {
            return self.t[k];
        }
        self.push(k);
        let mid = (lo + hi) / 2;
        let left = self.max_rec(2 * k, lo, mid, i, j);
        let right = self.max_rec(2 * k + 1, mid + 1, hi, i, j);
        left.max(right)
    }
}

/// Answers of Problem 1: one value per `Max` query, in input order.
/// O((n + m) log n) time, O(n) space.
pub fn solve_p1(input: &P1Input) -> Vec<i64> {
    let mut st = SegmentTree::new(&input.a);
    let mut out = Vec::new();
    for &q in &input.queries {
        // queries are 1-based, the tree is 0-based
        match q {
            Query1::Update { i, j, t } => st.update(i - 1, j - 1, t),
            Query1::Max { i, j } => out.push(st.max(i - 1, j - 1)),
        }
    }
    out
}

/// Problem 2: segment tree over the positions 0..n, node k stores how many
/// positions of its range are "active". Point update, range sum.
pub struct CountTree {
    n: usize,
    t: Vec<u32>,
}

impl CountTree {
    /// All positions inactive.
    pub fn new(n: usize) -> Self {
        CountTree {
            n,
            t: vec![0; 4 * n.max(1)],
        }
    }

    /// Marks position `pos` as active (true) or not (false). O(log n).
    pub fn set(&mut self, pos: usize, active: bool) {
        self.set_rec(1, 0, self.n - 1, pos, u32::from(active));
    }

    fn set_rec(&mut self, k: usize, lo: usize, hi: usize, pos: usize, v: u32) {
        if lo == hi {
            self.t[k] = v;
            return;
        }
        let mid = (lo + hi) / 2;
        if pos <= mid {
            self.set_rec(2 * k, lo, mid, pos, v);
        } else {
            self.set_rec(2 * k + 1, mid + 1, hi, pos, v);
        }
        self.t[k] = self.t[2 * k] + self.t[2 * k + 1];
    }

    /// Number of active positions in [i, j] (0-based, inclusive). O(log n).
    pub fn count(&self, i: usize, j: usize) -> u32 {
        self.count_rec(1, 0, self.n - 1, i, j)
    }

    fn count_rec(&self, k: usize, lo: usize, hi: usize, i: usize, j: usize) -> u32 {
        if hi < i || lo > j {
            return 0;
        }
        if i <= lo && hi <= j {
            return self.t[k];
        }
        let mid = (lo + hi) / 2;
        self.count_rec(2 * k, lo, mid, i, j) + self.count_rec(2 * k + 1, mid + 1, hi, i, j)
    }
}

/// Answers of Problem 2: one 0/1 per query, in input order.
///
/// 1. cover[p] = number of segments containing p, with a difference array: O(n).
/// 2. Offline, by value: for each k = 0..=n, activate in a CountTree the
///    positions with cover[p] == k, answer the queries with that k
///    (IsThere = at least one active position in [i, j]), then deactivate them.
///    Every position is activated and deactivated once.
///
/// A position is covered by at most n segments, so a query with k > n is 0.
/// O((n + m) log n) time, O(n + m) space.
pub fn solve_p2(input: &P2Input) -> Vec<u8> {
    let n = input.segments.len();
    let mut ans = vec![0u8; input.queries.len()];
    if n == 0 {
        return ans;
    }

    // 1. coverage of every position
    let mut diff = vec![0i64; n + 1];
    for &(l, r) in &input.segments {
        diff[l] += 1;
        diff[r + 1] -= 1;
    }
    let mut positions_with = vec![Vec::new(); n + 1]; // positions_with[k] = {p : cover[p] = k}
    let mut cover = 0i64;
    for (p, d) in diff.iter().take(n).enumerate() {
        cover += d;
        positions_with[cover as usize].push(p);
    }

    // 2. queries grouped by k (k > n stays 0)
    let mut queries_with = vec![Vec::new(); n + 1];
    for (q, &(_, _, k)) in input.queries.iter().enumerate() {
        if k <= n {
            queries_with[k].push(q);
        }
    }

    let mut tree = CountTree::new(n);
    for k in 0..=n {
        if queries_with[k].is_empty() {
            continue;
        }
        for &p in &positions_with[k] {
            tree.set(p, true);
        }
        for &q in &queries_with[k] {
            let (i, j, _) = input.queries[q];
            ans[q] = u8::from(tree.count(i, j) > 0);
        }
        for &p in &positions_with[k] {
            tree.set(p, false);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(s: &str) -> impl Iterator<Item = String> + '_ {
        s.lines().flat_map(tokenize_line)
    }

    #[test]
    fn p1_parses_statement_example_with_comments() {
        let text =
            "5 3       // n m\n5 1 4 3 2 // The array A\n0 1 2 2   // Update\n1 2 4\n1 1 2\n";
        let got = parse_p1(&mut toks(text)).unwrap();
        assert_eq!(
            got,
            P1Input {
                a: vec![5, 1, 4, 3, 2],
                queries: vec![
                    Query1::Update { i: 1, j: 2, t: 2 },
                    Query1::Max { i: 2, j: 4 },
                    Query1::Max { i: 1, j: 2 },
                ],
            }
        );
    }

    #[test]
    fn p1_parses_all_on_one_line_like_cli_args() {
        let args = "5 3 5 1 4 3 2 0 1 2 2 1 2 4 1 1 2"
            .split(' ')
            .map(String::from);
        let got = parse_p1(&mut args.into_iter()).unwrap();
        assert_eq!(got.a, vec![5, 1, 4, 3, 2]);
        assert_eq!(got.queries[0], Query1::Update { i: 1, j: 2, t: 2 });
        assert_eq!(got.queries.len(), 3);
    }

    #[test]
    fn p1_stops_after_last_query_without_consuming_more() {
        let mut it = toks("1 1\n7\n1 1 1\nEXTRA");
        let got = parse_p1(&mut it).unwrap();
        assert_eq!(got.queries, vec![Query1::Max { i: 1, j: 1 }]);
        assert_eq!(it.next().as_deref(), Some("EXTRA"));
    }

    #[test]
    fn p1_rejects_truncated_update() {
        let err = parse_p1(&mut toks("2 1\n1 2\n0 1 2")).unwrap_err();
        assert_eq!(err, "query 1, T: input ended too early");
    }

    #[test]
    fn p1_rejects_unknown_query_type() {
        let err = parse_p1(&mut toks("1 1\n1\n2 1 1")).unwrap_err();
        assert_eq!(err, "query 1: type must be 0 or 1, got 2");
    }

    #[test]
    fn p2_parses_statement_example() {
        let text = "5 4\n0 4\n1 3\n1 2\n1 1\n0 0\n0 4 4\n0 4 0\n1 3 1\n1 4 1\n";
        let got = parse_p2(&mut toks(text)).unwrap();
        assert_eq!(
            got,
            P2Input {
                segments: vec![(0, 4), (1, 3), (1, 2), (1, 1), (0, 0)],
                queries: vec![(0, 4, 4), (0, 4, 0), (1, 3, 1), (1, 4, 1)],
            }
        );
    }

    #[test]
    fn p2_rejects_non_numeric_token() {
        let err = parse_p2(&mut toks("1 1\n0 x\n0 0 1")).unwrap_err();
        assert_eq!(err, "segment 1, r: \"x\" is not a valid number");
    }

    /// Every official input parses, consumes all its tokens, and has as many
    /// answers in outputK.txt as the queries that produce one.
    #[test]
    fn official_testsets_parse_completely() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("testsets");
        if !root.is_dir() {
            // the official test set is not shipped with main.rs/lib.rs
            return;
        }
        for problem in [1, 2] {
            let dir = root.join(format!("p{problem}"));
            let mut checked = 0;
            for k in 0.. {
                let Ok(input) = std::fs::read_to_string(dir.join(format!("input{k}.txt"))) else {
                    break;
                };
                let output = std::fs::read_to_string(dir.join(format!("output{k}.txt"))).unwrap();
                let answers = output.split_whitespace().count();
                let mut it = toks(&input);
                let expected = if problem == 1 {
                    let p = parse_p1(&mut it).unwrap_or_else(|e| panic!("p1/input{k}: {e}"));
                    p.queries
                        .iter()
                        .filter(|q| matches!(q, Query1::Max { .. }))
                        .count()
                } else {
                    let p = parse_p2(&mut it).unwrap_or_else(|e| panic!("p2/input{k}: {e}"));
                    p.queries.len()
                };
                assert_eq!(expected, answers, "p{problem}/input{k}");
                assert_eq!(it.next(), None, "p{problem}/input{k}: leftover tokens");
                checked += 1;
            }
            assert!(checked > 0, "no input files in {}", dir.display());
        }
    }

    #[test]
    fn p2_rejects_missing_queries() {
        let err = parse_p2(&mut toks("1 2\n0 0\n0 0 1")).unwrap_err();
        assert_eq!(err, "query 2, i: input ended too early");
    }

    // ---- solvers ----

    /// Small deterministic generator (xorshift64), so the crate needs no dependencies.
    struct Rng(u64);
    impl Rng {
        fn below(&mut self, n: usize) -> usize {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 % n as u64) as usize
        }
    }

    fn brute_p1(input: &P1Input) -> Vec<i64> {
        let mut a = input.a.clone();
        let mut out = Vec::new();
        for q in &input.queries {
            match *q {
                Query1::Update { i, j, t } => {
                    for x in &mut a[i - 1..j] {
                        *x = (*x).min(t);
                    }
                }
                Query1::Max { i, j } => out.push(*a[i - 1..j].iter().max().unwrap()),
            }
        }
        out
    }

    fn brute_p2(input: &P2Input) -> Vec<u8> {
        let n = input.segments.len();
        let cover: Vec<usize> = (0..n)
            .map(|p| {
                input
                    .segments
                    .iter()
                    .filter(|&&(l, r)| l <= p && p <= r)
                    .count()
            })
            .collect();
        input
            .queries
            .iter()
            .map(|&(i, j, k)| u8::from(cover[i..=j].contains(&k)))
            .collect()
    }

    #[test]
    fn p1_statement_example() {
        let input = parse_p1(&mut toks("5 3\n5 1 4 3 2\n0 1 2 2\n1 2 4\n1 1 2")).unwrap();
        assert_eq!(solve_p1(&input), vec![4, 2]);
    }

    #[test]
    fn p1_update_covering_whole_array_then_partial_max() {
        // the lazy value stored at the root must reach the leaves read later
        let input = parse_p1(&mut toks("4 4\n4 4 4 4\n0 1 4 2\n1 3 3\n0 2 2 1\n1 1 2")).unwrap();
        assert_eq!(solve_p1(&input), vec![2, 2]);
    }

    #[test]
    fn p1_matches_brute_force_on_random_inputs() {
        let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
        for case in 0..3000 {
            let n = 1 + rng.below(16);
            let a = (0..n).map(|_| 1 + rng.below(n) as i64).collect();
            let queries = (0..1 + rng.below(30))
                .map(|_| {
                    let i = 1 + rng.below(n);
                    let j = i + rng.below(n - i + 1);
                    if rng.below(2) == 0 {
                        Query1::Update {
                            i,
                            j,
                            t: 1 + rng.below(n) as i64,
                        }
                    } else {
                        Query1::Max { i, j }
                    }
                })
                .collect();
            let input = P1Input { a, queries };
            assert_eq!(solve_p1(&input), brute_p1(&input), "case {case}: {input:?}");
        }
    }

    #[test]
    fn p2_statement_example() {
        let text = "5 4\n0 4\n1 3\n1 2\n1 1\n0 0\n0 4 4\n0 4 0\n1 3 1\n1 4 1";
        let input = parse_p2(&mut toks(text)).unwrap();
        assert_eq!(solve_p2(&input), vec![1, 0, 0, 1]);
    }

    #[test]
    fn p2_k_larger_than_n_is_never_there() {
        let input = parse_p2(&mut toks("2 2\n0 1\n0 1\n0 1 2\n0 1 3")).unwrap();
        assert_eq!(solve_p2(&input), vec![1, 0]);
    }

    #[test]
    fn p2_matches_brute_force_on_random_inputs() {
        let mut rng = Rng(0xD1B5_4A32_D192_ED03);
        for case in 0..3000 {
            let n = 1 + rng.below(16);
            let segments = (0..n)
                .map(|_| {
                    let l = rng.below(n);
                    (l, l + rng.below(n - l))
                })
                .collect();
            let queries = (0..1 + rng.below(30))
                .map(|_| {
                    let i = rng.below(n);
                    (i, i + rng.below(n - i), rng.below(n + 2))
                })
                .collect();
            let input = P2Input { segments, queries };
            assert_eq!(solve_p2(&input), brute_p2(&input), "case {case}: {input:?}");
        }
    }
}
