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
    let median = crabs[crabs.len() / 2];
    let total_fuel = crabs.iter().fold(0, |acc, x| acc + (x - median).abs());
    println!("Total fuel spent: {}", total_fuel)
}
