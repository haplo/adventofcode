// regex crate doesn't support backreferences, which are used in the part 2 rules
use fancy_regex::Regex;
use lazy_static::lazy_static;

fn is_nice_part1(s: &str) -> bool {
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

fn is_nice_part2(s: &str) -> bool {
    lazy_static! {
        // contains a pair of any two letters that appears at least twice in the string without
        // overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
        static ref RULE1_RE: Regex = Regex::new(r"(..).*\1").unwrap();
        // contains at least one letter which repeats with exactly one letter between them, like
        // xyx, abcdefeghi (efe), or even aaa.
        static ref RULE2_RE: Regex = Regex::new(r"(.).\1").unwrap();
    }
    RULE1_RE.is_match(s).unwrap() && RULE2_RE.is_match(s).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let nice_ones_part1 = input.lines().filter(|s| is_nice_part1(s)).count();
    let nice_ones_part2 = input.lines().filter(|s| is_nice_part2(s)).count();
    println!(
        "There are {} nice strings with part 1 rules",
        nice_ones_part1
    );
    println!(
        "There are {} nice strings with part 2 rules",
        nice_ones_part2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_part1() {
        assert_eq!(is_nice_part1(&""), false);
        assert_eq!(is_nice_part1(&"aaa"), true);
        assert_eq!(is_nice_part1(&"ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_part1(&"jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_part1(&"haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_part1(&"dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_is_nice_part2() {
        assert_eq!(is_nice_part2(&""), false);
        assert_eq!(is_nice_part2(&"qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_part2(&"xxyxx"), true);
        assert_eq!(is_nice_part2(&"uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_part2(&"ieodomkazucvgmuy"), false);
    }
}
