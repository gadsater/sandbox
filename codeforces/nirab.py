li = []
r = 0
for _ in range(int(input())):
    l, r = map(int, input().split())
    li.append(l)
li.append(r+1)

k = int(input())
cnt = 0

for _ in li:
    if k < _ :
        cnt += 1

print(cnt)
