#!/usr/bin/env python

def find_markers(input: str):
    for i in range(4, len(input)):
        if len(set(input[i-4:i])) == 4:
            yield i


def main():
    with open("input.txt", "r") as f:
        input = f.read()
    marker = next(find_markers(input))
    print(f'First marker "{input[marker]}" at position {marker}')


if __name__ == '__main__':
    main()
