#!/usr/bin/env python
from typing import Dict, List, Optional, Union

AVAILABLE = 70_000_000
TARGET = 30_000_000


class Dir:
    name: str
    parent: Optional["Dir"]
    size: int
    contents: List[Union["Dir", "File"]]

    def __init__(self, name: str, parent: Optional["Dir"]):
        self.name = name
        self.parent = parent
        self.size = 0
        self.contents = []

    def add(self, content: Union["Dir", "File"]):
        self.contents.append(content)
        self.size += content.size
        p = self.parent
        while p is not None:
            p.size += content.size
            p = p.parent

    def get(self, name: str) -> Union["Dir", "File"]:
        for c in self.contents:
            if c.name == name:
                return c


class File:
    name: str
    size: int

    def __init__(self, name: str, size: int):
        self.name = name
        self.size = size


def find_smallest_directory(initial_dir: Dir, target_size: int):
    """Find smallest directory that is larger than target_size"""
    candidate = initial_dir if initial_dir.size >= target_size else None
    for d in filter(lambda c: isinstance(c, Dir), initial_dir.contents):
        new_candidate = find_smallest_directory(d, target_size)
        is_better_candidate = (
            new_candidate and
            new_candidate.size >= target_size and
            (not candidate or new_candidate.size < candidate.size)
        )
        if is_better_candidate:
            candidate = new_candidate
    return candidate


def main():
    root = Dir("/", None)
    curdir = root
    with open("input.txt", "r") as f:
        for line in f.readlines():
            parts = line.split()
            if parts[0] == '$':
                if parts[1] == 'cd':
                    if parts[2] == '..':
                        curdir = curdir.parent
                    elif parts[2] == '/':
                        curdir = root
                    else:
                        curdir = curdir.get(parts[2])
                elif parts[1] == 'ls':
                    # following lines will be added as content to curdir
                    pass
                else:
                    raise ValueError(f"Unrecognized command: {parts[1]}")
            elif parts[0] == 'dir':
                newdir = Dir(parts[1], curdir)
                curdir.add(newdir)
            else:
                newfile = File(parts[1], int(parts[0]))
                curdir.add(newfile)
    total_used = AVAILABLE - root.size
    target_size = TARGET - total_used
    dir_to_delete = find_smallest_directory(root, target_size)
    print(f"Delete directory {dir_to_delete.name} to free up {dir_to_delete.size}")


if __name__ == '__main__':
    main()
