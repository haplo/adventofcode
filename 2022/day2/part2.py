#!/usr/bin/env python
from enum import Enum
from typing import Tuple


class Shape(Enum):
    Rock = 'R'
    Paper = 'P'
    Scissors = 'S'


class Outcome(Enum):
    Lose = 'X'
    Draw = 'Y'
    Win = 'Z'


SCORE_OUTCOME = {
    Outcome.Lose: 0,
    Outcome.Draw: 3,
    Outcome.Win: 6,
}
SCORE_SHAPE = {
    Shape.Rock: 1,
    Shape.Paper: 2,
    Shape.Scissors: 3,
}


def input_to_shape(char: str):
    if char == 'A':
        return Shape.Rock
    if char == 'B':
        return Shape.Paper
    if char == 'C':
        return Shape.Scissors
    raise ValueError(f"Invalid input: {char}")


def input_to_score(char: str):
    if char == 'X':
        return Outcome.Lose
    if char == 'Y':
        return Outcome.Draw
    if char == 'Z':
        return Outcome.Win
    raise ValueError(f"Invalid input: {char}")


def score(play: Tuple[Shape, Outcome]):
    opponent, outcome = play
    outcome_score = SCORE_OUTCOME[outcome]
    if outcome == Outcome.Draw:
        shape = opponent
    elif opponent == Shape.Rock:
        shape = Shape.Paper if outcome == Outcome.Win else Shape.Scissors
    elif opponent == Shape.Paper:
        shape = Shape.Scissors if outcome == Outcome.Win else Shape.Rock
    elif opponent == Shape.Scissors:
        shape = Shape.Rock if outcome == Outcome.Win else Shape.Paper
    else:
        raise Exception(f"Unhandled case: {play}")
    shape_score = SCORE_SHAPE[shape]
    return shape_score + outcome_score


def main():
    with open("input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
    pairs = (line.split(maxsplit=1) for line in lines)
    plays = ((input_to_shape(pair[0]), input_to_score(pair[1])) for pair in pairs)
    total = sum(score(play) for play in plays)
    print("Total score:", total)


if __name__ == '__main__':
    main()
