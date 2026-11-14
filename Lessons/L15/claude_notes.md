# **CPC — Lecture 15 (provisional reconstruction)** — Segment tree, end: #occs · Successor · triplets · lazy propagation · persistent segment tree

Competitive Programming and Contests — Wednesday 04/11/2026

> **Provisional reconstruction: no recording, no notes of yours** (your `Lessons/L15/notes.md` only says to recover it when the recording is out). **Nothing here is known to have been said on 04/11.** The content is *guessed* from: (1) where L14 stopped — the statement of \#occs and “next lecture we conclude our discussion of segment tree by presenting many other problems, starting from this one; just storing the answer, which is a number, is no longer enough” (`L14_transcript.txt`, 88:00–89:55); (2) the prof’s **2025** notes `Lessons/L13/Segment_tree_2025.pdf` p. 9–19, which continue exactly from there, in that order; (3) the course page: rows of 02/11/2026 “Lazy Propagation” and 04/11/2026 “Persistency” (the rows are the prof’s plan; this year he is slower: on 02/11 he did not reach lazy propagation). **Probably the lecture covered the first sections and maybe not all of them.** When the recording comes out, this lecture will be redone on it.

> **Sources.** `Segment_tree_2025.pdf` p. 9–19 (prof, **last year**: every page shown here is from 2025); `Lessons/L14/L14_transcript.txt` (end); course page (checked 11/11/2026). Explanations around the pages are mine, written to make the 2025 pages readable: treat them as <sup>\[extra\]</sup>.
>
> Code: `Lessons/L15/src` (`cargo test -p l15`), all <sup>\[extra\]</sup>. No practice exercises while the lecture is provisional.
>
> **Highlights:** **general / to remember** **good: works, efficient, correct** **bad: pitfalls, wrong ideas, costs** **definitions and key results**

> **Where we are.** L14 (02/11): the cost of <span class="sans-serif">RangeSum</span> ($O(\log n)$ nodes), dynamic RMQ, RMQ with occurrences, <span class="sans-serif">Search</span>$(s)$, and the *statement* of \#occs (homework: try to solve it). The common thread: **golden rule — every node stores the answer for its range**. The new twist expected for L15: **the answer in a node is no longer a number but a whole data structure** (hash map, BST), and then **updates on ranges** (lazy propagation) and **queries on the past** (persistency). After L15: **no lecture in the week of 09/11** (prof at a conference). Hands-On 2 deadline **22/11/2026**.

# \#occs$(i,j,x)$: a hash map in every node

**Problem** (stated at the end of L14). $A[0,n-1]$ of integers; <span class="sans-serif">Add</span>$(i,v)$; **$\#\textsf{occs}(i,j,x)$ = number of occurrences of $x$ in $A[i..j]$**.

**2025 solution.** **Every node stores a hash map value $\to$ number of occurrences in its range** (node $[4,7]$ of $A=[3,1,0,4,6,13,2,0]$: $6\to1$, $13\to1$, $2\to1$, $0\to1$). Query: the usual visit; in each total-overlap node look up $x$ and **sum the counters**. Example: $\#\textsf{occs}(1,7,0)=2$.

- **query time $\Theta(\log n)$ w.h.p.**: $O(\log n)$ nodes, $O(1)$ expected per lookup;

- **space $O(n\log n)$**: each position appears in the map of every ancestor (one per level); **a more space-efficient solution: wavelet trees**.

This is the L14 remark made concrete: **the cost of a query = number of touched nodes $\times$ cost per node**. *\[Prof.’s 2025 notes, Segment_tree_2025.pdf p. 9 — see the PDF\]* <sup>\[extra\]</sup> <span class="sans-serif">Add</span> is not on the page: on each node of the leaf–root path decrement the old value’s counter and increment the new one’s, $\Theta(\log n)$ expected. `l15::occs::OccsTree` (same as `l14::occs`), tested on the example, on the space ($n(\log n+1)$ entries with distinct values) and against a scan.

# Successor$(i,j,x)$: a BST in every node

**Problem.** <span class="sans-serif">Add</span>$(i,v)$ and **$\textsf{Successor}(i,j,x)$: the successor of $x$ among the values of $A[i..j]$**.

