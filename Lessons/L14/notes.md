What's the time complexity of RangeSum?
    -> how many nodes do we process? 

    Possible cases: 
    - partial overlap:        
        -> query interval is at the left of the center of the interval [easy case]
        -> query interval is at the right of the center of the interval [easy case]

        in easy cases one of the 2 branches stops immidiatly due to recursion

        -> query interval is in center of the interval : both to left and right to it 
            => go both left and right 
            -> cause we divide in the center then we obtain intevrals in which the left/right extreme coindice with the right/left extreme of the query range 
                => that implies at least one total overlaps with the center when we divide again 

    - total overlap:
        max 1 check -> one visited node 
    - no overlap:
        max 1 check -> one visited node 


    so total cost is at most θ(4*log n) that can became θ(2*log n) anticipating check before recursive calls 
        -> that a cause of the structure of the tree 


------------------------------------------------------------------------------------------------------------

[Diinamic range Minimum queries] (easy)
- Add(i,v)
- RMQ(i,j) = argmin (A[i,j])
    -> position of the min element in the range [i,j]

use segment tree in which in all the nodes there is the min of the range 
    RMQ => same algo as before 
        -> with the prev. ex. we reach -2 and 6 for query [3,6] 
        -> store pair (pos,value) or store pos and use it to get the value 

    Add(i,j) may imply to recompute the min up to the root to keep consistency 
        -> ofc there is possibility of early stopping in case the changed node is not the min 


------------------------------------------------------------------------------------------------------------

[Range Minimum Queries with occurrencies]
- Add(i,v)
- query(i,j) = report the min value in range [i,j] with it's # of occurrencies in A[i,j]

    -> for each node keep a pair (min, counter) and when 2 values are both the min increase the counter (ofc use Segment trees) and keep the counter updated :
        to update the counter we keep the counter of the min and in case of intermediate nodes we sum the counters of the childrens (if the value is the same and it's the min)
    -> query with the same algo as before 
    -> Add => recompute and update going upwords to the root 


------------------------------------------------------------------------------------------------------------

[Problem]
A[1,n] with not-negative integers 
- Add(i,v)
- RangeSum(i,j)
- Search(s) : report the position i of the shortest prefix of A (or the smallest pos i s.t. sum(A[k]) for k=1 to i >= s)

example:
A = [3,1,0,4,5,13,2,0]
pos 0 to 7 
Search(22) = 5

-> use Segment Tree and go left as possibile at each node if node value >= s, keep what we've found so far and update the query (subtracting only when we go right)
    -> reaching a leaf => 2 cases: 
        - value of the leaf > what we're looking for 
        - value of the leaf == what we're looking for 
    -> θ(log n) time
> OSS: is possible to use binary seach for the answear : θ(log n) steps, log(n) for each step (but less space)
    -> θ((log n)^2) time


------------------------------------------------------------------------------------------------------------

[Problem]
A[0, n-1] of integers 
- Add(i,v)
- #occs (i,j,x) : quantity occurrencies of value x in range  