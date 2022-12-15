#!/usr/bin/env python
import re
from dataclasses import dataclass
from typing import Tuple

TARGET_ROW = 2_000_000
POINT_RE = re.compile(r'x=(-?\d+), y=(-?\d+)')

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
    empty_points = set()
    for sensor in sensors:
        radius = manhattan(sensor.location, sensor.beacon)
        distance_to_target = abs(sensor.location.y - TARGET_ROW)
        spread = (radius - distance_to_target)
        if spread < 0:
            # sensor didn't reach the target row and can be ignored
            continue
        empty_points.update(range(sensor.location.x - spread, sensor.location.x + spread))
    print("Positions that cannot have a beacon:", len(empty_points))


if __name__ == '__main__':
    main()
