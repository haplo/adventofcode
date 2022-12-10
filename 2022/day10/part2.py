#!/usr/bin/env python

from enum import IntEnum
from typing import Optional, Sequence

FIRST_CYCLE = 20
EVERY_CYCLE = 40
MAX_CYCLE = 220

SCREEN_WIDTH = 40
SCREEN_HEIGHT = 6
SCREEN_MAX = SCREEN_WIDTH * SCREEN_HEIGHT


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


class Screen:

    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.pixels = []

    def is_full(self):
        return len(self.pixels) > (self.width * self.height)

    def paint(self, enabled: bool):
        if self.is_full():
            raise ValueError("Attempt to paint outside the screen")
        self.pixels.append(enabled)

    def clear(self):
        self.pixels = []

    def print(self):
        for row in self.rows():
            print(row)

    def rows(self):
        for r in range(self.height):
            pixels = self.pixels[r * self.width:r * self.width + self.width]
            row = "".join("#" if pixel else "." for pixel in pixels)
            yield row


class CPU:
    cycle: int = 1
    x: int = 1
    addx: Optional[int] = None
    screen: Screen

    def __init__(self, code: Sequence[Instruction]):
        self.code = code
        self.screen = Screen(SCREEN_WIDTH, SCREEN_HEIGHT)

    def tick(self):
        it = iter(self.code)
        while True:
            if self.screen.is_full():
                self.screen.print()
                self.screen.clear()
            column = (self.cycle - 1) % self.screen.width
            pixel = self.x - 1 <= column <= self.x + 1
            self.screen.paint(pixel)
            yield self.cycle
            self.cycle += 1
            if self.addx is not None:
                # ADDX operation finishes execution in this cycle
                self.x += self.addx
                self.addx = None
                continue
            inst = next(it, Instruction(Op.NOOP))
            if inst.code == Op.ADDX:
                self.addx = inst.arg


def main():
    with open("input.txt", "r") as f:
        ops = [Instruction.from_str(line.strip()) for line in f.readlines()]
    cpu = CPU(ops)
    for cycle in cpu.tick():
        if cycle % SCREEN_MAX == 0:
            cpu.screen.print()
        if cycle > SCREEN_MAX:
            break



if __name__ == '__main__':
    main()
