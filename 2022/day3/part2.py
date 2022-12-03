#!/usr/bin/env python
from typing import Sequence, Tuple

ORD_a = ord("a")
ORD_A = ord("A")
LOWERCASE_BASE_PRIORITY = 1
UPPERCASE_BASE_PRIORITY = 27


def priority(item: str) -> int:
    if "a" <= item <= "z":
        return ord(item) - ORD_a + LOWERCASE_BASE_PRIORITY
    elif "A" <= item <= "Z":
        return ord(item) - ORD_A + UPPERCASE_BASE_PRIORITY
    else:
        raise ValueError(f"Invalid item: {item}")


def find_badge(group: Tuple[str, str, str]) -> str:
    first, second, third = set(group[0]), set(group[1]), set(group[2])
    badge = first.intersection(second).intersection(third)
    if not badge:
        raise ValueError(f"No badge for group: {group}")
    elif len(badge) > 1:
        raise ValueError(f"More than one badge for group: {group}")
    return badge.pop()


def iter_groups(rucksacks: Sequence[str]) -> Tuple[str, str, str]:
    it = iter(rucksacks)
    for i in range(0, len(rucksacks), 3):
        yield (next(it), next(it), next(it))


def main():
    with open("input.txt", "r") as f:
        rucksacks = [line.strip() for line in f.readlines()]
    groups = iter_groups(rucksacks)
    total = sum(priority(find_badge(group)) for group in groups)
    print("Total of priorities:", total)


if __name__ == "__main__":
    main()
