fn is_nice(s: &str) -> bool {
    let mut num_vowels = 0;
    let mut double_letter = false;
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        let next_c = iter.peek();
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            num_vowels += 1;
        }
        if let Some(nc) = next_c {
            if c == *nc {
                double_letter = true;
            }
        }
        match (c, next_c) {
            (_, None) => (),
            ('a', Some('b')) => {
                return false;
            }
            ('c', Some('d')) => {
                return false;
            }
            ('p', Some('q')) => {
                return false;
            }
            ('x', Some('y')) => {
                return false;
            }
            (_, _) => (),
        }
    }
    num_vowels >= 3 && double_letter
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let nice_ones = input.lines().filter(|s| is_nice(s)).count();
    println!("There are {} nice strings", nice_ones);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert_eq!(is_nice(&""), false);
        assert_eq!(is_nice(&"aaa"), true);
        assert_eq!(is_nice(&"ugknbfddgicrmopn"), true);
        assert_eq!(is_nice(&"jchzalrnumimnmhp"), false);
        assert_eq!(is_nice(&"haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice(&"dvszwmarrgswjxmb"), false);
    }
}