**2025 solution.** **Every node stores a BST of the values of its range** (successor in $\Theta(\log n)$ inside one node). **Compute the successor in every total-overlap node and combine by taking the min**.

- **query $\Theta(\log^2 n)$**: $O(\log n)$ nodes $\times$ $\Theta(\log n)$ per BST search; space $\Theta(n\log n)$.

- **the query can be improved to $\Theta(\log n)$ with *fractional cascading*** (only named on the page, boxed in red; no details).

*\[Prof.’s 2025 notes, Segment_tree_2025.pdf p. 10 and p. 11 — see the PDF\]* <sup>\[extra\]</sup> On the page the example $\textsf{Successor}(1,5,5)$ is left unfinished (the value 6 at position 4 is circled): the answer is 6. I take “successor” as the smallest value $\ge x$ (the page does not say $\ge$ or $>$). `l15::successor::SuccTree` (a `BTreeMap` value$\to$count per node, as a multiset), tested against a scan. <span class="sans-serif">Add</span> costs $\Theta(\log^2 n)$ here (one BST update per level). <sup>\[extra\]</sup> Fractional cascading, in one line: link each element of a node’s sorted list to its position in the children’s lists, so that after one binary search at the top each lower node is reached in $O(1)$ instead of a new $O(\log n)$ search (standard technique; static version).

# Triplets: counting $i<j<k$ with $A[i]<A[j]<A[k]$

**Problem.** Given $A[0,n-1]$, **count the triplets $i<j<k$ with $A[i]<A[j]<A[k]$**. Example $A=[1,2,3,4,1]$: result 4 ($123$, $124$, $234$, $134$).

**2025 solution.**

- **$\Theta(n^3)$**: check every possible triplet.

- **Fix the middle element $j$**: “how many triplets with that $j$?” With $k=\#\{i<j: A[i]<A[j]\}$ and $k'=\#\{k>j: A[j]<A[k]\}$ the answer for $j$ is **$k\cdot k'$** (example: $j$ at the 3: $k=2$, $k'=1$, $k\cdot k'=2$). Counting by scanning: $\Theta(n^2)$.

- **$\Theta(n\log n)$**: compute all the $k$ with **a Fenwick tree of frequencies scanning from left to right** — exactly the inversion-counting trick of L12 —, and all the $k'$ scanning from right to left.

*\[Prof.’s 2025 notes, Segment_tree_2025.pdf p. 12 and p. 13 — see the PDF\]* <sup>\[extra\]</sup> Values remapped to their ranks first (as in L12), so the Fenwick tree has size $\le n$; “strictly smaller” = prefix sum up to rank$-1$, “strictly larger” = seen $-$ prefix up to rank. `l15::triplets::count_triplets` ($\Theta(n\log n)$) and `count_triplets_quadratic`, both tested against the $\Theta(n^3)$ brute force. **Note**: this problem uses a Fenwick tree, not a segment tree — the same counting structure, with the simpler tool.

# Range update with lazy propagation

**Problem.** $A[1,n]$; **$\textsf{RangeUpdate}(i,j,v)$: add $v$ to every $A[k]$, $k\in[i,j]$**, and <span class="sans-serif">Sum</span>$(i)$ (= <span class="sans-serif">RangeSum</span>$(0,i)$). **Range update was solved in L13 with two Fenwick trees**; the segment tree does it with a technique that works for many other operations.

**2025 solution: lazy propagation.** Besides the segment tree $ST$, keep a **lazy tree** of pending updates. RangeUpdate does the usual visit:

- **partial overlap**: **apply the change to the node in $ST$** (for sums: $+v\cdot|\text{overlap}|$) and continue;

- **total overlap**: apply it to the node and **remember the update in the lazy tree**, unless it is a leaf — **do not go down**;

- **no overlap**: do nothing.

