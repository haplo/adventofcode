#!/usr/bin/env python

from enum import IntEnum
from typing import Optional, Sequence

FIRST_CYCLE = 20
EVERY_CYCLE = 40
MAX_CYCLE = 220


class Op(IntEnum):
    NOOP = 1
    ADDX = 2


class Instruction:
    code: Op
    arg: Optional[int]

    def __init__(self, code: Op, arg: Optional[int] = None):
        self.code = code
        self.arg = arg

    def __str__(self):
        if self.arg:
            return f"{self.code.name} {self.arg}"
        return self.code.name

    @classmethod
    def from_str(cls, input: str):
        parts = input.split()
        if parts[0] == 'noop':
            return cls(code=Op.NOOP)
        elif parts[0] == 'addx':
            return cls(code=Op.ADDX, arg=int(parts[1]))
        else:
            raise ValueError("Unknown operation:", parts[0])


class CPU:
    cycle: int = 1
    x: int = 1
    addx: Optional[int] = None

    def __init__(self, code: Sequence[Instruction]):
        self.code = code

    def tick(self):
        it = iter(self.code)
        while True:
            yield (self.cycle, self.x)
            self.cycle += 1
            if self.addx is not None:
                # ADDX operation finishes execution in this cycle
                self.x += self.addx
                self.addx = None
                continue
            inst = next(it, Instruction(Op.NOOP))
            if inst.code == Op.ADDX:
                self.addx = inst.arg


def signal(cycle: int, x: int) -> int:
    return cycle * x


def main():
    with open("input.txt", "r") as f:
        ops = [Instruction.from_str(line.strip()) for line in f.readlines()]
    cpu = CPU(ops)
    total = 0
    for cycle, x in cpu.tick():
        if cycle > MAX_CYCLE:
            break
        elif (((cycle - FIRST_CYCLE) % EVERY_CYCLE) == 0):
            total += signal(cycle, x)
    print("Final signal:", total)



if __name__ == '__main__':
    main()
