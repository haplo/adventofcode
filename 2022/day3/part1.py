#!/usr/bin/env python

ORD_a = ord('a')
ORD_A = ord('A')
LOWERCASE_BASE_PRIORITY = 1
UPPERCASE_BASE_PRIORITY = 27


def priority(item: str) -> int:
    if 'a' <= item <= 'z':
        return ord(item) - ORD_a + LOWERCASE_BASE_PRIORITY
    elif 'A' <= item <= 'Z':
        return ord(item) - ORD_A + UPPERCASE_BASE_PRIORITY
    else:
        raise ValueError(f"Invalid item: {item}")


def find_repeated(rucksack: str) -> str:
    second = set(rucksack[len(rucksack) // 2:])
    for char in rucksack:
        if char in second:
            return char
    raise ValueError(f"No repeated character in rucksuck: {rucksack}")


def main():
    with open("input.txt", "r") as f:
        rucksacks = [line.strip() for line in f.readlines()]
    total = sum(priority(find_repeated(rucksack)) for rucksack in rucksacks)
    print("Total of priorities:", total)


if __name__ == '__main__':
    main()
