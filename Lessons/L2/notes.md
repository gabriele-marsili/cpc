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

•Priority queue <- max heap 
    store a set of values 