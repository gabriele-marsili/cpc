[L7] (lecture of Wednesday 07/10/2026)
=> checklist so far: standard data structures + binary search for the answer + two pointers trick (+ sweep line)

• End of binary search for the answer (from L6)
-> social distancing: greedy check for a target distance d (left to right, place a person, jump d, if outside an interval -> first point of the next interval)
    -> check: θ(n + c) (n intervals, c people); every BS step removes half of the remaining candidate distances
    => time O((n+c) log l)
    -> assumes d integer! if d is real -> # of steps depends on how many decimal digits we want
    -> better range: d* <= l/c (c people on a line of length l) -> binary search on [0, l/c]
-> car and fuel (left open in L6): G* = min amount of fuel in the tank at the beginning
    -> "we are asked for an answer" => BS for the answer
    -> monotone: less fuel than G* -> fail, more -> arrive
    -> predicate "can I reach the end with G?" = simulate: each segment -F*d_i (if < 0: false), each point +f_i => θ(n) per check
OSS: general techniques give A solution (standard, "almost brainless", usually what an interviewer asks for), not necessarily the optimal one: an ad hoc solution exploiting the problem can be better
    (here: G* = max over the prefixes of the deficit, θ(n) — extra)

• Puzzle: mutilated chessboard (top-left and bottom-right removed) -> cover with 2x1 dominoes?
-> NO, parity argument: the 2 removed cells have the SAME colour; every domino (horizontal or vertical) covers 1 white + 1 black
    -> #white != #black at the start, each domino lowers both by 1 -> they can't both reach 0 (general: any board with #white != #black)
-> follow-up (for L8): remove 1 black + 1 white (any two) -> is a tiling always possible? if yes, an efficient algorithm to find it?

• Two pointers trick
-> not sophisticated, but always keep in mind that 2 pointers can solve the problem
• 2 indices: i and j <- move only forward over the array 
    => θ(n) steps overall (replaces a θ(n^2) scan of all pairs (i,j) when the property is monotone)
    • warm up (extra, not done in class): pair with sum = target in a SORTED array, i = 0; j = n-1
        -> if a_i + a_j < target : i++ |-> no pair with a_i works (a_j = largest remaining value)
        -> if a_i + a_j > target : j-- |-> no pair with a_j works
        => θ(n) time, O(1) space 

• Longest k-good segment (Codeforces 616D, MANDATORY)
-> Given A[1,n] of values and a value k, a subarray is called k-good if it contains no more than k different values : 
    ex: [4,2,3,1,2,2,3,4] => [2,3,1,2,2,3] is 3-good 
-> find (a) largest one 
    A : [1,1,2,3,3,4,4,1,3,2,3] ; k=3  (example of the 2025 notes)
        -> [1,1,2,3,3]
        -> [3,3,4,4,1,3]
