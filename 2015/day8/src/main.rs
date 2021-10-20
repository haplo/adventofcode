fn count_code_chars(s: &str) -> usize {
    s.len()
}

fn count_memory_chars(s: &str) -> usize {
    let mut char_iter = s
        .trim_start_matches('"')
        .trim_end_matches('"')
        .chars()
        .peekable();
    let mut count = 0;
    while let Some(c) = char_iter.next() {
        count += match c {
            '\\' => match char_iter.peek() {
                Some('\\') | Some('"') => {
                    char_iter.next();
                    1
                }
                Some('x') => {
                    char_iter.next();
                    char_iter.next();
                    char_iter.next();
                    1
                }
                _ => 1,
            },
            _ => 1,
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut total_code_chars = 0;
    let mut total_memory_chars = 0;
    for line in input.lines() {
        total_code_chars += count_code_chars(line);
        total_memory_chars += count_memory_chars(line);
    }
    println!(
        "Code chars - Memory chars = {}",
        total_code_chars - total_memory_chars
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_code_chars() {
        assert_eq!(count_code_chars(&"\"\""), 2);
        assert_eq!(count_code_chars(&"\"abc\""), 5);
        assert_eq!(count_code_chars(&"\"aaa\\\"aaa\""), 10);
        assert_eq!(count_code_chars(&"\"\\x27\""), 6);
    }

    #[test]
    fn test_count_memory_chars() {
        assert_eq!(count_memory_chars(&"\"\""), 0);
        assert_eq!(count_memory_chars(&"\"abc\""), 3);
        assert_eq!(count_memory_chars(&"\"aaa\\\"aaa\""), 7);
        assert_eq!(count_memory_chars(&"\"\\x27\""), 1);
    }
}
