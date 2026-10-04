Random access approach: 
-> A |-> linked list in which cell i point to A[i].
    => no cell point to n (all values are < n ) => we never come back to n starting by n and following pointers
    -> infinetely many cells => cycle <- node with 2 incoming arrows (edges):
        • one from tail
        • one from the cycle 
        => it's a **duplicated value** :) 

    -> solution in θ(n) time, O(1) extra space (but doing so destroys A)
        mark every visited cell of A while destroying it, the first one reached twice is a duplicate :)
            [is possible to mark a cell writing value n in it cause n is not a valid value]

Floyd's cycle finding (solution in +O(1) space that's read only, it does not destroy A)
    • 2 pointers form the start: Slow (S) that moves 1 step and Fast (F) that moves 2 steps
    • phase 1) S and F meets inside the cycle (=> that assures that a cycle -a solution- exists)
    • phase 2) move F back to the beginning and set its speed to 1 
        => S and F will meet again at the beginning of the cycle = the solution 
    
    [Correctness: why it should be correct]:
    -> when F and S meets : S has moved m steps, F 2*m
    -> a = length of the tail, b = steps done by S inside the cycle <- m = a+b 
        => l: 2m = a+b+k*l (F has done the same steps as S plus k full hops of length l)
        => 2a + 2b = a+b+k*l 
            => a = k*l - b
    -> if phase 2, F walks a steps from the start and reches the beginnig of the cycle
        -> in the same a steps S (which was b steps past the beginning) goes around to b+k*l -b ≡ 0 <- the beginning of the cycle as well.


• Majority Element:
Given A[1, n], find (if any) the element that occurs at least n/2 +1 times. 
    - Sorting: Θ(n log n) time; 
    - hash map: Θ(n) expected time, Θ(n) space. 
Is Θ(n) time and O(1) space possible?

-> Boyez-Moore Algo:
    - candidate c
    - counter 
    - if A[i] = c, then counter++
        -> else counter--
        if counter == 0 : c=A[i] and counter=1

    • Example: A = [5, 3, 1, 1, 2, 3, 1, 1, 1]: at the end c = 1, counter 3. The counter is not the frequency of 1
    • Claim: if there is a majority element e, at the end c = e. 
        -> Why: every decrement pairs one element with a different one and “kills” both; an occurrence of the majority element can kill another element, but there are more occurrences of e than all the others together, so some occurrence of e survives.
    -> if the majority is not guaranteed, c can be anything (e.g. [1, 2, 3]): a second pass counting c is needed. Θ(n) time, O(1) space.

-> Supporting insertions and deletions:
With a dynamic set, Boyer–Moore does not work (it cannot undo). 
Idea: for each bit position keep how many elements have a 0 and how many a 1; the majority element is in the majority in every bit, so its i-th bit is the most frequent value in position i. 
    -> In the example: bits give 001 = 1; after deleting five 1’s and inserting a 3 the counts give 011 = 3. O(log u) time per operation and O(log u) space, with u the largest element. 


Misra–Gries heavy hitters
Generalization: find the elements that occur at least n/T + 1 times (at most T of them). Keep a set K of at most T candidates with counters: for each e, if e ∈ K increase its counter, otherwise insert it with counter 1; if now |K| > T , decrease all counters and delete the zeros.
-> for e in A:
    if e in K:
        K[e] += n
    else:
        K[e] = 1
        if len(K) > T : 
            for k in k.keys():
                K[k] -= 1
                if K[k] == 0:
                    del(K[k])

-> Running time: the naive analysis gives O(T · n), but it is Θ(n): the total cost of the “decrease all” loops cannot exceed the number of elements of the array (each decrement cancels a previous increment, and there are n increments). 
With T = 2 it is exactly the majority element problem.
    -> [extra] Candidates may include false positives: a second pass to count them gives the exact answer.

Puzzle: dominoes on a chessboard:
8 × 8 chessboard with two opposite corners removed: can it be covered with dominoes (2 × 1)? 
    ->No: each domino covers one black and one white square, but the two removed corners have the same colour, so 30 squares of one colour and 32 of the other remain.