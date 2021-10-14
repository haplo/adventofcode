fn main() {
    let mut total = 0;
    let input = std::fs::read_to_string("input.txt").unwrap();
    for char in input.chars() {
        match char {
            '(' => total += 1,
            ')' => total -= 1,
            _ => (),
        }
    }
    println!("Floor {}", total)
}
