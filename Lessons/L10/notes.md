Problem 1 : "Little girl and queries" (-> prefix sums)
-> given an array A[1,n] with n elements and a set of m queries called Q.
Each query is a RangeSum (i,j)
Our goal is to permute the values of the array to maximize the (sum) overall results of the queries of our set Q.
• Example:
A: [5,2,4,1,3] -> total sum = 15
Q: [(1,5), (1,2) ; (1,2)] => 15 + (2+4)+(2+4) = 27
A_π = [2,5,4,1,3] => 15+(5+4)+(5+4) = 33

1) trovare posizione che è nella maggior parte delle query 
    > intersezione tra insiemi (i,j) per ogni query 
2) mettere elementi con valore più alto di A in quelle posizioni 

Total time complexity : θ(n log n + m log m)

A: [5,2,4,1,3] -> 
frequencies (of each position respect to the # of queryes that touch that pos.): 1,3,3,1,1
-> 1,5,4,2,3
=> 15+9+9 = 33
Steps: 
1) count frequencies of every position 
    -> sweepline algo : θ(n log m)
    -> another possible approach : 
        A: [5,2,4,1,3] 
        Q: [(2,4), (1,3), (2,5)]
        build an array B such that it's prefix sum is equal to the freq array F : [1,3,3,2,1]
        Prefix sums already computed : 
        B_0 = [0,0,0,0,0]
        B_1 = [0,1,0,0,0] -> prefix sum => [0,1,1,1,1]
        -> pos after the end of a segment should contribute -1 => after the first segment : [0,1,1,1,0]
        then [1,1,1,0,0]
        then [0,1,1,1,1]
        => [1,3,3,2,1] = F
        Computing prefix sum later : 
        B : [1,2,0,-1,-1] => [1,3,3,2,1]
        θ(n + m)
    -> use 2 arrays (counting sort) :
    C_ends : [0,0,1,1,1]
    C_besins: [1,2,0,0,0]
    > B is an alternative arr to represent C ends and C begins

2) - sort A in decreasing order and then <- θ(n log n)
   - sort (freq,pos) <- θ(m log m)
   - assign highest value to most frequence position θ(n/2)


Problem #3 - "Number of ways"
Given A[1,n], count in **how many ways** we can split A in 3 parts, so that the sum of of the parts is the same 
example : A[1,2,3,0,3]
S = sum of all elements of A : -> target sum : S/3
    -> ranges (i,j) => RangeSum(i,j), if it's equal to T then we've to be sure that each split has sum = T 
compute left to right the prefix sum on A and mark with "+" all the positions in which the prefix sum = T 
    -> for each of this "+" pos we want to know how many possible way I can spit in 2 parts the remaining part s.t. each part has sum = T
        -> scan right to left and mask as a red point each pos s.t. that suffix sum = T 
    
    -> then scan left to right A and for each "+" sum all possible pairs ("+", red point) in with red point is at the right of the "+" to the final result 
        -> how many red point to the right of a "+": suffix sum on them

[Fenwick tree 2026] (Binary indexed tree, BIT)
DataStructure to perform **dynamic prefix sums**
Problem : given A[1,n], we would like to support :
    - sum(i) = sum_k=1 to i A[k]
    - Add(i,v) = A[i] += v
        remember: RangeSum(i,j) = Sum(j) - Sum(i)
Trivial solutions:
- store A
    - Sum(i) : θ(n) time
    - Add(i,j) : O(1) time
- precompute prefixsums of A (and store them)
    - Sum(i) : θ(1) time
    - Add(i,j) : O(n) time

BIT suports both in θ(log n) time 
example: |A| = 8, A[3,2,-1,5,7,-3,2,1]
Relaxed version of the problem:
- Sum(i) : sum_k=1 to i A[k] -> only if pos i is a power of 2    
- Add(i,j) : A[i] +=v for any i
precompute:
    [3,5,null,9,null,null,null,16]
    sum : O(1); Add θ(log n)

fake root pos 0 with value 0, linked with all the != null value of precomputed arr 
    -> [1,1], [1,2], [1,4], [1,8] -> from i to j in [i,j] 
        binary pos: 0001, 0010, 0100, 1000 (only one bit set to 1 by construction)

    -> cover the "uncovered" part ([7,-3,2] in the example) re applying the same principle
        > consider [7,-3,2] with positions 1,2,3 (positions computed as original pos -4, in with 4 is pos of last el before sub arr)
        > sub problem: [7,4]
            ranges: [5,5] ; [5,6]
            binary: 0101, 0110 (only 2 bit set to 1 by construction)
        > connect that to the parent 
    
        > other sub problems:
            [-1] with pos 3
            binary : 0011
            range: [3,3]

        OSS: # of 0s at the end of the binary repr. = # of childrens

-> sum(i) : start from node with last range i and sum up values of it + it's parents to the root 
    -> runnig time:
-> add(i,j): re compute values of nodes with i in it's range 
    -> runnig time:
    
    