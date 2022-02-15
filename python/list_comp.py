arr = [  ]

for i in range(0, 20):
    if not i & 1: # If `i` is even
        arr.append(i)

# Can be converted into...

arr = [ i for i in range(0, 20) if not i & 1 ]

print(arr)