s1 = input()
s2 = input()

n = len(s1)
c = 0

for i in range(n):
    if s1[i] != s2[i]:
        c += 1

print(c)
