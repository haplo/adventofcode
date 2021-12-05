#[derive(Debug, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Vent {
    from: Coordinate,
    to: Coordinate,
}

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;

type Grid = [u8; MAX_X * MAX_Y];

// Parse an input in the form "X,Y" into a Coordinate
fn parse_coordinate(input: &str) -> Coordinate {
    let mut iter = input.trim().splitn(2, ",").map(|n| {
        n.parse::<usize>()
            .expect("Invalid coordinate: expected integer")
    });
    let x = iter.next().expect("Invalid coordinate: missing X");
    let y = iter.next().expect("Invalid coordinate: missing Y");
    Coordinate { x: x, y: y }
}

// Parse an input in the form "X1,Y1 -> X2,Y2" into a Vent
fn parse_instruction(input: &str) -> Vent {
    let mut iter = input.splitn(2, "->");
    let from = parse_coordinate(
        iter.next()
            .expect("Invalid instruction: missing first coordinate"),
    );
    let to = parse_coordinate(
        iter.next()
            .expect("Invalid instruction: missing second coordinate"),
    );
    Vent { from: from, to: to }
}

fn apply_vent(vent: &Vent, grid: &mut Grid) {
    if vent.from.x == vent.to.x {
        let x = vent.from.x;
        let (from_y, to_y) = if vent.from.y < vent.to.y {
            (vent.from.y, vent.to.y)
        } else {
            (vent.to.y, vent.from.y)
        };
        for y in from_y..to_y + 1 {
            let i = y * MAX_X + x;
            let j = y * MAX_X + x + 1;
            grid[i..j].iter_mut().for_each(|v| *v += 1);
        }
    } else if vent.from.y == vent.to.y {
        let y = vent.from.y;
        let (from_x, to_x) = if vent.from.x < vent.to.x {
            (vent.from.x, vent.to.x)
        } else {
            (vent.to.x, vent.from.x)
        };
        let i = y * MAX_X + from_x;
        let j = y * MAX_X + to_x + 1;
        grid[i..j].iter_mut().for_each(|v| *v += 1);
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let vents: Vec<Vent> = input.lines().map(|l| parse_instruction(l)).collect();
    let mut grid: Grid = [0; MAX_X * MAX_Y];
    for vent in vents {
        apply_vent(&vent, &mut grid);
    }
    println!(
        "There are {} points with two or more vents",
        grid.iter().filter(|n| **n >= 2).count()
    );
}