Example: $\textsf{RangeUpdate}(0,4,20)$ on $A=[3,1,-2,4,6,13,2,0]$: root $27+100$, node $[0,3]$ (total overlap) $6+80$ and $+20$ in the lazy tree, $[4,7]$ $21+20$, $[4,5]$ $19+20$, leaf 4 $6+20$. **Running time: $\Theta(\log n)$** — the same visit as <span class="sans-serif">RangeSum</span>. $\textsf{Sum}(3)=\textsf{RangeSum}(0,3)$ also $\Theta(\log n)$. *\[Prof.’s 2025 notes, Segment_tree_2025.pdf p. 14 and p. 15 — see the PDF\]* <sup>\[extra\]</sup> What the pages leave implicit: **when a later visit goes below a node with a pending update, the update is first pushed to its two children** (applied to their values and stored in their lazy entries). `l15::lazy::LazyTree`, tested on the example (root 127, pending $+20$ on $[0,3]$) and against a plain array. <sup>\[extra\]</sup> Course page, row of **02/11/2026**: optional *Circular RMQ* (CF 52C) is exactly a lazy-propagation exercise (range add + range min on a circular array).

# Persistent segment tree

**Idea.** The data structure goes through versions: **operations that change the DS move the timestamp by 1**; **operations that query the DS may specify any valid timestamp**. **Problem** (*persistent range sum*): <span class="sans-serif">Add</span>$(i,v)$ and **$\textsf{RangeSum}(i,j,t)=\sum_{k=i}^{j}A[k]$ at time $t$**.

1.  **Attempt \#1: a copy of the ST after each update.** **RangeSum $\Theta(\log n)$**, but **space $\Theta(n\cdot t)$ after $t$ updates and <span class="sans-serif">Add</span> $\Theta(n)$**.

2.  **Attempt \#2.** Exploit: **an update changes just $\Theta(\log n)$ nodes**. Every node keeps the list of its (value, timestamp) pairs. **<span class="sans-serif">Add</span> $\Theta(\log n)$, space $\Theta(n+t\log n)$**; RangeSum$(i,j,t)$ searches for the **predecessor of $t$** in the list of every total-overlap node: **$\Theta(\log n\cdot\log t)$**.

3.  **Attempt \#3.** Exploit: **the $\Theta(\log n)$ changed nodes are on a root-to-leaf path**. Copy only that path (the new nodes point to the old, unchanged children) and keep **an array of roots indexed by timestamp**. **RangeSum $\Theta(\log n)$, <span class="sans-serif">Add</span> $\Theta(\log n)$, space $\Theta(n+t\log n)$ after $t$ updates.**

*\[Prof.’s 2025 notes, Segment_tree_2025.pdf p. 16 and p. 17 — see the PDF\]* *\[Prof.’s 2025 notes, Segment_tree_2025.pdf p. 18 and p. 19 — see the PDF\]* <sup>\[extra\]</sup> `l15::persistent::PersistentTree` (attempt \#3, nodes in a `Vec` arena with indices as pointers: the idiomatic Rust way to share subtrees), tested on the example of p. 19 ($\textsf{Add}(3,10)$, then $\textsf{Add}(7,2)$: 4 new nodes per update) and against attempt \#1 (a full copy per version).

# Exercises and deadlines (course page)

> **Row of 04/11/2026** (“Segment Trees: Persistency”): **no notes and no exercises**; the same date has a separate row for **Hands-On 2**, <https://pages.di.unipi.it/rossano/blog/2024/handson2>, **deadline 22/11/2026** (scaffold in `HandsOn/HO2`).
>
> **Still open, from earlier rows of this year’s table** (2026):
>
> - *Circular RMQ*, <http://codeforces.com/problemset/problem/52/C>: optional in the row of 02/11/2026.
>
> - **Update the Array**, <http://www.spoj.com/problems/UPDATEIT/>: mandatory in the row of **19/10/2026**.
>
> - **Nested Segments**, <http://codeforces.com/problemset/problem/652/D>: mandatory in the row of **21/10/2026**; with segment trees: optional, row of 26/10/2026.
>
> - *Pashmak and Parmida’s problem*, <http://codeforces.com/problemset/problem/459/D>: optional, row of 21/10/2026.
>
> No rows after 04/11/2026 on the page (checked 11/11/2026).

# Next time

No lecture in the week of 09/11 (said at the end of L14). The 2025 notes on segment trees end at p. 19; what comes after L15 is not on the course page yet.
