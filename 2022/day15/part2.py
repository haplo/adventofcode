#!/usr/bin/env python
import re
from dataclasses import dataclass
from typing import List, Tuple

MIN_X = 0
MAX_X = 4_000_000
MIN_Y = 0
MAX_Y = 4_000_000
X_MULTIPLIER = 4_000_000
POINT_RE = re.compile(r'x=(-?\d+), y=(-?\d+)')

Range = Tuple[int, int]


@dataclass(frozen=True)
class Point:
    x: int
    y: int


@dataclass
class Sensor:
    location: Point
    beacon: Point


def manhattan(p1: Point, p2: Point) -> int:
    return abs(p2.x - p1.x) + abs(p2.y - p1.y)


def merge_overlapping_ranges(ranges: List[Range]) -> List[Range]:
    result = []
    current_start = -1
    current_stop = -1
    for start, stop in sorted(ranges):
        if start > current_stop:
            # this segment starts after the last segment stops
            # just add a new segment
            result.append( (start, stop) )
            current_start, current_stop = start, stop
        else:
            # current_start already guaranteed to be lower
            current_stop = max(current_stop, stop)
            # segments overlap, replace
            result[-1] = (current_start, current_stop)
    return result


def find_beacon(sensors: List[Sensor]) -> Point:
    for y in range(MIN_Y, MAX_Y):
        if (y % 100000 == 0):
            print("Trying row", y)
        ranges = []
        for sensor in sensors:
            radius = manhattan(sensor.location, sensor.beacon)
            distance_to_target = abs(sensor.location.y - y)
            spread = (radius - distance_to_target)
            if spread < 0:
                # sensor didn't reach the target row and can be ignored
                continue
            from_x = max(MIN_X, sensor.location.x - spread)
            to_x = min(MAX_X, sensor.location.x + spread)
            ranges.append((from_x, to_x))
        ranges.sort()
        ranges = merge_overlapping_ranges(ranges)
        # if there is an available position for the beacon it must be either at one of the
        # borders (in which case there would be only one range) or in a position in the
        # middle (two ranges, beacon between them)
        if ranges == [(MIN_X, MAX_X)]:
            # full row is scanned, no room for distress beacon
            continue
        elif (num_ranges := len(ranges)) == 1:
            if ranges[0][0] == 0:
                return Point(MAX_X, y)
            else:
                return Point(MIN_X, y)
        elif num_ranges == 2:
            return Point(ranges[0][1]+1, y)



def main():
    with open("input.txt", "r") as f:
        lines = [line.strip() for line in f.readlines()]
    sensors = []
    for line in lines:
        sensor_match, beacon_match = POINT_RE.findall(line)
        location = Point(int(sensor_match[0]), int(sensor_match[1]))
        beacon = Point(int(beacon_match[0]), int(beacon_match[1]))
        sensor = Sensor(location, beacon)
        sensors.append(sensor)
    beacon = find_beacon(sensors)
    tuning_frequency = beacon.x * X_MULTIPLIER + beacon.y
    print(f"Distress beacon found at {beacon.x},{beacon.y}, "
          f"tuning frequency={tuning_frequency}")


if __name__ == '__main__':
    main()
