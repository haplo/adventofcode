type Height = u8;

#[derive(Debug, PartialEq)]
struct Grid {
    columns: usize,
    points: Vec<Height>,
}

impl Grid {
    fn from_str(input: &str) -> Grid {
        let mut lines = input.lines();
        let mut points = Vec::<Height>::new();
        let mut columns = 0;
        while let Some(line) = lines.next() {
            let row: Vec<Height> = line
                .chars()
                .map(|n| n.to_string().parse::<Height>().expect("Expected a number"))
                .collect();
            if columns != 0 && row.len() != columns {
                panic!("Grid must have columns of equal size");
            }
            columns = row.len();
            points.extend(row);
        }
        Grid {
            columns: columns,
            points: points,
        }
    }

    // true if point at index i is a low point
    fn is_low_point(&self, i: usize) -> bool {
        let c = self.columns;
        let p = &self.points;
        let v = p[i];
        let mut is_low = true;
        if i >= c {
            // not first column, look above
            is_low &= v < p[i - c];
        }
        if is_low && i <= p.len() - c {
            // not last column, look below
            is_low &= v < p[i + c];
        }
        if is_low && i > 0 && i % c != 0 {
            // not left border, look left
            is_low &= v < p[i - 1];
        }
        if is_low && (i + 1) % c != 0 {
            // not right border, look right
            is_low &= v < p[i + 1];
        }
        is_low
    }
}

fn risk_level(height: Height) -> u32 {
    (height + 1).into()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = Grid::from_str(&input);
    println!(
        "Total risk level = {}",
        grid.points
            .iter()
            .enumerate()
            .filter(|(i, _)| grid.is_low_point(*i))
            .map(|(_, h)| risk_level(*h))
            .sum::<u32>(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEIGHTMAP: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_parse_grid() {
        assert_eq!(
            Grid::from_str(HEIGHTMAP),
            Grid {
                columns: 10,
                points: vec![
                    2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8,
                    9, 8, 9, 2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8
                ]
            }
        )
    }

    #[test]
    fn test_is_low_point() {
        let grid = Grid::from_str(HEIGHTMAP);
        assert_eq!(grid.is_low_point(0), false);
        assert_eq!(grid.is_low_point(1), true);
        assert_eq!(grid.is_low_point(9), true);
        assert_eq!(grid.is_low_point(10), false);
        assert_eq!(grid.is_low_point(20), false);
        assert_eq!(grid.is_low_point(22), true);
        assert_eq!(grid.is_low_point(39), false);
        assert_eq!(grid.is_low_point(46), true);
        assert_eq!(grid.is_low_point(49), false);
    }
}
