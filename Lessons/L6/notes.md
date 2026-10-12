[L5]
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
    • correctness: the repetition of the majority el "kills" the other elements following the sequence during the algo

[L6]
-> Supporting insertions and deletions:
OSS: appending new el. does not broke the algo (it support dynamic inestions)
    -> Boyez-Moore does not support dynaimc deletions because we're not taking care about other elements ≠ by the candidate 
With a dynamic set, Boyer–Moore does not work (it cannot undo). 
Idea: for each bit position keep how many elements have a 0 and how many a 1; the majority element is in the majority in every bit, so its i-th bit is the most frequent value in position i. 
    -> In the example: bits give 001 = 1; after deleting five 1’s and inserting a 3 the counts give 011 = 3. O(log u) time per operation and O(log u) space, with u the largest element. 

(Boyez-Moore that supports deletions)
-> Example:
    -> idea: use binary representation of numbers and use log n counters 
A [5,3,1,1,2,3,1,1,1]
A [010, 011, 001, 001, 010, 011, 001, 001, 001]

-> tab: keep counting of #0s, #1s for each (position) row
    8 0s vs 1 1s ; 
    6 0s vs 3 1s ; 
    1 0s vs 8 1s ;
-> keep bit of the majority el: 
    [0,0,1] => 001 => 1

OSS: for each element we need to go over it's bynary repr and increase the counter
-> θ(n log n) time, θ(log n) space (u = max in A)
-> deletions: just reduce counter by 1 (for each deletion) <- supported


•Misra–Gries heavy hitters (network, keep track of the senders that congests the network, by ip of source/dest)
-> Generalization: given A[1,n], find the T-1 elements that occur at least n/T + 1 times (at most T-1 of them). 
-> Keep a set K of at most T candidates with counters: for each e, if e ∈ K increase its counter, otherwise insert it with counter 1; if now |K| > T , decrease all counters and delete the zeros.
-> for e in A:
    if e in K:
        K[e] += 1
    else:
        K[e] = 1
        if len(K) > T : 
            for k in K.keys():
                K[k] -= 1
                if K[k] == 0:
                    del(K[k])

-> Running time: the naive analysis gives O(T · n), but it is Θ(n): the total cost of the “decrease all” loops cannot exceed the number of elements of the array (each decrement cancels a previous increment, and there are n increments). 
With T = 2 it is exactly the majority element problem.
    -> [extra] Candidates may include false positives: a second pass to count them gives the exact answer.


Puzzle: dominoes on a chessboard:
8 × 8 chessboard with two opposite corners removed: can it be covered with dominoes (2 × 1)? 
    ->No: each domino covers one black and one white square, but the two removed corners have the same colour, so 30 squares of one colour and 32 of the other remain.


--------------------------------------------------------------------------------

• Problem:
given a track that begin by the beginning of a track, in the track there are several points, the lats one is the goal. We know the distance between the points and the beginning and the first point: d1, d2,..., dn
The car is spending a ratio of fuel for each km <- F = fuel/km
At each point f_i is possible to refil the carburant (tank) of the car by f_i liter (f1...fn)
compute G* = min amount of fuel in our tank at the begin 

• Tecnique <- Binary search for the answear :
considering BS at step i, I can see the array as an answear array with each pos with a bool value T/F:
[T,T,T,T,...,T,T,T,F,F,F,...,F]
<- I'm interesting in the position in which there is the change T->F

• Social Distancing:
we've not overlapping intervals with different lengths, starting by different positions (from start to last point l).
Given c people to place in this n intervals.
Problem: maximize the min distance <- d*
    -> distance d > d* |-> F
    -> distance d < d* |-> T
    start from l/2, then divide iteratively -> l/4, l/8 etc 
    => log l iteration 
    how much time to evaluare if a distance d is possibile? start by the starting point of the first segment, then put at distance d next person, if it is not in a segment then put it at the start of the next segment <- linear time 
    => n log l time 
