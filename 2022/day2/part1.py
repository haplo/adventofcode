#!/usr/bin/env python
from enum import Enum
from typing import Tuple


class Shape(Enum):
    Rock = 'R'
    Paper = 'P'
    Scissors = 'S'


SCORE_LOSE = 0
SCORE_DRAW = 3
SCORE_WIN = 6
SCORE_SHAPE = {
    Shape.Rock: 1,
    Shape.Paper: 2,
    Shape.Scissors: 3,
}


def input_to_shape(char: str):
    if char == 'A' or char == 'X':
        return Shape.Rock
    if char == 'B' or char == 'Y':
        return Shape.Paper
    if char == 'C' or char == 'Z':
        return Shape.Scissors
    raise ValueError(f"Invalid input: {char}")


def score(play: Tuple[Shape, Shape]):
    shape_score = SCORE_SHAPE[play[1]]
    if play[0] == play[1]:
        outcome_score = SCORE_DRAW
    elif play[0] == Shape.Rock and play[1] == Shape.Paper:
        outcome_score = SCORE_WIN
    elif play[0] == Shape.Paper and play[1] == Shape.Scissors:
        outcome_score = SCORE_WIN
    elif play[0] == Shape.Scissors and play[1] == Shape.Rock:
        outcome_score = SCORE_WIN
    elif play[0] == Shape.Rock and play[1] == Shape.Scissors:
        outcome_score = SCORE_LOSE
    elif play[0] == Shape.Paper and play[1] == Shape.Rock:
        outcome_score = SCORE_LOSE
    elif play[0] == Shape.Scissors and play[1] == Shape.Paper:
        outcome_score = SCORE_LOSE
    else:
        raise Exception("Unhandled outcome")
    return shape_score + outcome_score


def main():
    with open("input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
    pairs = (line.split(maxsplit=1) for line in lines)
    plays = ((input_to_shape(pair[0]), input_to_shape(pair[1])) for pair in pairs)
    total = sum(score(play) for play in plays)
    print("Total score:", total)


if __name__ == '__main__':
    main()
