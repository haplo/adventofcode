fn main() {
    let mut total = 0;
    let mut basement = 0;
    let input = std::fs::read_to_string("input.txt").unwrap();
    for (i, char) in input.chars().enumerate() {
        match char {
            '(' => total += 1,
            ')' => total -= 1,
            _ => (),
        }
        if total <= -1 && basement == 0 {
            basement = i + 1;
        }
    }
    println!("Floor {}", total);
    println!("Basement first entered at position {}", basement);
}
