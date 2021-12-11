use std::collections::HashSet;

type Energy = u8;
const COLS: usize = 10;
const MAX_ENERGY: Energy = 9;
const STEPS_PART_1: u32 = 100;

type Grid = Vec<Energy>;

#[derive(Debug, PartialEq)]
struct Simulation {
    grid: Grid,
    step: u32,
    flashed: HashSet<usize>,
}

impl Simulation {
    fn from_str(input: &str) -> Simulation {
        let mut grid = Grid::new();
        for line in input.lines() {
            grid.extend(
                line.chars()
                    .map(|x| x.to_string().parse::<Energy>().unwrap())
                    .collect::<Grid>(),
            );
        }
        Simulation {
            grid: grid,
            step: 0,
            flashed: HashSet::<usize>::new(),
        }
    }

    fn adjacents(&self, i: usize) -> Vec<usize> {
        let p = &self.grid;
        let mut adj: Vec<usize> = vec![];
        let not_first_row = i >= COLS;
        let not_last_row = i < p.len() - COLS;
        let not_left_border = i > 0 && i % COLS != 0;
        let not_right_border = (i + 1) % COLS != 0;
        if not_first_row {
            // not at the first row, use above neighbors including diagonals, unless the
            // point is on the left or right border
            if not_left_border {
                adj.push(i - COLS - 1);
            }
            adj.push(i - COLS);
            if not_right_border {
                adj.push(i - COLS + 1);
            }
        }
        if not_left_border {
            // not at the left border, use left neighbor
            adj.push(i - 1);
        }
        if not_right_border {
            // not at the right border, use right neighbor
            adj.push(i + 1);
        }
        if not_last_row {
            // not at the last row, use below neighbors including diagonals, unless the
            // point is on the left or right border
            if not_left_border {
                adj.push(i + COLS - 1);
            }
            adj.push(i + COLS);
            if not_right_border {
                adj.push(i + COLS + 1);
            }
        }
        adj
    }

    // make all octopus over max energy to flash, increasing the energy of adjacent octopi
    fn flash(&mut self) {
        let flashing: Vec<usize> = self
            .grid
            .iter()
            .enumerate()
            .filter(|(i, x)| **x > MAX_ENERGY && !self.flashed.contains(i))
            .map(|(i, _)| i)
            .collect();
        for i in flashing {
            // octopus flashes
            self.flashed.insert(i);
            self.adjacents(i).iter().for_each(|a| self.grid[*a] += 1);
        }
    }

