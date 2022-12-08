#!/usr/bin/env python
from itertools import repeat
from typing import List


class Memo:
    """Save the tallest tree on each direction for each position of the grid

    This is a dynamic programming optimization that calculates the tallest tree visible on
    each direction for all x,y positions in the grid. After this precalculation is done
    it's possible to determine whether a tree x,y is visible by checking the precalculated
    values in the immediate four directions, i.e. [x-1,y], [x+1,y], [x, y+1] and [x, y-1].
    This avoids loops and finds the solution in O(n*m) time for a grid with n rows and m
    columns, albeit with an extra 4x use of memory.

    """
    def __init__(self, grid):
        self.grid = grid
        self._memo_left = self._calculate_memo_left(grid)
        self._memo_right = self._calculate_memo_right(grid)
        self._memo_up = self._calculate_memo_up(grid)
        self._memo_down = self._calculate_memo_down(grid)

    @staticmethod
    def _calculate_memo_left(grid: List[List[int]]) -> List[List[int]]:
        memo = [list(repeat(-1, len(row))) for row in grid]
        for memo_row, grid_row in zip(memo, grid):
            for c in range(1, len(memo_row)):
                memo_row[c] = max(memo_row[c - 1], grid_row[c - 1])
        return memo

    @staticmethod
    def _calculate_memo_right(grid: List[List[int]]) -> List[List[int]]:
        memo = [list(repeat(-1, len(row))) for row in grid]
        for memo_row, grid_row in zip(memo, grid):
            for c in range(len(memo_row) - 2, -1, -1):
                memo_row[c] = max(memo_row[c + 1], grid_row[c + 1])
        return memo

    @staticmethod
    def _calculate_memo_up(grid: List[List[int]]) -> List[List[int]]:
        memo = [list(repeat(-1, len(row))) for row in grid]
        prev_row = memo[0]
        for r in range(1, len(memo)):
            memo_row = memo[r]
            grid_row = grid[r - 1]
            for c in range(0, len(memo_row)):
                memo_row[c] = max(prev_row[c], grid_row[c])
            prev_row = memo_row
        return memo

    @staticmethod
    def _calculate_memo_down(grid: List[List[int]]) -> List[List[int]]:
        memo = [list(repeat(-1, len(row))) for row in grid]
        prev_row = memo[-1]
        for r in range(len(prev_row) - 2, -1, -1):
            memo_row = memo[r]
            grid_row = grid[r + 1]
            for c in range(0, len(memo_row)):
                memo_row[c] = max(prev_row[c], grid_row[c])
            prev_row = memo_row
        return memo

    def is_visible(self, row: int, col: int) -> bool:
        current = self.grid[row][col]
        return (
            current > self._memo_left[row][col] or
            current > self._memo_right[row][col] or
            current > self._memo_up[row][col] or
            current > self._memo_down[row][col]
        )


def main():
    with open("input.txt", "r") as f:
        grid = [[int(c) for c in line.strip()] for line in f.readlines()]
    memo = Memo(grid)
    visible = 0
    for row in range(len(grid)):
        for col in range(len(grid[row])):
            if memo.is_visible(row, col):
                visible += 1
    print("Visible trees from outside the grid:", visible)


if __name__ == "__main__":
    main()
