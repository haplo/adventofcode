#!/usr/bin/env python
from itertools import zip_longest
from typing import List, Optional, Tuple, Union

def in_order(left: Union[int, List[int]], right: Union[int, List[int]]) -> Optional[bool]:
    if isinstance(left, list) and isinstance(right, list):
        for (a, b) in zip_longest(left, right):
            if a is None:
                # a is shorter
                return True
            elif b is None:
                # b is shorter
                return False
            order = in_order(a, b)
            if order is not None:
                return order
        else:
            return None
    elif isinstance(left, int) and isinstance(right, int):
        if left < right:
            return True
        elif left > right:
            return False
        else:
            return None
    elif isinstance(left, list) and isinstance(right, int):
        return in_order(left, [right])
    elif isinstance(left, int) and isinstance(right, list):
        return in_order([left], right)
    else:
        raise Exception("Unreachable code!")


def main():
    pairs: List[Tuple[list, list]] = []
    with open("input.txt", "r") as f:
        it = iter(f.readlines())
        while it:
            pair = (eval(next(it).strip()), eval(next(it).strip()))
            pairs.append(pair)
            try:
                next(it)
            except StopIteration:
                break
    total = 0
    for i, pair in enumerate(pairs, 1):
        if in_order(*pair):
            total += i
    print("Total sum of indices of pairs in the right order:", total)


if __name__ == '__main__':
    main()
