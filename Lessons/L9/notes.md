[L9] (provisional — Frogs and Mosquitoes not explained yet)
-> announced at the end of L7 ("a problem that needs a lot of BSTs"), not done in L7 nor in L8 (prof ~1 hour behind)
-> from my L7 notes = prof's 2025 notes (Binary_search_2025.pdf p.10-13) -> check it against the lecture

• Frogs and Mosquitoes
-> Codeforces 609F, MANDATORY
-> given a x-axis and n frogs on it, each frog (i) has a value t_i which is the length of it's tongue (x_i |-> pos the i-th frog on x-axis). 
-> There are m mosquitoes (j) start landing on the x-axis -> j lands at pos b_j and it has a value a_j
-> a frog i eats a mosquito j in pos b_j if b_j in [x_i, x_i+t_i]
    -> when i eats j its tongue grows: t_i = t_i + a_j
-> if there are many frogs able to eat b_j only the leftmost one eats it 
OSS: a mosquito j in pos b_j may be not eaten for a while 
• output: for each i frog i: # of mosquitoes it ate and its final tongue length 
• Target: time complexity θ( (n+m) log (n+m) )
    -> dynamic 1D point location
**Solution**
- store x_i in a BST with t_i. The candidate frog for b_j is the predecessor of b_j (largest x_i <= b_j):it eats the mosquito if b_j <= x_i + t_i <- θ(log n)
> issue #1 : overlapping intervals -> the predecessor is not necessarily the right frog, a frog x_h further left with a long tongue (x_h < x_i and t_h > t_i) may also reach b_j and it has priority! 
    -> preprocess the interval to remove overlaps: scanning the frogs by x, frog i keeps only the part of [x_i, x_i+t_i] not covered by the frogs on its left (its start moves after their end, if it's covered entirely it is removed: it will never eat!)
    => now the intervals are disjoint and the predecessor is the right one
    -> sorting: θ(n log n)
> issue #2: A interval is growing to right->we're introducing new overlaps. 
    => Repeat:  
        - move the next frog
        - remove it if it is now totally overlapped, otherwise update its starting point (just after the new end) and stop 
    -> each step: θ(log n)
    -> a frog is removed at most once, so total cost is θ(n log n) (amortized: only the last frog visited is not removed)
> issue #3: not eaten mosquitoes. Store them in a BST by position. After frog i grows, (re)check whether it can eat an old mosquito: the first waiting mosquito at pos >= the start of its interval and <= x_i + t_i (a successor query); eat it, grow, and check again.
    -> each mosquito is inserted and removed once: θ(m log m)
-> total: θ( (n+m) log (n+m)) time with 2 BSTs
