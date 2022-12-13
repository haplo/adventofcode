#!/usr/bin/env python
from dataclasses import dataclass
from typing import Dict, Generator, List, Optional

BASE_LEVEL = ord("a")
MAX_LEVEL = ord("z")

@dataclass(frozen=True)
class Position:
    row: int
    col: int


class Grid:
    rows: List[List[int]]
    rowsize: int
    end: Position

    def __init__(self, rows, end):
        self.rows = rows
        self.rowsize = len(rows[0])
        self.end = end

    @classmethod
    def from_lines(cls, lines: List[str]):
        rows = []
        for r, line in enumerate(lines):
            row = []
            for c, char in enumerate(line.strip()):
                if char == 'E':
                    end = Position(r, c)
                    row.append(MAX_LEVEL - BASE_LEVEL)
                elif char == 'S':
                    row.append(ord('a') - BASE_LEVEL)
                else:
                    row.append(ord(char) - BASE_LEVEL)
            rows.append(row)
        return cls(rows, end)

    def all_starts(self) -> Generator[Position, None, None]:
        for r, row in enumerate(self.rows):
            for c, value in enumerate(row):
                if value == 0:
                    yield Position(r, c)

    def try_all_starts(self) -> Optional[int]:
        best = None
        for start in self.all_starts():
            if path := self.bfs(start):
                path_len = len(path) - 1
                if best is None or path_len < best:
                    best = path_len
        return best

    def bfs(self, start: Position) -> List[Position]:
        path = []
        to_visit = [start]
        visited = {start}
        parents: Dict[Position, Position] = {}
        while to_visit:
            pos = to_visit.pop(0)
            if pos == self.end:
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
    best = grid.try_all_starts()
    print(f"Shortest path from a lowest point to end in {best} steps")


if __name__ == '__main__':
    main()
