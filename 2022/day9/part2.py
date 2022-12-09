#!/usr/bin/env python
from collections import Counter
from dataclasses import dataclass
from enum import Enum
from itertools import repeat
from typing import Dict, List, Tuple


Pos = Tuple[int, int]


class Direction(Enum):
    Up = 'U'
    Down = 'D'
    Right = 'R'
    Left = 'L'

    @classmethod
    def from_str(cls, input: str):
        if input in set(d.value for d in Direction):
            return cls(input)
        raise ValueError(f"Invalid value for Direction: {input}")


DIRECTIONS: Dict[Direction, Pos] = {
    Direction.Up: (0, 1),
    Direction.Down: (0, -1),
    Direction.Right: (1, 0),
    Direction.Left: (-1, 0),
}


@dataclass
class Move:
    direction: Direction
    amount: int

    @classmethod
    def from_str(cls, input: str):
        parts = input.split()
        return cls(Direction.from_str(parts[0]), int(parts[1]))

    def __str__(self):
        return f"{self.amount} {self.direction.name}"


class Rope:
    segments: List[Pos]

    def __init__(self, x: int, y: int, num_knots: int):
        self.segments = list(repeat((x, y), num_knots))

    @property
    def head(self) -> Pos:
        return self.segments[0]

    @property
    def tail(self) -> Pos:
        return self.segments[-1]

    def execute(self, move: Move):
        mx, my = DIRECTIONS[move.direction]
        new_segments = [(self.head[0] + mx, self.head[1] + my)]
        for segment in self.segments[1:]:
            new_segments.append(self.follow(segment, new_segments[-1]))
        self.segments = new_segments

    @staticmethod
    def follow(tail: Pos, next_segment: Pos) -> Pos:
        nx, ny = next_segment
        tx, ty = tail
        nx_tx = nx - tx
        ny_ty = ny - ty

        if nx_tx == 2:
            dx = 1
        elif nx_tx == -2:
            dx = -1
        else:
            dx = nx_tx if abs(ny_ty) == 2 else 0

        if ny_ty == 2:
            dy = 1
        elif ny_ty == -2:
            dy = -1
        else:
            dy = ny_ty if abs(nx_tx) == 2 else 0

        return (tx + dx, ty + dy)



class Grid(Counter):
    rope: Rope

    def __init__(self, num_knots):
        super()
        self.rope = Rope(0, 0, num_knots)
        # In part 1 the start position was not included, but in part 2 it is!
        self[self.rope.tail] += 1

    def execute(self, move: Move):
        for i in range(move.amount):
            last_pos = self.rope.tail
            self.rope.execute(move)
            if self.rope.tail != last_pos:
                self[self.rope.tail] += 1


def main():
    with open("input.txt", "r") as f:
        moves = [Move.from_str(line.strip()) for line in f.readlines()]
    grid = Grid(10)
    for move in moves:
        grid.execute(move)
    total = len(grid)
    print(f"Tail visited {total} unique positions")


if __name__ == '__main__':
    main()
