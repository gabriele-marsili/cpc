Proof of the strategy of 100 prisoners puzzle:

The drawers are numbered as well 
the strategy is as follow:
- Prisoner i enter the room and opens drawers numbered i with number d_i inside 
    -> if d_i == i => he wins 
    -> otherwise he opens drawer d_i and continue 

OSS : if we do not stop after 50 steps, the prosoner is guaranteed to find his number 

drawers 1 2 3 4 5 6 7 8 
content 7 4 6 8 2 3 5 1 

prisoner 1 opens drawer 1, finds el 7, then open drawer 7, finds el 5, open drawer 5 and finds 2, open 2 and finds 4, open 4 and finds 8, open 8 and finds 1, open 1 and finds 7, then stop 
    -> that define a cycle 
    -> prisoner wins if he close the cycle before 50 steps 
        -> in that case he and all other prisoners in the cycle win 

P(all win) = P(random permutation has no cycle > 50 steps)

how much permutations have a cycle > 50? 
-> number of permutation (of 100) with cycles of length > 50:
for l > 50, only 1 cycle of length l 
(100 
  l) -> elements of the cycle 

(100 su l) * (l-1)! (100-l)! = 
= 100! / l
Total number of permutations with a cycle of length l > 50:
summation l=51 to 100 (100!/l) = 100! * summ l=51 to 100(1/l) = 
= 100! (1/51 + 1/52 +...+1/100) 
= 100! (H_100 - H_50); H_n = n th harmonic number 

P(all win) = 1 - (H_100 - H_50) ≈ 0.3118 
≈ 0.3069 = 1 - ln 2

the point is that the prob of winning of one prisoner is combined with the other prisoners cause if one of them wins in a cycle then all the other ones in that cycle win.

------------------------------------------------------------------------------------------------------------


Duplicate element in an array:
We've an array A[0,...,n] of n+1 numbers in {0, ..., n-1}. (=> exists at least one duplicate by the pidgeons holes principle)

Goal: find any of them, say e 
Example: 
A : 0,1,2,3,4,5,6,7,8
   [1,3,5,6,5,2,0,4,7] e = 5

Solution #1
Use hashset H to store all the elements, scan the array and for each element check if it's duplicate, otherwise add it to H 
    -> stop as soon as current element is already in H 
    -> Θ(n) with high prob, but Θ(n) extra space

Solution #2
It's possible to use a Direct Access Table D (or bitvect) to replace H 
    -> still Θ(n), but faster 

Is constant extra space enough? 

Θ(log n) passes over A
Θ(log n) passes we "reconstruct" e bit by bit 
(assuming |A| is power of 2)

1) write A as binary rapresentation: [001, 011, 010, 110, 101, 010, 000, 100, 111]
2) first pass: check only the most significant bit -> reconstruct 
    invariant: pass i checks bit in pos i 
3) we've a bit more frequent than the other at earch scan <- that's the most significant one 
    => count frequencies: how many 0s, how many 1s (in the example: 4 zeros and 5 ones first scan => most significant bit in first pass is 1)
        -> it works only for the fist row
        -> second row we've to check elements 01 / 10 
        -> third row : 010 / 101
            > smaller istance of the previous problem at each scan 

Θ(n log n) time; O(1) extra space
    <- best if no random access is allowed 
    
does random access help?
Destroy A:
1) first scan of the previous solution -> find most significant bit -> discard the fist bit of all the element, use the discarded bit to store a binary vector that I use with the trivial solution only for the still considered elements (the ones with the most significant bit) 

MySOL:
for i = 0 to n+2:
    f = j = A[i] #j=1 | [1,3,5,6,5,2,0,4,7]
    k = A[j] #k=3 | [1,3,5,6,5,2,0,4,7]
    A[j] = j #A[1] = 1 | [1,1,5,6,5,2,0,4,7]
    j = A[k] #j=6
    A[k] = k #A[3] = 3 | [1,1,5,3,5,2,0,4,7]
    k = A[j] #k=0=i
    A[j] = j #A[6] = 6 | [1,1,5,6,5,2,6,4,7]
    j = A[k] #j=1 | [1,1,5,6,5,2,6,4,7]
    A[k] = k | [0,1,5,6,5,2,6,4,7]
    k = A[j] #k=1 <- k=f => i++ => i=1

    ----
    f = j = A[i] #j=1 
    k = A[j] #k=1 <- k=f => i++ => i=2 | [0,1,5,6,5,2,6,4,7]

    ----
    f = j = A[2] #j=5 
    k = A[5] #k=2
    A[j] = j #A[5] = 5 | [0,1,5,6,5,5,6,4,7]
    j = A[k] #j = 5
    A[k] = k #A[2] = 2 | [0,1,2,6,5,5,6,4,7]
    k = A[j] 

:

for i in range(len(A)):
    while A[i] != i:
        v = A[i]
        if A[v] == v:        # la cella v è già occupata da v → duplicato
            return v
        A[i], A[v] = A[v], A[i]   # manda v a casa sua