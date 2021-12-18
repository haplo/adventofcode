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

fn find_velocities(target: &Area) -> Vec<Velocity> {
    let mut vels = vec![];
    for x in 1..100 {
        for y in -200..500 {
            let vel = Velocity { x: x, y: y };
            if hits_target(&target, vel.clone()) {
                vels.push(vel);
            }
        }
    }
    vels
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
    let velocities = find_velocities(&area);
    let best_vel = velocities.iter().max_by(|v1, v2| v1.y.cmp(&v2.y)).unwrap();
    let highest = highest_position(best_vel);
    println!(
        "Velocity x={} y={} reachest highest point {} while hitting target area",
        best_vel.x, best_vel.y, highest
    );
    println!(
        "There are {} distinct velocities that hit the target area",
        velocities.len()
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
    fn test_find_velocities() {
        assert_eq!(
            find_velocities(&TARGET),
            vec![
                Velocity { x: 6, y: 0 },
                Velocity { x: 6, y: 1 },
                Velocity { x: 6, y: 2 },
                Velocity { x: 6, y: 3 },
                Velocity { x: 6, y: 4 },
                Velocity { x: 6, y: 5 },
                Velocity { x: 6, y: 6 },
                Velocity { x: 6, y: 7 },
                Velocity { x: 6, y: 8 },
                Velocity { x: 6, y: 9 },
                Velocity { x: 7, y: -1 },
                Velocity { x: 7, y: 0 },
                Velocity { x: 7, y: 1 },
                Velocity { x: 7, y: 2 },
                Velocity { x: 7, y: 3 },
                Velocity { x: 7, y: 4 },
                Velocity { x: 7, y: 5 },
                Velocity { x: 7, y: 6 },
                Velocity { x: 7, y: 7 },
                Velocity { x: 7, y: 8 },
                Velocity { x: 7, y: 9 },
                Velocity { x: 8, y: -2 },
                Velocity { x: 8, y: -1 },
                Velocity { x: 8, y: 0 },
                Velocity { x: 8, y: 1 },
                Velocity { x: 9, y: -2 },
                Velocity { x: 9, y: -1 },
                Velocity { x: 9, y: 0 },
                Velocity { x: 10, y: -2 },
                Velocity { x: 10, y: -1 },
                Velocity { x: 11, y: -4 },
                Velocity { x: 11, y: -3 },
                Velocity { x: 11, y: -2 },
                Velocity { x: 11, y: -1 },
                Velocity { x: 12, y: -4 },
                Velocity { x: 12, y: -3 },
                Velocity { x: 12, y: -2 },
                Velocity { x: 13, y: -4 },
                Velocity { x: 13, y: -3 },
                Velocity { x: 13, y: -2 },
                Velocity { x: 14, y: -4 },
                Velocity { x: 14, y: -3 },
                Velocity { x: 14, y: -2 },
                Velocity { x: 15, y: -4 },
                Velocity { x: 15, y: -3 },
                Velocity { x: 15, y: -2 },
                Velocity { x: 20, y: -10 },
                Velocity { x: 20, y: -9 },
                Velocity { x: 20, y: -8 },
                Velocity { x: 20, y: -7 },
                Velocity { x: 20, y: -6 },
                Velocity { x: 20, y: -5 },
                Velocity { x: 21, y: -10 },
                Velocity { x: 21, y: -9 },
                Velocity { x: 21, y: -8 },
                Velocity { x: 21, y: -7 },
                Velocity { x: 21, y: -6 },
                Velocity { x: 21, y: -5 },
                Velocity { x: 22, y: -10 },
                Velocity { x: 22, y: -9 },
                Velocity { x: 22, y: -8 },
                Velocity { x: 22, y: -7 },
                Velocity { x: 22, y: -6 },
                Velocity { x: 22, y: -5 },
                Velocity { x: 23, y: -10 },
                Velocity { x: 23, y: -9 },
                Velocity { x: 23, y: -8 },
                Velocity { x: 23, y: -7 },
                Velocity { x: 23, y: -6 },
                Velocity { x: 23, y: -5 },
                Velocity { x: 24, y: -10 },
                Velocity { x: 24, y: -9 },
                Velocity { x: 24, y: -8 },
                Velocity { x: 24, y: -7 },
                Velocity { x: 24, y: -6 },
                Velocity { x: 24, y: -5 },
                Velocity { x: 25, y: -10 },
                Velocity { x: 25, y: -9 },
                Velocity { x: 25, y: -8 },
                Velocity { x: 25, y: -7 },
                Velocity { x: 25, y: -6 },
                Velocity { x: 25, y: -5 },
                Velocity { x: 26, y: -10 },
                Velocity { x: 26, y: -9 },
                Velocity { x: 26, y: -8 },
                Velocity { x: 26, y: -7 },
                Velocity { x: 26, y: -6 },
                Velocity { x: 26, y: -5 },
                Velocity { x: 27, y: -10 },
                Velocity { x: 27, y: -9 },
                Velocity { x: 27, y: -8 },
                Velocity { x: 27, y: -7 },
                Velocity { x: 27, y: -6 },
                Velocity { x: 27, y: -5 },
                Velocity { x: 28, y: -10 },
                Velocity { x: 28, y: -9 },
                Velocity { x: 28, y: -8 },
                Velocity { x: 28, y: -7 },
                Velocity { x: 28, y: -6 },
                Velocity { x: 28, y: -5 },
                Velocity { x: 29, y: -10 },
                Velocity { x: 29, y: -9 },
                Velocity { x: 29, y: -8 },
                Velocity { x: 29, y: -7 },
                Velocity { x: 29, y: -6 },
                Velocity { x: 29, y: -5 },
                Velocity { x: 30, y: -10 },
                Velocity { x: 30, y: -9 },
                Velocity { x: 30, y: -8 },
                Velocity { x: 30, y: -7 },
                Velocity { x: 30, y: -6 },
                Velocity { x: 30, y: -5 },
            ]
        );
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
