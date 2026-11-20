# Sparse table: the PROF's code, copied verbatim from his board (lecture L16, 18/11/2026,
# page "Sparse table: the code and an example", recording L_16.MP4 at ~16:40).
# Only the comments marked "##" and the example at the bottom are mine (Claude).
#
## T[i][k] = POSITION of the minimum of A[i .. i + 2^k - 1]  (argmin, leftmost on ties because of <=)
## build: Theta(n log n) time and space.  rmq: O(1), both ends included.

def build(A):
    n = len(A)
    T = [[i] for i in range(n)]          # T[i][0] = i
    k = 1
    while (1 << k) <= n:
        h = 1 << (k - 1)
        for i in range(n - (1 << k) + 1):
            a, b = T[i][k - 1], T[i + h][k - 1]
            T[i].append(a if A[a] <= A[b] else b)
        k += 1
    return T                             # T[i][k] = RMQ(i, i + 2^k - 1)

def rmq(A, T, i, j):                     # both ends included
    k = (j - i + 1).bit_length() - 1     # largest 2^k <= j-i+1
    a, b = T[i][k], T[j - (1 << k) + 1][k]
    return a if A[a] <= A[b] else b


if __name__ == "__main__":
    ## the example of the lecture: A = [5,2,7,3,8,1,6,4], RMQ(0, 6)
    A = [5, 2, 7, 3, 8, 1, 6, 4]
    T = build(A)
    ## rows are ragged: row i has an entry for every k with i + 2^k - 1 <= n-1 (the grey cells are skipped)
    for i, row in enumerate(T):
        print(i, [A[p] for p in row])    ## values, as the prof wrote them in the table
    p = rmq(A, T, 0, 6)                  ## k = 2, T[0][2] -> value 2, T[3][2] -> value 1
    print("RMQ(0,6) = position", p, "value", A[p])
    assert (p, A[p]) == (5, 1)
    ## brute-force check on all ranges
    import random
    for _ in range(200):
        n = random.randint(1, 40)
        B = [random.randint(-5, 5) for _ in range(n)]
        TB = build(B)
        for i in range(n):
            for j in range(i, n):
                q = rmq(B, TB, i, j)
                assert B[q] == min(B[i:j + 1])
    print("ok")
