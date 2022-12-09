#!/usr/bin/env python
from collections import Counter
from dataclasses import dataclass
from enum import Enum
from typing import Dict, Tuple


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
    head: Pos = (0, 0)
    tail: Pos = (0, 0)

    def __init__(self, head_x, head_y, tail_x, tail_y):
        self.head = (head_x, head_y)
        self.tail = (tail_x, tail_y)

    def __str__(self):
        return f"H={self.head} T={self.tail}"

    def execute(self, move: Move):
        mx, my = DIRECTIONS[move.direction]
        new_head = (self.head[0] + mx, self.head[1] + my)
        new_tail = self.move_tail(new_head)
        new_rope = self.__class__(new_head[0], new_head[1], new_tail[0], new_tail[1])
        return new_rope

    def move_tail(self, new_head: Pos) -> Pos:
        ox, oy = self.head
        nx, ny = new_head
        tx, ty = self.tail
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

    def __init__(self):
        super()
        self.rope = Rope(0, 0, 0, 0)

    def execute(self, move: Move):
        for i in range(move.amount):
            new_rope = self.rope.execute(move)
            if new_rope.tail != self.rope.tail:
                self[new_rope.tail] += 1
            self.rope = new_rope


def main():
    with open("input.txt", "r") as f:
        moves = [Move.from_str(line.strip()) for line in f.readlines()]
    grid = Grid()
    for move in moves:
        grid.execute(move)
    total = len(grid)
    print(f"Tail visited {total} unique positions")


if __name__ == '__main__':
    main()
