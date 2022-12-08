#!/usr/bin/env python
from typing import List


def calculate_score(grid: List[List[int]], row: int, col: int) -> int:
    current = grid[row][col]
    total_score = 1
    partial_score = 0
    for r in range(row + 1, len(grid)):
        partial_score += 1
        if grid[r][col] >= current:
            break
    total_score *= partial_score
    partial_score = 0
    for r in range(row - 1, -1, -1):
        partial_score += 1
        if grid[r][col] >= current:
            break
    total_score *= partial_score
    partial_score = 0
    for c in range(col + 1, len(grid[row])):
        partial_score += 1
        if grid[row][c] >= current:
            break
    total_score *= partial_score
    partial_score = 0
    for c in range(col - 1, -1, -1):
        partial_score += 1
        if grid[row][c] >= current:
            break
    total_score *= partial_score
    return total_score


def main():
    with open("input.txt", "r") as f:
        grid = [[int(c) for c in line.strip()] for line in f.readlines()]
    best = -1
    for row in range(1, len(grid) - 1):
        for col in range(1, len(grid[row]) - 1):
            if (score := calculate_score(grid, row, col)) > best:
                best = score
    print("Best scenic score:", best)


if __name__ == "__main__":
    main()
