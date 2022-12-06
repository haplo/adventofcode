#!/usr/bin/env python

def find_markers(input: str, length: int):
    for i in range(length, len(input)):
        if len(set(input[i-length:i])) == length:
            yield i


def main():
    with open("input.txt", "r") as f:
        input = f.read()
    marker = next(find_markers(input, 14))
    print(f'First marker "{input[marker]}" at position {marker}')


if __name__ == '__main__':
    main()
