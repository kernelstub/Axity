#!/usr/bin/env python3
numbers = list(range(1000))
print("Created array with 1000 elements")

sum_val = sum(numbers)
print(f"Sum of all numbers: {sum_val}")

even_count = sum(1 for x in numbers if x % 2 == 0)
print(f"Filtered {even_count} even numbers")

found = next((x for x in numbers if x == 500), -1)
print(f"Found element: {found}")

unsorted = [64, 34, 25, 12, 22, 11, 90]
sorted_arr = sorted(unsorted)
print(f"Sorted array: {', '.join(map(str, sorted_arr))}")

reversed_arr = list(reversed(sorted_arr))
print(f"Reversed array: {', '.join(map(str, reversed_arr))}")
