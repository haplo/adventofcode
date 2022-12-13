#!/usr/bin/env python
from dataclasses import dataclass
from typing import Dict, List, Optional, Tuple

BASE_LEVEL = ord("a")
MAX_LEVEL = ord("z")

@dataclass(frozen=True)
class Position:
    row: int
    col: int


class Grid:
    rows: List[List[int]]
    rowsize: int
    start: Position
    end: Position

    def __init__(self, rows, start, end):
        self.rows = rows
        self.rowsize = len(rows[0])
        self.start = start
        self.end = end

    @classmethod
    def from_lines(cls, lines: List[str]):
        rows = []
        for r, line in enumerate(lines):
            row = []
            for c, char in enumerate(line.strip()):
                if char == 'S':
                    start = Position(r, c)
                    row.append(BASE_LEVEL - BASE_LEVEL)
                elif char == 'E':
                    end = Position(r, c)
                    row.append(MAX_LEVEL - BASE_LEVEL)
                else:
                    row.append(ord(char) - BASE_LEVEL)
            rows.append(row)
        return cls(rows, start, end)

    def bfs(self) -> List[Position]:
        to_visit = [self.start]
        visited = {self.start}
        parents: Dict[Position, Position] = {}
        while to_visit:
            pos = to_visit.pop(0)
            if pos == self.end:
                path = []
                p: Optional[Position] = pos
                while p:
                    path.append(p)
                    p = parents.get(p)
                return path[::-1]
            for candidate in self.valid_neighbors(pos):
                if candidate not in visited:
                    visited.add(candidate)
                    parents[candidate] = pos
                    to_visit.append(candidate)
        return path

    def valid_neighbors(self, pos: Position):
        neighbors: List[Position] = []
        row, col = pos.row, pos.col
        value = self.rows[row][col]
        if col > 1:
            neighbors.append(Position(row, col - 1))
        if col < self.rowsize - 1:
            neighbors.append(Position(row, col + 1))
        if row > 1:
            neighbors.append(Position(row - 1, col))
        if row < len(self.rows) - 1:
            neighbors.append(Position(row + 1, col))
        # only return neighbors that are at most one  higher than the current one
        return [n for n in neighbors if self.rows[n.row][n.col] <= value + 1]


def main():
    with open("input.txt", "r") as f:
        grid = Grid.from_lines(f.readlines())
    path = grid.bfs()
    print(f"Shortest path from start to end in {len(path)-1} steps")


if __name__ == '__main__':
    main()
