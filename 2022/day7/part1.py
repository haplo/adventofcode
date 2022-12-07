#!/usr/bin/env python
from typing import Dict, List, Optional, Union

MAX_SIZE = 100_000


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


def find_directories(initial_dir: Dir, max_size: int):
    if initial_dir.size <= max_size:
        yield initial_dir
    for d in initial_dir.contents:
        if isinstance(d, Dir):
            yield from find_directories(d, max_size)


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
    total = sum(d.size for d in find_directories(root, MAX_SIZE))
    print(f"Total size of directories under {MAX_SIZE}: {total}")


if __name__ == '__main__':
    main()
