use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Grid {
    columns: usize,
    points: Vec<u32>,
}

impl Grid {
    fn from_str(input: &str) -> Grid {
        let mut lines = input.lines();
        let mut points = Vec::<u32>::new();
        let mut columns = 0;
        while let Some(line) = lines.next() {
            let row: Vec<u32> = line
                .chars()
                .map(|n| n.to_string().parse::<u32>().expect("Expected a number"))
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

    // find shortest path between two points
    fn shortest_path_weight(&self, from: usize, to: usize) -> u32 {
        // calculate minimum distances with Djikstra's Shortest Path algorithm
        // https://brilliant.org/wiki/dijkstras-short-path-finder/
        let mut dist = vec![u32::MAX; self.points.len()];
        dist[from] = 0;
        let mut to_visit: HashSet<usize> = HashSet::from_iter(0..self.points.len());
        while !to_visit.is_empty() {
            let v = to_visit
                .iter()
                .min_by(|&&x, &&y| dist[x].cmp(&dist[y]))
                .unwrap()
                .clone();
            to_visit.remove(&v);
            for adj in self.adjacents(v) {
                let alt = dist[v] + self.points[adj];
                if alt < dist[adj] {
                    dist[adj] = alt
                }
            }
        }
        dist[to]
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = Grid::from_str(&input);
    println!(
        "Total risk of shortest path: {}",
        grid.shortest_path_weight(0, grid.points.len() - 1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEIGHTMAP: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_parse_grid() {
        assert_eq!(
            Grid::from_str(HEIGHTMAP),
            Grid {
                columns: 10,
                points: vec![
                    1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 1, 3, 6, 5, 1,
                    1, 3, 2, 8, 3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 1, 3,
                    1, 9, 1, 2, 8, 1, 3, 7, 1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 3, 1, 2, 5, 4, 2, 1, 6,
                    3, 9, 1, 2, 9, 3, 1, 3, 8, 5, 2, 1, 2, 3, 1, 1, 9, 4, 4, 5, 8, 1,
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
        assert_eq!(grid.adjacents(90), vec![80, 91]);
        assert_eq!(grid.adjacents(99), vec![89, 98]);
    }

    #[test]
    fn test_shortest_path() {
        let grid = Grid::from_str(HEIGHTMAP);
        assert_eq!(grid.shortest_path_weight(0, 99), 40);
    }
}