-> trivial solutions:
    -> all θ(n^2) subarrays, scan each one => cubic
    -> like the max subarray (L1): fix i, extend j one position at a time keeping a multiset (set + # occurrences) => θ(n^2)
––> sol with 2 pointers : 
    keep window [l,r] that's always k-good and dict. H:value|->occurences in the window (size = # of distinct values)
    • move r one step to the right and add A[r] to H 
    • while H has more than k values, remove A[l] (delete the key if its count = 0, otherwise size of H is wrong) and move l right
    • now [l,r] is the longest k-good window ending at r <- compare its length with the best so far 
-> correctness: if [l,r] is not k-good no window [l,r'] with r' >= r is k-good, so l can be discarded forever (=> monotonicity)
-> example (2025 notes): k = 3, A = [1,1,2,3,3,4,4,1,3,2,3], 0-based positions
    -> up to r=4 : [1,1,2,3,3] has 3 values 
    -> r = 5 => value 4 enters => l moves past both 1's and 2 => counter of 1s: 2->1->0 => 1 deleted 
    -> r = 9 => the 2 forces l forward again 
    -> answer: positions 3..8, length 6
=> each pos enters and leaves the window once => θ(n) dictionary op. 
    -> hash map => θ(n) expected time 
    -> BST => θ(n log n)
-> 2 pointers as in class: L, R start at the beginning
    -> move R: if A[R] already in the set -> freq++, otherwise insert it
    -> if the set has k+1 distinct values -> move L (decrement freq, remove the value at 0) until k again (may skip a whole run of equal values)
    -> keep the MAX R-L+1 seen so far and report it at the end
    !! the pointers do NOT stop on the optimal window: ex. [4,4,4,4,1,5,5,5,5], k=1 -> when L,R are on the 4s you can't know how many 5s follow => keep the best so far
-> correctness (prof): L = beginning of a candidate, R extends it as much as possible; when R adds a new distinct value no subarray starting at L is k-good anymore => implicitly we try all possible L
-> time θ(n): at every iteration one of the 2 pointers moves right by 1, both only move right => <= n + n iterations

• Palindrome check: one pointer at the beginning, one at the end, compare and move inwards; they meet before a mismatch => palindrome

• Alternative (student): binary search for the answer on the LENGTH
-> "find the longest one" hides an answer: the length L; known L -> sliding window of size L (O(1) to remove one elem and add the next) finds a k-good subarray
-> monotone: k-good of length L exists => shorter ones exist too
-> θ(n) per check => θ(n log n) total (more precisely θ(n log(n - k)): answer >= k); slower than 2 pointers
-> in a real implementation: early exit at the first k-good window
-> example in class: A = [1,1,2,3,3,4,4,7,3,2,3], k = 3, L* in [1, 11]
    -> L' = 6 NO, L' = 3 yes, L' = 4 yes, L' = 5 yes => 5 (prof doubted it at the end)
    !! WRONG: [3,3,4,4,7,3] (pos 4..9, 1-based) has 3 distinct values -> L' = 6 is YES; all windows of length 7 have 4 => L* = 6 (only the first check was wrong)

• Sweep line ("very popular": conceptually simple, sometimes hard to see why it works)
-> Maximum number of overlapping intervals
    -> n intervals [s_i, e_i], s_i, e_i in N; two intervals overlap if ∃ x s.t. x in [s_i, e_i] AND x in [s_j, e_j]
    -> compute the max number of intervals overlapping all together in some point (example on the board: 4)
-> array of counters on [0, l] (l = θ(n)): +1 for every point of every interval
    !! filling it costs the sum of the lengths => quadratic even with l = θ(n) (many long intervals)
    !! scanning 0..l and checking all intervals for each point => quadratic too
    -> with prefix sums the increments are O(1) offline -> later lecture
-> sort the intervals by start, keep only the active ones -> still wasteful: most positions have the same counter as the previous one
    => the counter changes only where a segment STARTS or ENDS
=> SWEEP LINE: each interval -> 2 events (s_i, start), (e_i, end); sort them; process in order: start -> counter+1, end -> counter-1; answer = max counter
    -> a vertical line sweeps the axis, the counter = # intervals intersecting it; we stop only at the important points
    -> time θ(n log n) (dominated by sorting the endpoints)
    -> ties (extra): closed intervals -> at the same coordinate starts BEFORE ends ([1,3] and [3,5] overlap in 3)
-> same thing as balanced parentheses: "(" for each start, ")" for each end, in sweep order
    -> answer = max depth (max # of open, not yet closed parentheses) -> it's exactly the counter (+1 = open, -1 = close)
    -> ex. board: ((((  )(  )))  ()()()() )  depths 1 2 3 4 3 4 3 2 1 2 1 2 1 2 1 2 1 0 => 4

• Misc
-> Rust challenges on the course page: shortest possible Rust program (map/filter/count chains)
-> next lecture (L8): closest pair of points (sweep line) + a problem with many BSTs (Frogs and Mosquitoes: not done in L8 either -> L9/notes.md)
