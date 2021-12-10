const CORRUPT_PAREN: u32 = 3;
const CORRUPT_SQUARE: u32 = 57;
const CORRUPT_CURLY: u32 = 1197;
const CORRUPT_ANGLE: u32 = 25137;
const INCOMPLETE_PAREN: u64 = 1;
const INCOMPLETE_SQUARE: u64 = 2;
const INCOMPLETE_CURLY: u64 = 3;
const INCOMPLETE_ANGLE: u64 = 4;

#[derive(Debug, PartialEq)]
enum LineResult {
    Corrupt(u32),
    Incomplete(u64),
    Valid,
}

fn score_line(line: &str) -> LineResult {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' => {
                if stack.pop() != Some(')') {
                    return LineResult::Corrupt(CORRUPT_PAREN);
                }
            }
            ']' => {
                if stack.pop() != Some(']') {
                    return LineResult::Corrupt(CORRUPT_SQUARE);
                }
            }
            '}' => {
                if stack.pop() != Some('}') {
                    return LineResult::Corrupt(CORRUPT_CURLY);
                }
            }
            '>' => {
                if stack.pop() != Some('>') {
                    return LineResult::Corrupt(CORRUPT_ANGLE);
                }
            }
            _ => panic!("Invalid character in input: {}", c),
        }
    }
    if !stack.is_empty() {
        // line is incomplete, whatever remains in stack are the missing characters
        let score = stack.iter().rev().fold(0, |acc, c| {
            (acc * 5)
                + match c {
                    ')' => INCOMPLETE_PAREN,
                    ']' => INCOMPLETE_SQUARE,
                    '}' => INCOMPLETE_CURLY,
                    '>' => INCOMPLETE_ANGLE,
                    // could have an Enum for characters in stack to make the match
                    // exhaustive and avoid this panic!, but I found it overkill
                    _ => panic!("Invalid character in stack, how could this happen?!: {}", c),
                }
        });
        return LineResult::Incomplete(score);
    }
    LineResult::Valid
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let scores: Vec<LineResult> = input.lines().map(|l| score_line(l)).collect();
    println!(
        "Total syntax error score: {}",
        scores
            .iter()
            .map(|s| match s {
                LineResult::Corrupt(score) => *score,
                _ => 0,
            })
            .sum::<u32>(),
    );
    let mut incomplete_scores: Vec<u64> = scores
        .iter()
        .filter(|s| matches!(s, LineResult::Incomplete(_)))
        .map(|s| match s {
            LineResult::Incomplete(score) => *score,
            _ => 0,
        })
        .collect();
    incomplete_scores.sort_unstable();
    println!(
        "Middle completion score: {}",
        incomplete_scores[incomplete_scores.len() / 2]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_line() {
        // valid lines
        assert_eq!(score_line(""), LineResult::Valid);
        assert_eq!(score_line("[]"), LineResult::Valid);
        assert_eq!(score_line("{()()()}"), LineResult::Valid);
        assert_eq!(score_line("[<>({}){}[([])<>]]"), LineResult::Valid);
        // incomplete lines
        assert_eq!(
            score_line("[({(<(())[]>[[{[]{<()<>>"),
            LineResult::Incomplete(288957)
        );
        assert_eq!(
            score_line("[(()[<>])]({[<{<<[]>>("),
            LineResult::Incomplete(5566)
        );
        assert_eq!(
            score_line("(((({<>}<{<{<>}{[]{[]{}"),
            LineResult::Incomplete(1480781)
        );
        assert_eq!(
            score_line("{<[[]]>}<{[{[{[]{()[[[]"),
            LineResult::Incomplete(995444)
        );
        assert_eq!(
            score_line("<{([{{}}[<[[[<>{}]]]>[]]"),
            LineResult::Incomplete(294)
        );
        // corrupt lines
        assert_eq!(score_line("[[<[([]))<([[{}[[()]]]"), LineResult::Corrupt(3));
        assert_eq!(
            score_line("[{[{({}]{}}([{[{{{}}([]"),
            LineResult::Corrupt(57)
        );
        assert_eq!(
            score_line("{([(<{}[<>[]}>{[]{[(<()>"),
            LineResult::Corrupt(1197)
        );
        assert_eq!(
            score_line("<{([([[(<>()){}]>(<<{{"),
            LineResult::Corrupt(25137)
        );
    }
}
