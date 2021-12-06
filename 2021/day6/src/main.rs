const RESET: usize = 6;
const SPAWN: usize = RESET + 2;
const DAYS_TO_SIMULATE_PART_1: u32 = 80;
const DAYS_TO_SIMULATE_PART_2: u32 = 256;

struct Simulation {
    fishes_per_day: Vec<u64>,
    day: u32,
}

impl Simulation {
    pub fn from_fishes(fishes: &Vec<usize>) -> Self {
        let mut per_day: Vec<u64> = [0; SPAWN + 1].into_iter().collect();
        for f in fishes {
            per_day[*f] += 1;
        }
        Self {
            day: 0,
            fishes_per_day: per_day,
        }
    }

    pub fn new(day: u32) -> Self {
        Self {
            day: day,
            fishes_per_day: [0; SPAWN + 1].into_iter().collect(),
        }
    }

    pub fn step(&self) -> Simulation {
        let mut next = Simulation::new(self.day + 1);
        for (days_until_spawn, number_of_fishes) in self.fishes_per_day.iter().enumerate() {
            if days_until_spawn == 0 {
                next.fishes_per_day[RESET] = *number_of_fishes;
                next.fishes_per_day[SPAWN] = *number_of_fishes;
            } else {
                next.fishes_per_day[days_until_spawn - 1] += *number_of_fishes;
            }
        }
        next
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let fishes: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().expect("Expected an integer number"))
        .collect();
    let mut simulation = Simulation::from_fishes(&fishes);
    for _ in 0..DAYS_TO_SIMULATE_PART_1 {
        simulation = simulation.step();
    }
    println!(
        "There are {} fishes after {} days",
        simulation.fishes_per_day.iter().sum::<u64>(),
        DAYS_TO_SIMULATE_PART_1
    );
    let remaining_days = DAYS_TO_SIMULATE_PART_2 - DAYS_TO_SIMULATE_PART_1;
    for _ in 0..remaining_days {
        simulation = simulation.step();
    }
    println!(
        "There are {} fishes after {} days",
        simulation.fishes_per_day.iter().sum::<u64>(),
        DAYS_TO_SIMULATE_PART_2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_step() {
        let simulation = Simulation::from_fishes(&vec![3, 4, 3, 1, 2]);
        assert_eq!(simulation.fishes_per_day, vec![0, 1, 1, 2, 1, 0, 0, 0, 0]);
        let simulation = simulation.step();
        assert_eq!(simulation.fishes_per_day, vec![1, 1, 2, 1, 0, 0, 0, 0, 0]);
        let simulation = simulation.step();
        assert_eq!(simulation.fishes_per_day, vec![1, 2, 1, 0, 0, 0, 1, 0, 1]);
        let simulation = simulation.step();
        assert_eq!(simulation.fishes_per_day, vec![2, 1, 0, 0, 0, 1, 1, 1, 1]);
        let simulation = simulation.step();
        assert_eq!(simulation.fishes_per_day, vec![1, 0, 0, 0, 1, 1, 3, 1, 2]);
        let simulation = simulation.step();
        assert_eq!(simulation.fishes_per_day, vec![0, 0, 0, 1, 1, 3, 2, 2, 1]);
    }
}
