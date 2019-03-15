n, k = map(int, input().split())

if k == 1 or k == n :
    print(3*(n-2) + 6)
else :
    m = min(n-k,k-1)  
    print(4*m + 3*(n-m))

