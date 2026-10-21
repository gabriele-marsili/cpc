[Fenwick tree 2026] (Binary indexed tree, BIT)
> lvl i has i bits to 1 => last lvl max log n bits to 1

Sum(7) : start by node with pos 7 + use nodes to cover the inerval 1-7
    -> [7,7] , [5,6] , [1,4] 
    -> Sum(7) = 2+4+9 = 15
    positions: 
    7 |-> 0111
    6 |-> 0110
    4 |-> 0100
    0 |-> 0000
    always cancel out the least significant bit a cause of the structure of the BIT
        -> child to parent: remove least significant bit
OSS: direct links are not necessary to be stored -> structure of the tree can be not stored cause we just need computation for links 
    
parent(i) = i - (i & -i)
    -> take into account only the last significant bit set to 1
        -> 0111 -> 0001
        -> we can isolate it splitting nums (32 bits) into blocks (of 8 bits) then store in byte in a table (that's not necessary in our case)
        considering i and -i I can isolate the least significant bit set to 1 using logical and (-i obtained with 2 complement)

Sum(i){
    sum = 0
    while(i != 0){
        s + A[i]
        i -= (i && -i)
    }
    return sum
}
 
Complexity is θ(log n) in time due to # of lvls

Add(5, 10) -> add 10 to every node that contains pos 5
    -> right sibling then right sibling of my parent then right sibling of grandparent etc 
    5 |-> 0101
    6 |-> 0110
    8 |-> 1000

        example: 1010000
        children: 1010001, 1010010, 1011000
        right siblings: least significant bit set to 1 moved one pos to the left
            => select least significant bit and then sum it 
            -> I don't need the check more than one time, I've just to keep adding the least significant bit to current pos 
    
    i += (i && -1)

Add(i,v) : 
    while i < n:
        H[i] += v
        i += (i && -1)

Complexity: θ(log n * log n)
    -> but least significant bit is moved only 1 pos to the left so θ(log n)



Counting inversions 
Given A[1,n] of positive numbers <= n, count the # of inversions in A
    -> an inversion is a pair of pos (i,j) s.t.
        • i < j and A[i] > [j] 
        • [3,2,1] => (3,2), (2,1), (3,1) -> (n-1)! inversions 
    -> a possibility is to use insertion sort -> # of steps ≈ number that's the output
Example: A[3,2,1,6,5,4]
    inverions: (3,2), (2,1), (3,1), (6,5), (6,4), (5,4)
Standard approach: variant of mergesort θ(n log n)
    -> in the merge procedure mantain a counter increase it every time i move a number in the final (partial) array 
        counter += 1 for each number > moved element at the left of the moved element 
    -> running time: θ(n log n)

