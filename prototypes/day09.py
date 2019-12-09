with open('input/day09.txt') as f:
    numbers = [int(i) for i in f.read().strip().split(',')]

print(max(numbers))
print(min(numbers))
