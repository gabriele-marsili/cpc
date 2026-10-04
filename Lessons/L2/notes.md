Lezione lunedì 21/9/26

Sliding Window Maxima: 
Given an array A[1,n] and an integer k 
-> find the larget element of each window of size k 
es:
A 1,2,3,1,4,5,2,3,1 ; k = 3

max window:
1) [1,2,3] -> 3
2) [2,3,1] -> 3
3) [3,1,4] -> 4
4) [1,4,5] -> 5
5) [4,5,2] -> 5

Naive solution Θ(n*k) :
sol = []
for i = 0 to n-1 
    m = A[i]
    for j=1 to k-2:
        if A[i+j] > m : m = A[i+j]
    
    sol.append(m)

OSS: when sliding the first element is deleted, a new one is appendend :
    -> multiset in which I remove first and add at the end 
    -> mantain the max element 

=> using a multiset (with k elements):
    • Remove an element 
    • Insert an element
    • Report the max 

• (max) Priority queue <- max heap (every node has a key >= childrens => max |-> root)
    store a set of values and support:
    - Max(S) : return the max in S : O(1)
    - Insert(x,S) : insert el x in S : Θ(log(S))
    - ExtractMax(S) : removes the max from S : Θ(log(S))
-> OSS: delete an arbitrary el is difficult cause it's position is necessary, so it must be searched in the tree or be mantained -> no integration of general delete 

Points [general framework]: 
1) undestand the problem 
2) write a naive sol 
3) understand characteristic of the problem <- define which operations are required 
4) improve the sol using appropriate Data Structure (appropriate => DS that support all the operations and only the one required, not necessary to use a BTS for dictionary problem for example)
5) identify the properties, running time etc of the approach used 

• Big question: which datastructure supports this operations?

OSS : we cannot delete arbitrary element using max heap
-> we've the sliding window, so we can integrate our version of the max heap 
    -> in each cell of the sliding window we store a pointer to the related node in the (max)heap 
    -> deletion is allowed cause we've pointers for each node, so we can just delete 
    -> this solution is not good, too complex, not efficient 
        -> even if the complexity if O(n*log(k))

Is possibile to support the requested operation with a standard heap?
    -> so, is there a way to let the maxheap be able to support deletion?
    1) not remove elements
    2) get the max 
    3) is max is outside the window then remove it (safely, forever)
        -> we can store the index/auxiliary info (value of the element as key of heap, position)
    > runnig time: 
        - log n time of each op (insertion, extract etc)
        - n insertion 
        - n-k deletions 
        - at most n-k extraction of the max for each window 
        => n*(n-k) ≈ n^2
        -> each elem is inserted only one time and extracted only one time, so the # of extraction is max n <- bound the quantity (number) of iteration

-> summing up: [Lazy delete] :
we store pairs <value, position> in which positions is required to understand if the value is in the window 
At every it:
    - insert the new element 
    - extract the current max and loop this until the current_max is in the window :
    while curr_max is not in the widow :
        - extract current max 
    - report curr_max 

Time complexity: (# insertions + #extractions) log n 
#insertions = n
#extractions <= #insertions = n
Overall complexity : Θ(n*log n) time



---------------------------------------------------------------------------------------

DataStructures <-> problem :
• Heap <-> priority queue
• Binary Search Trees <-> (ordered) predecessor
• HashMap <-> dictionary problem 


• Predecessor Problem ((B)BST) <- balances binary search trees : 
[digression]
-> for B-Trees B-1 keys are required -> log b to find the children with binary search => log B (n)* log B -> reduce the quantity of I/O
-> BT: log n random cache line loads 
-> B-tree: fewer cache lines cause I'm using more than 1 el per line 

Store a set S, supporting:
    - Insert(x,S)
    - Delete(x,S)
    - Search(x,S)
        -> those operations define the dictionary problem (|->hash map)
    - Max/min (min|->always left in the tree / max|->always right <- not necessary leafs)
    - Successor(x)
    - Predecessor(x) |-> max {y | y in S && y<=x}
> All op. in Θ(log(n)) ; n = |S|

Sol. with BTS: 
    -> trivial sol, store the window, insert/reomve in it, use max to compute max 
    -> SWM in Θ(n log k) time