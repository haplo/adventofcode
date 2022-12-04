#!/usr/bin/env python
from dataclasses import dataclass
from typing import Tuple


@dataclass
class Range:
    start: int
    end: int

    @classmethod
    def from_string(cls, input: str):
        start, end = input.split('-')
        return cls(start=int(start), end=int(end))

    def overlap(self, other) -> bool:
        return ((self.start <= other.start <= self.end) or
                (self.start <= other.end <= self.end))


def parse_pair(input: str) -> Tuple[Range, Range]:
    first, second = input.strip().split(",")
    return Range.from_string(first), Range.from_string(second)


def main():
    with open("input.txt", "r") as f:
        pairs = [parse_pair(line) for line in f.readlines()]
    total = 0
    for (first, second) in pairs:
        if first.overlap(second) or second.overlap(first):
            total += 1
    print("Number of pairs where one range overlaps the other:", total)


if __name__ == '__main__':
    main()
