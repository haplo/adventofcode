use std::collections::HashSet;

type Height = u8;
const BASIN_LIMIT: Height = 9;

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

    fn adjacents(&self, i: usize) -> Vec<usize> {
        let c = self.columns;
        let p = &self.points;
        let mut adj: Vec<usize> = vec![];
        if i >= c {
            // not at the first column, use above neighbor
            adj.push(i - c);
        }
        if i > 0 && i % c != 0 {
            // not at the left border, use left neighbor
            adj.push(i - 1);
        }
        if (i + 1) % c != 0 {
            // not at the right border, use right neighbor
            adj.push(i + 1);
        }
        if i < p.len() - c {
            // not at the last column, use below neighbor
            adj.push(i + c);
        }
        adj
    }

    // return basin size around point at index i
    fn basin(&self, i: usize) -> u32 {
        let mut visited = HashSet::<usize>::new();
        let mut to_visit = Vec::<usize>::new();
        to_visit.push(i);
        while !to_visit.is_empty() {
            let i = to_visit.pop().unwrap();
            visited.insert(i);
            let unvisited_adj: Vec<usize> = self
                .adjacents(i)
                .into_iter()
                .filter(|x| self.points[*x] != BASIN_LIMIT && !visited.contains(x))
                .collect();
            to_visit.extend(unvisited_adj);
        }
        visited.len() as u32
    }

    // true if point at index i is a low point
    fn is_low_point(&self, i: usize) -> bool {
        let v = self.points[i];
        self.adjacents(i)
            .iter()
            .map(|a| self.points[*a])
            .all(|n| v < n)
    }
}

fn risk_level(height: Height) -> u32 {
    (height + 1).into()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = Grid::from_str(&input);
    // grid position of the low points
    let low_points: Vec<usize> = grid
        .points
        .iter()
        .enumerate()
        .filter(|(i, _)| grid.is_low_point(*i))
        .map(|(i, _)| i)
        .collect();
    println!(
        "Total risk level = {}",
        low_points
            .iter()
            .map(|x| risk_level(grid.points[*x]))
            .sum::<u32>(),
    );
    // sizes of all basins, calculations rely on the fact that each point belongs to only
    // one basin
    let mut basins = low_points
        .iter()
        .map(|p| grid.basin(*p))
        .collect::<Vec<u32>>();
    // sort will put smallest basins first, that's why rev() is used later
    basins.sort_unstable();
    println!(
        "Multiplication of 3 largest basins: = {}",
        basins.iter().rev().take(3).fold(1, |acc, b| acc * b)
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
    fn test_adj() {
        let grid = Grid::from_str(HEIGHTMAP);
        assert_eq!(grid.adjacents(0), vec![1, 10]);
        assert_eq!(grid.adjacents(5), vec![4, 6, 15]);
        assert_eq!(grid.adjacents(9), vec![8, 19]);
        assert_eq!(grid.adjacents(10), vec![0, 11, 20]);
        assert_eq!(grid.adjacents(15), vec![5, 14, 16, 25]);
        assert_eq!(grid.adjacents(20), vec![10, 21, 30]);
        assert_eq!(grid.adjacents(25), vec![15, 24, 26, 35]);
        assert_eq!(grid.adjacents(49), vec![39, 48]);
    }

    #[test]
    fn test_basin() {
        let grid = Grid::from_str(HEIGHTMAP);
        assert_eq!(grid.basin(1), 3);
        assert_eq!(grid.basin(9), 9);
        assert_eq!(grid.basin(22), 14);
        assert_eq!(grid.basin(46), 9);
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
