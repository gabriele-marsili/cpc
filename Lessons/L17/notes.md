• recheck for notes on RMA etc on Teams

----------------------------

1) Binary Parentesis 
    • pre order: open parentesis
    • then close parentesis
    -> parentesis are balanced 
        -> go recursively on right/left and close (open while descending the tree, close when ascending from leaves to the root)
    Subtrees in the tree corrisponds to sub strings in the parentesis : "(((())()))" for example in "(((())())(()))"
    
    find-open : 
        -> closed parentesis => leaf 
        -> open => first child 
    
    next child = right siblling of the child 
        -> use close parentesis 

    this rapresentation lose the order of the children when there is only one child for a node (we can't say if it's the right/left one)
        -> use notation: left/right for each open parentesis (=> closed / open left / open right) (no specification for the root)

2) LOUDS : 
    • store in unary the degree
        1 |->0
        01 |-> 1
        001 |-> 2

        -> binary string reading left ro right the rapresentation from root to leaves, from top to down 
        -> for each node: 
            one for the degree in the unary rapresentatoin 
            # of 0s depends by the degree 

        how can I recover the tree?
        001 001 01 01 1 1 1 
        # of children related to # of 0s so first 001 is the root with 2 children, then just make nodes in the first free spot at the top : an arch wihout the node 
            -> add the node and repeat the process 

        OSS: you can rapresent a node with 2 bits: leave (00), with right children (01), with left children (10), with both children (11)
            => 11 11 10 10 00 00 00 <- 2 bits per node

        

3) DFUDS 

Linear space and constant time solution (#4): (OSS: general approach that does not depends by this specific problem)
translate operation from RMQ to trees 
-> reduce size of blocks then trasform the array in a tree 
    => 1 query on the Array mapped into 2 queries in the tree
    -> syntax rappresentation: log n nodes , binary rappresentation of that tree => 2 bits per node 

block of x = log n numbers -> a tree of x = log n nodes = 2*x bits 
    # of trees <= 2^(2x) trees in total 
    with x = log n => n^2 possible trees 

considering x' = 1/2 log n => n possible trees 
x' = 1/4 log n => sqrt(n) possible trees
    -> now I can use a table (sqrt(n) * sqrt(n) = n) to store all the possibile trees 
    • first tree: 00000...00 => store all answers : (1/4 log n)^2 answers 
    • that for all possible trees => sqrt(n) * (1/4 log n)^2
        OSS: using x' < 1/4 just reduce the dim of the table (like 1/8 |-> n^1/4) but sqrt(n) is enough 
    
OSS : queries must depends only by the structure of the tree.

Cartesian tree :
binary tree which is built recursevely. Given an Array you choose the root and build the tree splitting it in subarrays and using recursion.
    Root = leftmost occurrence of the min 
        -> A = [2,1,1,3,2,2,1,2] => root = A[1], so index 1 
            root = A[1] = 1, then 
                A_left = [2]
                    => root = [2] => stop 
                A_right = [1,3,2,2,1,2]
                    => root = index 0 => 1 
                        => [] for left -> stop 
                        => A = [3,2,2,1,2] for right 
                            => root = A[3], so index = 3
                                => A_left = [3,2,2]
                                    => root = 2 = A[1], index 1
                                        => A_left = [3] => stop 
                                        => A_right = [2] => stop 
                                => A_right = [2] => stop 
                                    
The tree in each node store the min value of the related sub tree so we can answer a query just finding the lowerst common ancestor between nodes i a and j, 
    note that the lowerst common ancestor is a node that's necessary included in the range cuase we're ascending the subtree to the root, considering only elements between i and j (both included)
    OSS : the answer is the exact element that divide both subarrays cause it's consistent, so it's also the answer for the conjuction of the subarrays (min of a smaller subarray is also the min of a larget subarray that includes the smaller one by construction)

We can rapresent the cartesian tree with 2 bits per node and answer the query on the array, in the table just store the pos of the lowerst common ancestor.
    => θ(1), θ(n)

