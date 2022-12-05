#!/usr/bin/env python
import re
from dataclasses import dataclass
from itertools import takewhile
from typing import List


@dataclass
class Move:
    quantity: int
    start: int
    end: int

    @classmethod
    def from_str(cls, input:str):
        parts = input.split()
        quantity = int(parts[1])
        start = int(parts[3])
        end = int(parts[5])
        return cls(quantity, start, end)


def execute(stacks: List[List[str]], op: Move):
    from_stack = stacks[op.start - 1]
    to_stack = stacks[op.end - 1]
    for i in range(op.quantity):
        to_stack.append(from_stack.pop())


def parse_stacks(lines: List[str]) -> List[str]:
    # lines should be just the stacks and the numbers
    num_stacks = int(lines[-1].split()[-1])
    stacks = [[] for i in range(num_stacks)]
    for line in lines[-2::-1]:
        # look only where the letters should be
        for i in range(0, num_stacks):
            if (letter := line[1+i*4]) != ' ':
                stacks[i].append(letter)
    return stacks


def main():
    with open("input.txt", "r") as f:
        lines = f.readlines()
    # separate lines between stacks and moves
    for n, line in enumerate(lines):
        if not line.strip():
            stacks = parse_stacks(lines[:n])
            moves = (Move.from_str(line) for line in lines[n+1:])
            break
    for move in moves:
        execute(stacks, move)
    print('Top of each stack:', ''.join(s.pop() for s in stacks))


if __name__ == '__main__':
    main()