    fn step(&mut self) {
        self.flashed = HashSet::<usize>::new();
        self.grid.iter_mut().for_each(|x| *x += 1);
        loop {
            let n = self.flashed.len();
            self.flash();
            // stop when no more octopi flash
            if n == self.flashed.len() {
                break;
            }
        }
        // reset octopus that are above max energy to 0
        self.grid
            .iter_mut()
            .filter(|x| **x > MAX_ENERGY)
            .for_each(|x| *x = 0);
        self.step += 1;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut simulation = Simulation::from_str(&input);
    let mut num_flashes: u32 = 0;
    loop {
        simulation.step();
        num_flashes += simulation.flashed.len() as u32;
        if simulation.step == STEPS_PART_1 {
            println!(
                "There has been a total of {} flashes after {} steps",
                num_flashes, STEPS_PART_1
            )
        }
        if simulation.flashed.len() == simulation.grid.len() {
            println!("All octopi flash at step {}", simulation.step);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_from_str() {
        assert_eq!(
            Simulation::from_str(GRID),
            Simulation {
                grid: vec![
                    5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5,
                    6, 1, 7, 3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1,
                    6, 7, 5, 2, 4, 6, 4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1,
                    3, 4, 4, 8, 4, 6, 8, 4, 8, 5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
                ],
                step: 0,
                flashed: HashSet::new(),
            }
        )
    }

    #[test]
    fn test_adj() {
        let sim = Simulation::from_str(GRID);
        assert_eq!(sim.adjacents(0), vec![1, 10, 11]);
        assert_eq!(sim.adjacents(5), vec![4, 6, 14, 15, 16]);
        assert_eq!(sim.adjacents(9), vec![8, 18, 19]);
        assert_eq!(sim.adjacents(10), vec![0, 1, 11, 20, 21]);
        assert_eq!(sim.adjacents(15), vec![4, 5, 6, 14, 16, 24, 25, 26]);
        assert_eq!(sim.adjacents(20), vec![10, 11, 21, 30, 31]);
        assert_eq!(sim.adjacents(25), vec![14, 15, 16, 24, 26, 34, 35, 36]);
        assert_eq!(sim.adjacents(99), vec![88, 89, 98]);
    }

    #[test]
    fn test_step() {
        let mut sim = Simulation::from_str(GRID);
        sim.step();
        assert_eq!(sim.step, 1);
        assert_eq!(
            sim.grid,
            vec![
                6, 5, 9, 4, 2, 5, 4, 3, 3, 4, 3, 8, 5, 6, 9, 6, 5, 8, 2, 2, 6, 3, 7, 5, 6, 6, 7, 2,
                8, 4, 7, 2, 5, 2, 4, 4, 7, 2, 5, 7, 7, 4, 6, 8, 4, 9, 6, 5, 8, 9, 5, 2, 7, 8, 6, 3,
                5, 7, 5, 6, 3, 2, 8, 7, 9, 5, 2, 8, 3, 2, 7, 9, 9, 3, 9, 9, 2, 2, 4, 5, 5, 9, 5, 7,
                9, 5, 9, 6, 6, 5, 6, 3, 9, 4, 8, 6, 2, 6, 3, 7,
            ]
        );
        assert_eq!(sim.flashed, HashSet::from_iter(vec![]));
        sim.step();
        assert_eq!(sim.step, 2);
        assert_eq!(
            sim.grid,
            vec![
                8, 8, 0, 7, 4, 7, 6, 5, 5, 5, 5, 0, 8, 9, 0, 8, 7, 0, 5, 4, 8, 5, 9, 7, 8, 8, 9, 6,
                0, 8, 8, 4, 8, 5, 7, 6, 9, 6, 0, 0, 8, 7, 0, 0, 9, 0, 8, 8, 0, 0, 6, 6, 0, 0, 0, 8,
                8, 9, 8, 9, 6, 8, 0, 0, 0, 0, 5, 9, 4, 3, 0, 0, 0, 0, 0, 0, 7, 4, 5, 6, 9, 0, 0, 0,
                0, 0, 0, 8, 7, 6, 8, 7, 0, 0, 0, 0, 6, 8, 4, 8,
            ]
        );
        assert_eq!(
            sim.flashed,
            HashSet::from_iter(vec![
                2, 11, 14, 17, 28, 38, 39, 42, 43, 45, 48, 49, 52, 53, 54, 62, 63, 64, 65, 70, 71,
                72, 73, 74, 75, 81, 82, 83, 84, 85, 86, 92, 93, 94, 95
            ])
        );
        sim.step();
        assert_eq!(sim.step, 3);
        assert_eq!(
            sim.grid,
            vec![
                0, 0, 5, 0, 9, 0, 0, 8, 6, 6, 8, 5, 0, 0, 8, 0, 0, 5, 7, 5, 9, 9, 0, 0, 0, 0, 0, 0,
                3, 9, 9, 7, 0, 0, 0, 0, 0, 0, 4, 1, 9, 9, 3, 5, 0, 8, 0, 0, 6, 3, 7, 7, 1, 2, 3, 0,
                0, 0, 0, 0, 7, 9, 1, 1, 2, 5, 0, 0, 0, 9, 2, 2, 1, 1, 1, 3, 0, 0, 0, 0, 0, 4, 2, 1,
                1, 2, 5, 0, 0, 0, 0, 0, 2, 1, 1, 1, 9, 0, 0, 0,
            ]
        );
        assert_eq!(
            sim.flashed,
            HashSet::from_iter(vec![
                0, 1, 3, 5, 6, 12, 13, 15, 16, 22, 23, 24, 25, 26, 27, 32, 33, 34, 35, 36, 37, 44,
                46, 47, 55, 56, 57, 58, 59, 66, 67, 68, 76, 77, 78, 79, 80, 87, 88, 89, 90, 91, 97,
                98, 99
            ])
        );
    }
}
