use std::cmp::min;

#[derive(Debug, PartialEq)]
struct Area {
    x_from: u32,
    x_to: u32,
    y_from: i32,
    y_to: i32,
}

impl Area {
    fn inside(&self, x: u32, y: i32) -> bool {
        x >= self.x_from && x <= self.x_to && y >= self.y_from && y <= self.y_to
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Velocity {
    x: u32,
    y: i32,
}

fn hits_target(target: &Area, mut vel: Velocity) -> bool {
    let mut x = 0;
    let mut y = 0;
    let limit_y = min(target.y_from, target.y_to);
    while y > limit_y {
        x += vel.x;
        y += vel.y;
        if vel.x > 0 {
            vel.x -= 1;
        }
        vel.y -= 1;
        if target.inside(x, y) {
            return true;
        }
    }
    false
}

fn find_optimum_velocity(target: &Area) -> Velocity {
    let mut best = Velocity { x: 1, y: 0 };
    for x in 1..100 {
        for y in 1..500 {
            let vel = Velocity { x: x, y: y };
            if hits_target(&target, vel.clone()) && y > best.y {
                best = vel;
            }
        }
    }
    best
}

fn highest_position(vel: &Velocity) -> i32 {
    let mut y = 0;
    let mut y_vel = vel.y;
    while y_vel > 0 {
        y += y_vel;
        y_vel -= 1;
    }
    y
}

fn parse_target_area(input: &str) -> Area {
    use regex::Regex;
    let re = Regex::new(r"target area: x=(\d+)\.\.(\d+), y=(-?\d+)+\.\.(-?\d+)").unwrap();
    let captures = re.captures(input).expect("Invalid input format");
    Area {
        x_from: captures[1].parse::<u32>().expect("Invalid x"),
        x_to: captures[2].parse::<u32>().expect("Invalid x"),
        y_from: captures[3].parse::<i32>().expect("Invalid y"),
        y_to: captures[4].parse::<i32>().expect("Invalid y"),
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let area = parse_target_area(&input);
    let vel = find_optimum_velocity(&area);
    let highest = highest_position(&vel);
    println!(
        "Velocity x={} y={} reachest highest point {} while hitting target area",
        vel.x, vel.y, highest
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: Area = Area {
        x_from: 20,
        x_to: 30,
        y_from: -10,
        y_to: -5,
    };

    #[test]
    fn test_area_inside() {
        assert_eq!(TARGET.inside(25, -7), true);
        assert_eq!(TARGET.inside(20, -10), true);
        assert_eq!(TARGET.inside(30, -5), true);
        assert_eq!(TARGET.inside(0, 0), false);
        assert_eq!(TARGET.inside(25, -12), false);
        assert_eq!(TARGET.inside(25, -2), false);
        assert_eq!(TARGET.inside(15, -7), false);
        assert_eq!(TARGET.inside(35, -7), false);
    }

    #[test]
    fn test_find_optimum_velocity() {
        assert_eq!(find_optimum_velocity(&TARGET), Velocity { x: 6, y: 9 });
    }

    #[test]
    fn test_hits_target() {
        assert_eq!(hits_target(&TARGET, Velocity { x: 7, y: 2 }), true);
        assert_eq!(hits_target(&TARGET, Velocity { x: 6, y: 3 }), true);
        assert_eq!(hits_target(&TARGET, Velocity { x: 9, y: 0 }), true);
        assert_eq!(hits_target(&TARGET, Velocity { x: 6, y: 9 }), true);
        assert_eq!(hits_target(&TARGET, Velocity { x: 17, y: -4 }), false);
    }

    #[test]
    fn test_parse_target_area() {
        assert_eq!(
            parse_target_area("target area: x=20..30, y=-10..-5"),
            TARGET
        );
    }
}
