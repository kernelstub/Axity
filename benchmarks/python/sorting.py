#!/usr/bin/env python3

def bubble_sort(arr):
    n = len(arr)
    for i in range(n - 1):
        for j in range(n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
    return arr

arr = [64, 34, 25, 12, 22, 11, 90, 88, 45, 50, 33, 77, 99, 18, 7]

bubble_sort(arr)

print("Sorting completed")
print(f"First 5: {arr[0]}, {arr[1]}, {arr[2]}, {arr[3]}, {arr[4]}")
