#!/usr/bin/env python
from functools import cmp_to_key
from itertools import zip_longest
from typing import List, Optional, Tuple, Union

DIVISOR_1 = [[2]]
DIVISOR_2 = [[6]]


def cmp_packets(left: Union[int, List[int]], right: Union[int, List[int]]) -> int:
    if isinstance(left, list) and isinstance(right, list):
        for (a, b) in zip_longest(left, right):
            if a is None:
                # a is shorter
                return -1
            elif b is None:
                # b is shorter
                return 1
            order = cmp_packets(a, b)
            if order == 0:
                continue
            return order
        else:
            return 0
    elif isinstance(left, int) and isinstance(right, int):
        if left < right:
            return -1
        elif left > right:
            return 1
        else:
            return 0
    elif isinstance(left, list) and isinstance(right, int):
        return cmp_packets(left, [right])
    elif isinstance(left, int) and isinstance(right, list):
        return cmp_packets([left], right)
    else:
        raise Exception("Unreachable code!")


def main():
    packets: List[list] = []
    with open("input.txt", "r") as f:
        for line in f.readlines():
            if line.strip():
                packets.append(eval(line))
    packets.append(DIVISOR_1)
    packets.append(DIVISOR_2)
    packets.sort(key=cmp_to_key(cmp_packets))
    index_divisor_1 = packets.index(DIVISOR_1) + 1
    index_divisor_2 = packets.index(DIVISOR_2) + 1
    total = index_divisor_1 * index_divisor_2
    print("Decoder key:", total)


if __name__ == '__main__':
    main()
