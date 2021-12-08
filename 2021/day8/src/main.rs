const SEGMENTS_1: u8 = 2;
const SEGMENTS_4: u8 = 4;
const SEGMENTS_7: u8 = 3;
const SEGMENTS_8: u8 = 7;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut unique_output_digits = 0;
    for line in input.lines() {
        let mut split = line.split(" | ");
        // ignore first part
        split.next().unwrap();
        unique_output_digits += split
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.len() as u8)
            .filter(|&x| x == SEGMENTS_1 || x == SEGMENTS_4 || x == SEGMENTS_7 || x == SEGMENTS_8)
            .count();
    }
    println!(
        "There are {} digits in output with a unique number of segments",
        unique_output_digits
    );
}
