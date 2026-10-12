[L8] (lecture of Monday 12/10/2026)
-> prof's detailed notes in English (with code) on his web page: closest pair + previous binary search ones (later: Fenwick tree, DP) -> use them as reference
-> oral exam (Italian system) = in depth: know everything behind the introduction of each topic
-> "we are about 1 hour behind the expected programme": Frogs and Mosquitoes NOT done yet (my notes on it moved to L9/notes.md)

• Solution of the tiling puzzle (L7): remove 2 cells of DIFFERENT colour -> always tileable?
!! solving it by cases is possible but worthless
-> draw ONE chain (closed loop) through all the cells of the board (each step to an adjacent cell), regardless of the removed cells
-> removing 2 cells breaks the loop into 2 sub-chains, each starts and ends with different colours => even length
-> cover each sub-chain with dominoes on consecutive cells => YES, always, no cases (extra: Gomory's theorem)

• Puzzle: Alice & Bob
-> 2n coins in a row (even length), Alice starts, alternate; each turn take one of the 2 coins at the ENDS; full knowledge of the sequence; more money wins
    ex: [10,2,100,1,5,6,3,4] played badly -> A = 21, B = 110
-> question: strategy for Alice to get ALWAYS at least as much as Bob (any sequence, any Bob)?
!! take the largest end: no, ignores the inside (ex. 10 1 1M 5)
-> super coin (> sum of all the others): Alice never takes a coin adjacent to it; kernel (super coin + 2 neighbours) has odd length = Bob's turn => Bob must free it
=> STRATEGY: sum of coins in odd positions vs even positions, take the larger class
    -> at the start the 2 ends have different parity: Alice takes the one of her class; Bob sees only the other class at both ends; whatever he takes uncovers one of Alice's class
    -> ex: O = 10+100+5+3 = 118, E = 2+1+6+4 = 13 => Alice takes all the odd ones
    -> worst case O = E: Alice doesn't win but gets as much as Bob
-> best possible gain (not guaranteed by odd/even): game tree (A chooses L/R, then B, ...) -> 2^(2n) leaves, exponential
    -> but each node = contiguous subsequence, only θ(n^2) distinct ones => remember solved ones => DYNAMIC PROGRAMMING (soon; "quite a hard DP problem")
    -> e.g. maximize (my gain - opponent's gain), opponent optimal
-> why puzzles: they lead to questions related to next topics

• Closest pair of points ("super classic")
-> n points in 2D, find the 2 closest (Euclidean: d = sqrt((x1-x2)^2 + (y1-y2)^2))
-> trivial: all pairs θ(n^2); target θ(n log n)
-> exists a divide & conquer θ(n log n) (split, solve halves, handle "dangerous" points near the cut) -> worth reading, not in class
=> SWEEP LINE: sort by x (θ(n log n)), process left to right; δ = smallest distance among the points processed so far
!! comparing (x,y) with all previous points => quadratic again
-> candidates = points in the rectangle [x-δ, x] x [y-δ, y+δ] (only the left side: points on the right are compared when processed)
    (board typo: bottom-left corner is (x-δ, y-δ))
-> with an "oracle" giving the points in the rectangle: compute distances, update δ, move on
-> few points in the rectangle: any 2 of them are at distance >= δ (otherwise δ would already be smaller)
    -> split the rectangle in 8 squares of side δ/2 -> at most 1 point per square => at most 8 (involved proof: 6)
    => with the oracle: linear after sorting
-> oracle:
    !! BST sorted by x (x' >= x-δ): returns the whole vertical strip above/below the rectangle (blue) -> potentially most points
    -> BST sorted by y of the points processed so far: successor of y-δ (O(log n)), then next ones (O(1) each) until y' > y+δ
        -> returns rectangle + horizontal band on its left (green), still potentially many
    => KEY: a green point (x' < x-δ) is useless forever (future points have larger x, δ only decreases) -> DELETE it when touched
        (a blue point may be needed later)
-> cost: range search O(log n) per point + <= 8 (6) points in the rectangle + every point inserted once and deleted at most once => θ(n log n)
!! build the BST step by step (insert a point when processed): inserting all at the start would also return points on the right
!! the bound on the rectangle is needed: those points are not deleted
