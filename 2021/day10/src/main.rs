const SCORE_PAREN: u32 = 3;
const SCORE_SQUARE: u32 = 57;
const SCORE_CURLY: u32 = 1197;
const SCORE_ANGLE: u32 = 25137;

fn score_line(line: &str) -> u32 {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' => {
                if stack.pop() != Some(')') {
                    return SCORE_PAREN;
                }
            }
            ']' => {
                if stack.pop() != Some(']') {
                    return SCORE_SQUARE;
                }
            }
            '}' => {
                if stack.pop() != Some('}') {
                    return SCORE_CURLY;
                }
            }
            '>' => {
                if stack.pop() != Some('>') {
                    return SCORE_ANGLE;
                }
            }
            _ => panic!("Invalid character in input: {}", c),
        }
    }
    0
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "Total syntax error score: {}",
        input.lines().map(|l| score_line(l)).sum::<u32>(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_line() {
        // valid lines
        assert_eq!(score_line(""), 0);
        assert_eq!(score_line("[]"), 0);
        assert_eq!(score_line("{()()()}"), 0);
        assert_eq!(score_line("[<>({}){}[([])<>]]"), 0);
        // incomplete lines
        assert_eq!(score_line("[({(<(())[]>[[{[]{<()<>>"), 0);
        assert_eq!(score_line(""), 0);
        // corrupt lines
        assert_eq!(score_line("[[<[([]))<([[{}[[()]]]"), 3);
        assert_eq!(score_line("[{[{({}]{}}([{[{{{}}([]"), 57);
        assert_eq!(score_line("{([(<{}[<>[]}>{[]{[(<()>"), 1197);
        assert_eq!(score_line("<{([([[(<>()){}]>(<<{{"), 25137);
    }
}
