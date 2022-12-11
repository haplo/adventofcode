#!/usr/bin/env python
from dataclasses import dataclass
from typing import Callable, Dict, List, Tuple

ROUNDS = 10000

MonkeyId = int
Worry = int


@dataclass
class Monkey:
    id: MonkeyId
    items: List[Worry]
    op: Callable[[Worry], Worry]
    divisor: Worry
    test_pass: MonkeyId
    test_fail: MonkeyId
    num_inspections: int = 0

    @classmethod
    def from_str(cls, lines: List[str]):
        id = cls.parse_id(lines[0])
        items = cls.parse_items(lines[1])
        op = cls.parse_op(lines[2])
        divisor = cls.parse_divisor(lines[3])
        test_pass = cls.parse_test_result(lines[4], 'true')
        test_fail = cls.parse_test_result(lines[5], 'false')
        return cls(id, items, op, divisor, test_pass, test_fail)

    @staticmethod
    def parse_id(input: str) -> MonkeyId:
        parts = input.split()
        if parts[0] != "Monkey":
            raise ValueError("Invalid input for parse_id:", input)
        return int(parts[1].rstrip(":")[0])

    @staticmethod
    def parse_items(input: str) -> List[Worry]:
        parts = input.split(":")
        if parts[0] != "Starting items":
            raise ValueError("Invalid input for parse_items:", input)
        return [int(i) for i in parts[1].split(",")]

    @staticmethod
    def parse_op(input: str) -> Callable[[Worry], Worry]:
        parts = input.split()
        if parts[0] != "Operation:":
            raise ValueError("Invalid input for parse_op:", input)
        operator = parts[-2]
        if operator not in ('+', '*'):
            raise ValueError("Unsupported operator for Operation:", operator)
        arg = parts[-1]
        def op(worry: Worry) -> Worry:
            if arg == 'old':
                if operator == '+':
                    worry += worry
                elif operator == '*':
                    worry *= worry
            else:
                if operator == '+':
                    worry += int(arg)
                elif operator == '*':
                    worry *= int(arg)
            return worry
        return op

    @staticmethod
    def parse_divisor(input: str) -> Worry:
        parts = input.split()
        if parts[0] != "Test:":
            raise ValueError("Invalid input for parse_test:", input)
        elif parts[1] != "divisible":
            raise ValueError("Only divisible test is supported, got:", parts[1])
        div = int(parts[-1])
        return div

    @staticmethod
    def parse_test_result(input: str, expected: str) -> MonkeyId:
        parts = input.split()
        if (b := parts[1].rstrip(":")) != expected:
            raise ValueError("Expected", expected, "got", b)
        if parts[2:5] != ["throw", "to", "monkey"]:
            raise ValueError("Invalid input for parse_test_result:", input)
        return int(parts[-1])

    def inspect_item(self, item: Worry, gcd: Worry) -> Tuple[Worry, MonkeyId]:
        new_worry = self.op(item) % gcd
        result = new_worry % self.divisor
        new_monkey = self.test_pass if result == 0 else self.test_fail
        self.num_inspections += 1
        return (new_worry, new_monkey)


# GCD across all monkeys' test divisors
# As worry levels grow too large we can take the remainder with the GCD,
# this works because all the monkeys' tests check for divisibility
def calculate_gcd(monkeys: Dict[MonkeyId, Monkey]) -> Worry:
    gcd = 1
    for monkey in monkeys.values():
        if gcd % monkey.divisor != 0:
            gcd *= monkey.divisor
    return gcd


def execute_round(monkeys: Dict[MonkeyId, Monkey], gcd: Worry):
    for monkey in monkeys.values():
        while monkey.items:
            worry = monkey.items.pop(0)
            new_worry, new_monkey = monkey.inspect_item(worry, gcd)
            monkeys[new_monkey].items.append(new_worry)


def main():
    with open("input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]

    monkeys = {}
    last_i = -1
    for i, line in enumerate(lines):
        if line == "":
            monkey = Monkey.from_str(lines[last_i + 1: i])
            monkeys[monkey.id] = monkey
            last_i = i
    else:
        monkey = Monkey.from_str(lines[last_i + 1:])
        monkeys[monkey.id] = monkey

    gcd = calculate_gcd(monkeys)

    for i in range(ROUNDS):
        execute_round(monkeys, gcd)
    num_inspections = [m.num_inspections for m in monkeys.values()]
    num_inspections.sort(reverse=True)
    monkey_business = num_inspections[0] * num_inspections[1]
    print("Monkey business level:", monkey_business)


if __name__ == "__main__":
    main()
