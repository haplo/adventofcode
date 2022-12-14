#!/usr/bin/env python
from typing import Dict, List, Optional, Tuple

Point = Tuple[int, int]

FLOOR_DISTANCE = 2

EMPTY = '.'
WALL = '#'
SAND = 'o'


def make_point(input: str) -> Point:
    x, y = input.strip().split(',')
    return int(x), int(y)


class Grid(dict):
    max_y: int = -1

    def __str__(self) -> str:
        # for debugging, just print the grid to see it
        min_x = min(k[0] for k in self.keys())
        max_x = max(k[0] for k in self.keys())
        min_y = min(k[1] for k in self.keys())
        rows = []
        for y in range(0, self.max_y + 2):
            row = []
            for x in range(min_x - 1, max_x + 20):
                row.append(self.get((x, y), EMPTY))
            rows.append(''.join(row))
        rows.append(WALL * (max_x - min_x + 22))
        return '\n'.join(rows)

    @property
    def floor(self) -> int:
        return self.max_y + FLOOR_DISTANCE

    @classmethod
    def from_input(cls, lines: List[str]):
        grid = cls()
        for line in lines:
            points = [make_point(p) for p in line.split('->')]
            prev_point = None
            for point in points:
                if prev_point is not None:
                    grid.trace_line(prev_point, point)
                prev_point = point
        return grid

    def trace_line(self, start: Point, end: Point):
        sx, sy = start
        ex, ey = end
        max_y = max(sy, ey)
        if sx == ex:  # vertical line
            for y in range(min(sy, ey), max_y + 1):
                self[(sx, y)] = WALL
        else:  # horizontal line
            for x in range(min(sx, ex), max(sx, ex) + 1):
                self[(x, sy)] = WALL
        if max_y > self.max_y:
            self.max_y = max_y


def simulate_sand(grid: Grid):
    x, y = 500, 0
    while True:
        if y + 1 == grid.floor:
            break
        elif (x, y + 1) not in grid:
            y += 1
        elif (x - 1, y + 1) not in grid:
            x -= 1
            y += 1
        elif (x + 1, y + 1) not in grid:
            x += 1
            y += 1
        else:
            break
    grid[(x, y)] = SAND


def main():
    with open("input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
    grid = Grid.from_input(lines)
    print("=========================================")
    print("================ START ==================")
    print("=========================================")
    print(str(grid))

    rounds = 0
    prev_len = 0
    while len(grid) != prev_len:
        prev_len = len(grid)
        # simulate sand falling until grid stops changing
        simulate_sand(grid)
        rounds += 1

    print()
    print("=========================================")
    print("================= END ===================")
    print("=========================================")
    print(str(grid))
    print()
    print("Units of sand:", rounds - 1)


if __name__ == '__main__':
    main()
