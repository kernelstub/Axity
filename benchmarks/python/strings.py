#!/usr/bin/env python3

def reverse_string(s):
    return s[::-1]

def count_occurrences(haystack, needle_char):
    count = 0
    for char in haystack:
        if char == needle_char:
            count += 1
    return count

def simple_concatenate(base, addition, times):
    result = base
    for _ in range(times):
        result += addition
    return result

long_string = "Neutron programming language is fast and efficient. " * 100

reversed_result = reverse_string(long_string)
char_count = count_occurrences(long_string, "a")
concatenated = simple_concatenate("", "Hello World from Python! ", 50)

print(f"String ops: len={len(long_string)} rev={len(reversed_result)} count={char_count}")
