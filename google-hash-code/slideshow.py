h = {}
vi = {}
for _ in range(int(input())):
    p = input().split()
    if(p[0]=='H'):
        h[_] = set(p[2:])
    else:
        v[_] = set(p[2:])

        
v,nv = [],[i for i in h.keys()]
m = 0
for i in h.keys():
    if(len(h[i])>m):
        m = len(h[i])
        top = bottom = i

v = [top]
nv.remove(top)
l = len(nv)
for i in range(l):
    m,s = 0,nv[0]
    for j in nv:
        p = min(len(h[j]),len(h[j]&h[top]))
        if(p>=m):
           m = p
           s = j
    top = s
    v.append(top)
    nv.remove(top)
print(len(v))
print(*v, sep = '\n')
print(*vi, sep = '\n')
