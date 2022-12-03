#!/usr/bin/env python
from itertools import takewhile

def sum_calories(lines):
    totals = []
    it = iter(lines)
    while True:
        total = sum(int(n) for n in takewhile(bool, it))
        if not total:
            break
        totals.append(total)
    return totals

def main():
    with open("input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
    totals = sum_calories(lines)
    totals.sort(reverse=True)
    print(f"Three elves with the most calories: {totals[0] + totals[1] + totals[2]}")

if __name__ == "__main__":
    main()
