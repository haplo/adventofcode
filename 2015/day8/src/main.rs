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

fn count_escaped_chars(s: &str) -> usize {
    let mut escaped = String::with_capacity(s.len() * 2);
    escaped.push('"');
    let mut char_iter = s.chars();
    while let Some(c) = char_iter.next() {
        match c {
            '\\' => {
                escaped.push('\\');
                escaped.push('\\');
            }
            '\"' => {
                escaped.push('\\');
                escaped.push('\"');
            }
            c => escaped.push(c),
        }
    }
    escaped.push('"');
    escaped.len()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut total_code_chars = 0;
    let mut total_memory_chars = 0;
    let mut total_escaped_chars = 0;
    for line in input.lines() {
        total_code_chars += count_code_chars(line);
        total_memory_chars += count_memory_chars(line);
        total_escaped_chars += count_escaped_chars(line);
    }
    println!(
        "Part 1: Code chars - Memory chars = {}",
        total_code_chars - total_memory_chars
    );
    println!(
        "Part 2: Escaped chars - Code chars = {}",
        total_escaped_chars - total_code_chars
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
    fn test_count_escaped_chars() {
        assert_eq!(count_escaped_chars(&"\"\""), 6);
        assert_eq!(count_escaped_chars(&"\"abc\""), 9);
        assert_eq!(count_escaped_chars(&"\"aaa\\\"aaa\""), 16);
        assert_eq!(count_escaped_chars(&"\"\\x27\""), 11);
    }

    #[test]
    fn test_count_memory_chars() {
        assert_eq!(count_memory_chars(&"\"\""), 0);
        assert_eq!(count_memory_chars(&"\"abc\""), 3);
        assert_eq!(count_memory_chars(&"\"aaa\\\"aaa\""), 7);
        assert_eq!(count_memory_chars(&"\"\\x27\""), 1);
    }
}
