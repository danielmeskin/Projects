a=0
b=1
z=int(input("Number of iterations: "))-1
sequence=[1]
for i in range(z-1):
    n=a+b
    a=b
    b=n
    sequence.append(n)
print(sequence)
