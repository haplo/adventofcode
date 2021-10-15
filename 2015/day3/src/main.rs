use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn deliver_presents(plan: &str, num_santas: usize) -> u32 {
    if num_santas == 0 {
        return 0;
    }
    let mut santas = vec![Point { x: 0, y: 0 }; num_santas];
    let mut visited = HashSet::new();
    visited.insert(Point { x: 0, y: 0 });
    for (i, char) in plan.chars().enumerate() {
        let mut pos = &mut santas[i % num_santas];
        match char {
            '<' => pos.x -= 1,
            '>' => pos.x += 1,
            'v' => pos.y -= 1,
            '^' => pos.y += 1,
            _ => (),
        }
        visited.insert(pos.clone());
    }
    visited.len() as u32
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let santa_visits = deliver_presents(&input, 1);
    let santa_and_robot_visits = deliver_presents(&input, 2);
    println!("Santa visits {} houses", santa_visits);
    println!("Santa and Robot visit {} houses", santa_and_robot_visits);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_santa_alone() {
        assert_eq!(deliver_presents("^v", 1), 2);
        assert_eq!(deliver_presents("^>v<", 1), 4);
        assert_eq!(deliver_presents("^v^v^v^v^v", 1), 2);
    }

    #[test]
    fn test_santa_and_robot() {
        assert_eq!(deliver_presents("^v", 2), 3);
        assert_eq!(deliver_presents("^>v<", 2), 3);
        assert_eq!(deliver_presents("^v^v^v^v^v", 2), 11);
    }
}
