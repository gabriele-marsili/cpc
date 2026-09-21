A = [1,2,3,1,4,5,2,3,1]
k = 3

def sliding_w_max(A,k):
    sol = []
    for i in range(0,len(A)-k+1):
        m = A[i]
        for j in range(1,k):
            if A[i+j] > m : 
                m = A[i+j]
        
        sol.append(m)

    return sol


def main():
    s = sliding_w_max(A,k)
    print(f"Solution:\n{s}")

if __name__ == "__main__":
    main()