// solution is the sum of the distance of each crab to the median
// crabs MUST BE SORTED
fn calculate_fuel_part1(crabs: &Vec<i32>) -> i32 {
    let median = crabs[crabs.len() / 2];
    crabs.iter().fold(0, |acc, x| acc + (x - median).abs())
}

// calculated by brute forcing all possible solutions
// crabs MUST BE SORTED
fn calculate_fuel_part2(crabs: &Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    for i in crabs[0]..crabs[crabs.len() - 1] {
        let c = crabs.iter().fold(0, |acc, x| acc + gauss_distance(i, *x));
        if c < min {
            min = c;
        }
    }
    min
}

// calculate distance from a to b, then add all numbers from 1 to the distance
// e.g. a=3, b=7, distance=4, sum 1+2+3+4=10
fn gauss_distance(a: i32, b: i32) -> i32 {
    let d = (a - b).abs();
    // https://betterexplained.com/articles/techniques-for-adding-the-numbers-1-to-100/
    (d * (d + 1)) / 2
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut crabs: Vec<i32> = input
        .lines()
        .next()
        .expect("Invalid input")
        .split(",")
        .map(|x| x.parse::<i32>().expect("Expected an integer number"))
        .collect::<Vec<i32>>();
    crabs.sort();
    println!("Total fuel spent Part 1: {}", calculate_fuel_part1(&crabs));
    println!("Total fuel spent Part 2: {}", calculate_fuel_part2(&crabs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CRABS: [i32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part1() {
        let mut crabs = CRABS.to_vec();
        crabs.sort();
        assert_eq!(calculate_fuel_part1(&crabs), 37);
    }

    #[test]
    fn test_part2() {
        let mut crabs = CRABS.to_vec();
        crabs.sort();
        assert_eq!(calculate_fuel_part2(&crabs), 168);
    }
}
