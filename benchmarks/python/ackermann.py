#!/usr/bin/env python3

def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

def power(base, exp):
    if exp == 0:
        return 1
    return base * power(base, exp - 1)

result = factorial(10)
print(f"Factorial(10) = {result}")

result = factorial(12)
print(f"Factorial(12) = {result}")

result = factorial(15)
print(f"Factorial(15) = {result}")

result = power(2, 10)
print(f"Power(2, 10) = {result}")

result = power(3, 8)
print(f"Power(3, 8) = {result}")

result = power(5, 7)
print(f"Power(5, 7) = {result}")
