#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    depth: i32,
    horizontal: i32,
}

fn move_submarine<'a>(plan: impl Iterator<Item = &'a str>) -> Position {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    for line in plan {
        if let [op, amountstr] = line.splitn(2, " ").collect::<Vec<&str>>()[..] {
            let amount = amountstr.parse::<i32>().expect("Units must be integers");
            match op {
                "up" => depth -= amount,
                "down" => depth += amount,
                "forward" => horizontal += amount,
                _ => panic!("Invalid instruction: {}", op),
            }
        } else {
            panic!("Invalid format for line: {}", line)
        }
    }
    Position { depth, horizontal }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let position = move_submarine(input.lines());
    println!(
        "Final depth = {} horizontal = {}. Multiplication = {}",
        position.depth,
        position.horizontal,
        position.depth * position.horizontal
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_submarine() {
        assert_eq!(
            move_submarine(Vec::<&str>::new().into_iter()),
            Position {
                depth: 0,
                horizontal: 0
            }
        );
        assert_eq!(
            move_submarine(
                vec![
                    "down 10",
                    "up 5",
                    "forward 3",
                    "down 4",
                    "forward 6",
                    "up 7",
                ]
                .into_iter()
            ),
            Position {
                depth: 2,
                horizontal: 9
            }
        );
    }
}
